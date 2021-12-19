
enum TokenType {
    EOF = -1,
    NEWLINE = 0,
    NUMBER = 1,
    IDENTIFIER = 2,

    TAG = 101,
    TASK = 102,
    ENDTASK = 103,
    PERIOD = 104,
    EVENT = 105,
    ROUTINE = 106,
    ENDROUTINE = 107,
    RUNG = 108,
    ENDRUNG = 109,
    FALSE = 110,
    TRUE = 111,
    XIC = 112,
    XIO = 113,
    OTE = 114,
    OTL = 115,
    OTU = 116,
    JSR = 117,
    RET = 118,
    EMIT = 119,

    EQ = 201,
    OPEN_ANGLE = 202,
    CLOSE_ANGLE = 203,
    OPEN_BRACKET = 204,
    CLOSE_BRACKET = 205,
    INDEXER = 206
}

pub struct Token {
    text: String,
    token_type: TokenType
}

impl Token {
    pub fn is_keyword(token_text: &str) {
        return;
    }
}

pub struct Lexer {
    source_code: String,
    line_number: u32,
    current_character: char,
    current_position: i32
}

impl Lexer {
    pub fn new(mut source_code: String) -> Lexer {
        source_code.push('\n');
        let mut lexer = Lexer {
            source_code: source_code,
            line_number: 1,
            current_character: '\0',
            current_position: -1
        };
        lexer.next_character();
        lexer
    }

    pub fn next_character(&mut self) {
        if self.current_character == '\n' {
            self.line_number += 1;
        }

        self.current_position += 1;
        if self.current_position >= self.source_code.len().try_into().unwrap() {
            self.current_character = '\0';
        } else {
            self.current_character = self.source_code.as_bytes()[self.current_position as usize] as char;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_character() {
        let test_input = "test input".to_string();
        let mut lexer = Lexer::new(test_input.clone());
        assert_eq!(test_input.clone() + "\n", lexer.source_code);

        for (i, character) in test_input.chars().enumerate() {
            assert_eq!(character, lexer.current_character);
            assert_eq!(i, lexer.current_position as usize);
            lexer.next_character();

            if i == test_input.len() - 1 {
                assert_eq!('\n', lexer.current_character);
                assert_eq!(i + 1, lexer.current_position as usize);
            }
        }
        
        lexer.next_character();
        assert_eq!('\0', lexer.current_character);
    }

    
}