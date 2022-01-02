
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Eof = -1,
    NewLine = 0,
    Number = 1,
    Identifier = 2,

    Tag = 101,
    Task = 102,
    EndTask = 103,
    Period = 104,
    Event = 105,
    Continuous = 106,
    Routine = 107,
    EndRoutine = 108,
    Rung = 109,
    EndRung = 110,
    False = 111,
    True = 112,
    Xic = 113,
    Xio = 114,
    Ote = 115,
    Otl = 116,
    Otu = 117,
    Jsr = 118,
    Ret = 119,
    Emit = 120,

    Eq = 201,
    OpenAngle = 202,
    CloseAngle = 203,
    OpenBracket = 204,
    CloseBracket = 205,
    Indexer = 206
}

impl Default for TokenType {
    fn default() -> Self {
        TokenType::Eof    
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
            "TAG" => retval = Some(TokenType::Tag),
            "TASK" => retval = Some(TokenType::Task),
            "ENDTASK" => retval = Some(TokenType::EndTask),
            "PERIOD" => retval = Some(TokenType::Period),
            "EVENT" => retval = Some(TokenType::Event),
            "CONTINUOUS" => retval = Some(TokenType::Continuous),
            "ROUTINE" => retval = Some(TokenType::Routine),
            "ENDROUTINE" => retval = Some(TokenType::EndRoutine),
            "RUNG" => retval = Some(TokenType::Rung),
            "ENDRUNG" => retval = Some(TokenType::EndRung),
            "FALSE" => retval = Some(TokenType::False),
            "TRUE" => retval = Some(TokenType::True),
            "XIC" => retval = Some(TokenType::Xic),
            "XIO" => retval = Some(TokenType::Xio),
            "OTE" => retval = Some(TokenType::Ote),
            "OTL" => retval = Some(TokenType::Otl),
            "OTU" => retval = Some(TokenType::Otu),
            "JSR" => retval = Some(TokenType::Jsr),
            "RET" => retval = Some(TokenType::Ret),
            "EMIT" => retval = Some(TokenType::Emit),
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
            source_code,
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
                token.token_type = TokenType::Eq;
            },
            '<' => {
                token.text = self.current_character.to_string();
                token.token_type = TokenType::OpenAngle;
            },
            '>' => {
                token.text = self.current_character.to_string();
                token.token_type = TokenType::CloseAngle;
            },
            '[' => {
                token.text = self.current_character.to_string();
                token.token_type = TokenType::OpenBracket;
            },
            ']' => {
                token.text = self.current_character.to_string();
                token.token_type = TokenType::CloseBracket;
            },
            '\n' => {
                token.text = self.current_character.to_string();
                token.token_type = TokenType::NewLine;
            },
            '\0' => {
                token.text = self.current_character.to_string();
                token.token_type = TokenType::Eof;
            },
            '.' => {
                token.text = self.current_character.to_string();
                token.token_type = TokenType::Indexer;
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
                    token.token_type = TokenType::Number;
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
                    token.token_type = keyword.unwrap_or(TokenType::Identifier);
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
        assert_eq!(TokenType::Task, token.token_type);
        assert_eq!("TASK", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::OpenAngle, token.token_type);
        assert_eq!("<", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::Period, token.token_type);
        assert_eq!("PERIOD", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::Eq, token.token_type);
        assert_eq!("=", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::Number, token.token_type);
        assert_eq!("10.50", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::CloseAngle, token.token_type);
        assert_eq!(">", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::Identifier, token.token_type);
        assert_eq!("myTask", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::NewLine, token.token_type);
        assert_eq!("\n", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::Eof, token.token_type);
        assert_eq!("\0", token.text);
    }

    #[test]
    fn test_get_token_success_2() {
        let test_input = "TAG[10] myTagArray = FALSE".to_string();
        let mut lexer = Lexer::new(test_input.clone());

        let mut token = lexer.get_token();
        assert_eq!(TokenType::Tag, token.token_type);
        assert_eq!("TAG", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::OpenBracket, token.token_type);
        assert_eq!("[", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::Number, token.token_type);
        assert_eq!("10", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::CloseBracket, token.token_type);
        assert_eq!("]", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::Identifier, token.token_type);
        assert_eq!("myTagArray", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::Eq, token.token_type);
        assert_eq!("=", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::False, token.token_type);
        assert_eq!("FALSE", token.text);
    }

    #[test]
    fn test_get_token_success_3() {
        let test_input = "OTE myTagArray.0".to_string();
        let mut lexer = Lexer::new(test_input.clone());

        let mut token = lexer.get_token();
        assert_eq!(TokenType::Ote, token.token_type);
        assert_eq!("OTE", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::Identifier, token.token_type);
        assert_eq!("myTagArray", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::Indexer, token.token_type);
        assert_eq!(".", token.text);

        token = lexer.get_token();
        assert_eq!(TokenType::Number, token.token_type);
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