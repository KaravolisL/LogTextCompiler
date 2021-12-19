
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
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

impl Default for TokenType {
    fn default() -> Self {
        TokenType::EOF    
    }
}

#[derive(Default, Debug, Clone)]
pub struct Token {
    text: String,
    token_type: TokenType
}

impl Token {
    pub fn get_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn is_keyword(token_text: &str) -> Option<TokenType> {
        let mut retval: Option<TokenType> = None;
        match token_text {
            "TAG" => retval = Some(TokenType::TAG),
            "TASK" => retval = Some(TokenType::TASK),
            "ENDTASK" => retval = Some(TokenType::ENDTASK),
            "PERIOD" => retval = Some(TokenType::PERIOD),
            "EVENT" => retval = Some(TokenType::EVENT),
            "ROUTINE" => retval = Some(TokenType::ROUTINE),
            "ENDROUTINE" => retval = Some(TokenType::ENDROUTINE),
            "RUNG" => retval = Some(TokenType::RUNG),
            "ENDRUNG" => retval = Some(TokenType::ENDRUNG),
            "FALSE" => retval = Some(TokenType::FALSE),
            "TRUE" => retval = Some(TokenType::TRUE),
            "XIC" => retval = Some(TokenType::XIC),
            "XIO" => retval = Some(TokenType::XIO),
            "OTE" => retval = Some(TokenType::OTE),
            "OTL" => retval = Some(TokenType::OTL),
            "OTU" => retval = Some(TokenType::OTU),
            "JSR" => retval = Some(TokenType::JSR),
            "RET" => retval = Some(TokenType::RET),
            "EMIT" => retval = Some(TokenType::EMIT),
            _ => ()
        }
        retval
    }
}

pub struct Lexer {
    source_code: String,
    line_number: u32,
    current_character: char,
    current_position: usize
}

impl Lexer {
    pub fn new(mut source_code: String) -> Lexer {
        source_code.push('\n');
        let mut lexer = Lexer {
            source_code: source_code,
            line_number: 1,
            current_character: '\0',
            current_position: 0
        };
        lexer.current_character = lexer.source_code.chars().collect::<Vec<char>>()[0];
        lexer
    }

    fn next_character(&mut self) {
        if self.current_character == '\n' {
            self.line_number += 1;
        }

        self.current_position += 1;
        if self.current_position >= self.source_code.len() {
            self.current_character = '\0';
        } else {
            self.current_character = self.source_code.chars().collect::<Vec<char>>()[self.current_position];
        }
    }

    fn peek(&self) -> char{
        if self.current_position + 1 >= self.source_code.len() {
            return '\0';
        }
        self.source_code.chars().collect::<Vec<char>>()[self.current_position + 1]
    }

    fn skip_whitespace(&mut self) {
        while (self.current_character == ' ') ||
              (self.current_character == '\t') ||
              (self.current_character == '\r') {
            self.next_character();
        }
    }

    fn skip_comment(&mut self) {
        if self.current_character == '#' {
            while self.current_character != '\n' {
                self.next_character();
            }
        }
    }

    pub fn get_token(&mut self) -> Token {
        self.skip_whitespace();
        self.skip_comment();
        let mut token = Token::default();

        match self.current_character {
            '=' => {
                token.text = self.current_character.to_string();
                token.token_type = TokenType::EQ;
            },
            '<' => {
                token.text = self.current_character.to_string();
                token.token_type = TokenType::OPEN_ANGLE;
            },
            '>' => {
                token.text = self.current_character.to_string();
                token.token_type = TokenType::CLOSE_ANGLE;
            },
            '[' => {
                token.text = self.current_character.to_string();
                token.token_type = TokenType::OPEN_BRACKET;
            },
            ']' => {
                token.text = self.current_character.to_string();
                token.token_type = TokenType::CLOSE_BRACKET;
            },
            '\n' => {
                token.text = self.current_character.to_string();
                token.token_type = TokenType::NEWLINE;
            },
            '\0' => {
                token.text = self.current_character.to_string();
                token.token_type = TokenType::EOF;
            },
            '.' => {
                token.text = self.current_character.to_string();
                token.token_type = TokenType::INDEXER;
            }
            _ => {
                if self.current_character.is_digit(10) {
                    // Token is a number, so get all the next digits
                    let start_position = self.current_position;
                    while self.peek().is_digit(10) {
                        self.next_character();
                    }

                    // It could have a decimal point
                    if self.peek() == '.' {
                        self.next_character();

                        // We need to have at least one digit after the decimal
                        if !self.peek().is_digit(10) {
                            panic!("Illegal character in number");
                        }

                        // Get all the digits after the decimal point
                        while self.peek().is_digit(10) {
                            self.next_character();
                        }
                    }

                    // Construct the substring and token
                    let number = &self.source_code[start_position..self.current_position + 1];
                    token.text = number.to_string();
                    token.token_type = TokenType::NUMBER;
                } else if self.current_character.is_alphabetic() {
                    // Token is either a keyword or identifier
                    let start_position = self.current_position;
                    while self.peek().is_alphabetic() || self.peek().is_digit(10) {
                        self.next_character();
                    }

                    // Construct the substring and check if it's a keyword
                    let word = &self.source_code[start_position..self.current_position + 1];
                    token.text = word.to_string();

                    let keyword = Token::is_keyword(word);
                    token.token_type = keyword.unwrap_or(TokenType::IDENTIFIER);
                } else {
                    panic!("Unknown token: {}", self.current_character);
                }
            }
        }

        self.next_character();
        token
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
            assert_eq!(i, lexer.current_position);
            lexer.next_character();

            if i == test_input.len() - 1 {
                assert_eq!('\n', lexer.current_character);
                assert_eq!(i + 1, lexer.current_position);
            }
        }
        
        lexer.next_character();
        assert_eq!('\0', lexer.current_character);
    }

    #[test]
    fn test_peek() {
        let test_input = "test input".to_string();
        let lexer = Lexer::new(test_input.clone());
        assert_eq!(test_input.chars().collect::<Vec<char>>()[1], lexer.peek());
        assert_eq!(test_input.chars().collect::<Vec<char>>()[0], lexer.current_character);
    }

    #[test]
    fn test_get_token_success() {
        let test_input = "TASK<PERIOD=10.50> myTask # This is my task".to_string();
        let mut lexer = Lexer::new(test_input.clone());

        let mut token = lexer.get_token();
        assert_eq!(TokenType::TASK, token.token_type);
        assert_eq!("TASK", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::OPEN_ANGLE, token.token_type);
        assert_eq!("<", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::PERIOD, token.token_type);
        assert_eq!("PERIOD", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::EQ, token.token_type);
        assert_eq!("=", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::NUMBER, token.token_type);
        assert_eq!("10.50", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::CLOSE_ANGLE, token.token_type);
        assert_eq!(">", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::IDENTIFIER, token.token_type);
        assert_eq!("myTask", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::NEWLINE, token.token_type);
        assert_eq!("\n", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::EOF, token.token_type);
        assert_eq!("\0", token.text);
    }

    #[test]
    fn test_get_token_success_2() {
        let test_input = "TAG[10] myTagArray = FALSE".to_string();
        let mut lexer = Lexer::new(test_input.clone());

        let mut token = lexer.get_token();
        assert_eq!(TokenType::TAG, token.token_type);
        assert_eq!("TAG", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::OPEN_BRACKET, token.token_type);
        assert_eq!("[", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::NUMBER, token.token_type);
        assert_eq!("10", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::CLOSE_BRACKET, token.token_type);
        assert_eq!("]", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::IDENTIFIER, token.token_type);
        assert_eq!("myTagArray", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::EQ, token.token_type);
        assert_eq!("=", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::FALSE, token.token_type);
        assert_eq!("FALSE", token.text);
    }

    #[test]
    fn test_get_token_success_3() {
        let test_input = "OTE myTagArray.0".to_string();
        let mut lexer = Lexer::new(test_input.clone());

        let mut token = lexer.get_token();
        assert_eq!(TokenType::OTE, token.token_type);
        assert_eq!("OTE", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::IDENTIFIER, token.token_type);
        assert_eq!("myTagArray", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::INDEXER, token.token_type);
        assert_eq!(".", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::NUMBER, token.token_type);
        assert_eq!("0", token.text);
    }

    #[test]
    #[should_panic(expected="Illegal character in number")]
    fn test_get_token_failure_1() {
        let test_input = "TASK<PERIOD=10.> myTask # This is my task".to_string();
        let mut lexer = Lexer::new(test_input.clone());

        lexer.get_token();
        lexer.get_token();
        lexer.get_token();
        lexer.get_token();
        lexer.get_token();
    }

    #[test]
    #[should_panic(expected="Unknown token: _")]
    fn test_get_token_failure_2() {
        let test_input = "TASK<PERIOD=10.50> my_Task # This is my task".to_string();
        let mut lexer = Lexer::new(test_input.clone());

        lexer.get_token();
        lexer.get_token();
        lexer.get_token();
        lexer.get_token();
        lexer.get_token();
        lexer.get_token();
        lexer.get_token();
        lexer.get_token();
    }
}