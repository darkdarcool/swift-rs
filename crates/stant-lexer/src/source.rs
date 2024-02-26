// Concept of [Source] is taken from oxc by Boshen <https://github.com/oxc-project/oxc/blob/main/crates/oxc_parser/src/lexer/source.rs>

/// `Source` contains the source code of the program, and is consumed the by lexer
/// The reason it's only stored in the lexer is because the lexer is the only consumer
///
/// `Source` also provides a pointer to the current position in the source, which is used by the lexer for fast reads
///
/// # Under the hood (of the hood?)
/// * `start` is a pointer to the start of the source
/// * `end` is a pointer to the end of the source
/// * `ptr` is a pointer to the current position in the source
pub struct Source {
    /// Pointer to the start of the source
    pub(crate) start: *const u8,
    /// Pointer to the end of the source
    pub(crate) end: *const u8,
    /// Pointer to the current position in the source
    pub(crate) ptr: *const u8,
}

impl Source {
    /// Creates a new `Source` instance.
    ///
    /// # Parameters
    ///
    /// * `source`: A string slice that represents the source code.
    ///
    /// # Returns
    ///
    /// A [`Source`] instance.
    ///
    /// # Safety
    ///
    /// This function is safe to call as it does not perform any unsafe operations.
    /// However, the returned [`Source`] instance contains raw pointers that should be handled with care.
    /// Misuse of these pointers can lead to undefined behavior.
    pub fn new<'a>(source: &'a str) -> Self {
        // create a pointer to the initial start of the source
        let start = source.as_ptr();

        // create a pointer to the end of the source
        let end = unsafe { start.add(source.len()) };

        // Return the `Source` instance
        Source {
            start,
            end,
            ptr: start,
        }
    }



    /// Returns the entire source code as a string.
    ///
    /// # Safety
    ///
    /// This function is unsafe due to unchecked operations. Ensure the following:
    /// * `Source` instance was created with a valid string.
    /// * Pointers `start`, `end`, and `ptr` are untouched.
    /// * Original string that `Source` was created with is intact.
    ///
    /// # Returns
    ///
    /// A string slice representing the entire source code.
    pub unsafe fn get_whole_source<'a>(&self) -> &'a str {
        // Calculate the length of the source
        let len = self.end as usize - self.start as usize;
        // Create a slice from the raw parts
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.start, len))
    }

    pub fn get_current_pos(&self) -> usize {
        self.ptr as usize - self.start as usize
    }

    pub(super) fn advance(&mut self, offset: usize) {
        // let value = unsafe { *self.start.add(index) };
        // self.ptr = unsafe { self.start.add(index) };
        self.ptr = unsafe { self.ptr.add(offset) };
    }

    pub(super) fn advance_ptr(&mut self) {
        self.ptr = unsafe { self.ptr.add(1) };
    }

    pub(super) fn current(&self) -> u8 {
        unsafe { *self.ptr.as_ref().unwrap() }
    }

    pub(super) fn current_pos(&self) -> usize {
        self.ptr as usize - self.start as usize
    }

    pub(super) fn is_at_end(&self) -> bool {
        self.ptr >= self.end
    }

    pub(super) fn peek(&self) -> Option<u8> {
        if self.ptr < self.end {
            let value = unsafe { *self.ptr.offset(1).as_ref().unwrap() };

            Some(value)
        } else {
            None
        }
    }

    pub(super) fn get_slice<'a>(&self, start: usize, end: usize) -> &'a str {
        let len = end - start;
        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.start.add(start), len)) }
    }

    /// Get offset of the current position
    #[allow(dead_code)]
    pub(super) fn offset(&self) -> u32 {
        self.ptr as u32 - self.start as u32
    }


    pub(super) fn next_char(&mut self) -> Option<char> {
        if self.ptr < self.end {
            let value = unsafe { *self.ptr.as_ref().unwrap() };
            self.ptr = unsafe { self.ptr.add(1) };
            Some(value as char)
        } else {
            None
        }
    }

    /// Peek at the next character in the Source
    /// (Will be used later)
    #[allow(dead_code)]
    pub(super) fn peek_char(&self) -> Option<char> {
        if self.ptr < self.end {
            let value = unsafe { *self.ptr.as_ref().unwrap() };
            Some(value as char)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_source_ptr_works() {
        let prgm = "123";
        let src = Source::new(prgm);

        assert_eq!(prgm.as_ptr(), src.ptr);
    }
}
