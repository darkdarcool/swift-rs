use crate::Lexer;

impl<'a> Lexer<'a> {
    pub(super) fn identifier_handler(&mut self) -> &'a str {
        let start = self.index;
        while !self.source.is_at_end() {
            let byte = self.read_byte();

            if byte.is_ascii_alphanumeric() && (byte as char) != ' ' || byte == b'_' {
                println!("Byte: {}", byte as char);
                self.index += 1;
                // println!("Byte: {}", byte as char);
            } else {
                break;
            }
        }
        // Return a slice of the source code
        &self.source.get_slice(start, self.index)
    }

    pub(super) fn quoted_identifier_handler(&mut self) -> &'a str {
        let start = self.index;
        // TODO: Make this function just call a string function (wait until char type shit)
        self.bump();
        while !self.source.is_at_end() {
            let byte = self.read_byte();
            if byte.is_ascii_alphanumeric() {
                // self.source.advance(1);
                self.index += 1;
            } else {
                break;
            }
        }
        // Move past the ending '`'
        self.index += 1;

        &self.source.get_slice(start, self.index)
    }
}
