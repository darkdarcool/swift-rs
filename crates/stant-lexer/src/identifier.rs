use crate::Lexer;


impl<'a> Lexer<'a> {
    pub(super) fn identifier_handler(&mut self) -> &'a str {
        let start = self.source.current_pos();
        while !self.source.is_at_end() {
            let byte = self.read_byte();

            if byte.is_ascii_alphanumeric() && (byte as char) != ' ' || byte == b'_' {
                // self.index += 1;
                self.bump();
                // println!("Byte: {}", byte as char);
            } else {
                break;
            }
        }
        // Return a slice of the source code
        &self.source.get_slice(start, self.source.current_pos())
    }

    pub(super) fn quoted_identifier_handler(&mut self) -> &'a str {
        let start = self.source.current_pos(); // self.index;
        // TODO: Make this function just call a string function (wait until char type shit)
        self.bump();
        while !self.source.is_at_end() {
            let byte = self.read_byte();
            if byte.is_ascii_alphanumeric() {
                // self.source.advance(1);
                // self.index += 1;
                self.bump();
            } else {
                break;
            }
        }
        // Move past the ending '`'
        // self.index += 1;
        self.bump();

        &self.source.get_slice(start, self.source.get_current_pos())
    }
}
