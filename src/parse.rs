use crate::{lexer::{Lexer, Token, TokenType}, emitter::Emitter};

#[derive(Clone)]
struct TagDescriptor {
    name: String,
    length: usize
}

pub struct Parser<'a> {
    lexer: Lexer,
    emitter: Emitter<'a>,

    tags: Vec<TagDescriptor>,
    main_flag: bool,

    previous_token: Token,
    current_token: Token,
    peek_token: Token
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer, emitter: Emitter<'a>) -> Parser<'a> {
        let mut parser = Parser {
            lexer: lexer,
            emitter: emitter,
            tags: Vec::new(),
            main_flag: false,
            previous_token: Token::default(),
            current_token: Token::default(),
            peek_token: Token::default()
        };

        // Call next token twice to initialize current and peek
        parser.next_token();
        parser.next_token();
        parser
    }

    fn check_token(&self, token_type: TokenType) -> bool {
        token_type == *(self.current_token.get_type())
    }

    fn check_peek(&self, token_type: TokenType) -> bool {
        token_type == *(self.peek_token.get_type())
    }

    fn match_token(&mut self, token_type: TokenType) {
        if !self.check_token(token_type) {
            panic!("Expected {:?}, but found {:?}", token_type, self.current_token);
        }
        self.next_token();
    }

    fn next_token(&mut self) {
        self.previous_token = self.current_token.clone();
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.get_token();
    }

    pub fn program(&mut self) {
        // Parse all of the statements
        while !self.check_token(TokenType::EOF) {
            self.statement();
        }

        self.emitter.write_file();
    }

    fn statement(&mut self) {
        match self.current_token.get_type() {
            &TokenType::TASK => {
                self.next_token();
                self.task();
            },
            &TokenType::ROUTINE => {
                self.next_token();
                self.routine();
            },
            &TokenType::RUNG => {
                self.next_token();
                self.rung();
            },
            &TokenType::XIC | &TokenType::XIO | &TokenType::OTE |
            &TokenType::OTL | &TokenType::OTU | &TokenType::JSR |
            &TokenType::RET | &TokenType::EMIT => {
                self.next_token();
                self.instruction();
            },
            &TokenType::ENDRUNG => {
                self.next_token();
                self.end_rung();
            },
            &TokenType::ENDROUTINE => {
                self.next_token();
                self.end_routine();
            },
            &TokenType::ENDTASK => {
                self.next_token();
                self.end_task();
            },
            &TokenType::TAG => {
                self.next_token();
                self.tag();
            },
            _ => {
                panic!("Invalid statement at {} ({:?})", self.current_token.get_text(), self.current_token.get_type());
            }
        }

        // All statements end in nl
        self.new_line()
    }

    fn task(&mut self) {
        self.emitter.emit("TASK ");

        self.task_type();
        self.match_token(TokenType::IDENTIFIER);
        self.emitter.emit(" ");
        self.emitter.emit_line(self.previous_token.get_text());
        self.emitter.emit_line("{");
    }

    fn task_type(&mut self) {
        // Require an open bracket
        self.match_token(TokenType::OPEN_ANGLE);

        // Determine whether it's periodic or event driven
        if self.check_token(TokenType::PERIOD) {
            self.period_type();
        } else if self.check_token(TokenType::EVENT) {
            self.event_type();
        } else {
            panic!("Invalid task type {}", self.current_token.get_text());
        }

        // Require a closing bracket
        self.match_token(TokenType::CLOSE_ANGLE);
    }

    fn period_type(&mut self) {
        // Require the following tokens
        self.match_token(TokenType::PERIOD);
        self.emitter.emit("PERIOD ");
        self.match_token(TokenType::EQ);
        self.match_token(TokenType::NUMBER);
        self.emitter.emit(self.previous_token.get_text());

        // Enforce a lower bound on the period
        const PERIOD_LOWER_BOUND: u32 = 20;
        if self.previous_token.get_text().parse::<u32>().unwrap() < PERIOD_LOWER_BOUND {
            panic!("Period below allowable limit {}", PERIOD_LOWER_BOUND);
        }
    }

    fn event_type(&mut self) {
        // Require the following tokens
        self.match_token(TokenType::EVENT);
        self.emitter.emit("EVENT ");
        self.match_token(TokenType::EQ);
        self.match_token(TokenType::IDENTIFIER);
        self.emitter.emit(self.previous_token.get_text());
    }

    fn routine(&mut self) {
        self.match_token(TokenType::IDENTIFIER);

        // Determine if this is a Main routine or not
        if self.previous_token.get_text() == "Main" {
            if self.main_flag {
                panic!("There can only be one Main routine");
            } else {
                self.main_flag = true;
            }
        }
    }

    fn rung(&mut self) {
        if self.check_token(TokenType::IDENTIFIER) {
            self.next_token();
        }
    }

    fn instruction(&mut self) {
        let instruction_type = self.previous_token.get_type().to_owned();

        if instruction_type == TokenType::RET {
            return;
        }

        self.match_token(TokenType::IDENTIFIER);
        let mut target = self.previous_token.get_text().to_string();

        match instruction_type {
            TokenType::JSR => {

            },
            TokenType::EMIT => {

            },
            _ => {
                let tag_descriptor = self.tags.iter()
                                                           .find(|&item| item.name == target)
                                                           .or_else(|| {
                                                                panic!("Referencing tag {} before assignment", target);
                                                           }).unwrap().clone();

                // We are referencing a tag array, so require an index
                if tag_descriptor.length != 0 {
                    self.match_token(TokenType::INDEXER);
                    target += self.previous_token.get_text();

                    self.match_token(TokenType::NUMBER);
                    target += self.previous_token.get_text();

                    if self.previous_token.get_text().parse::<usize>().unwrap() >= tag_descriptor.length {
                        panic!("Index {} is out of bounds for tag array of length {}", self.previous_token.get_text(),
                                                                                       tag_descriptor.length);
                    }
                }
            }
        }
    }

    fn end_rung(&mut self) {

    }

    fn end_routine(&mut self) {

    }

    fn end_task(&mut self) {

        if !self.main_flag {
            panic!("There must be a single Main routine");
        } else {
            self.main_flag = false;
        }

        self.emitter.emit_line("}");
    }

    fn tag(&mut self) {
        // Determine if this is a tag array or a single tag
        let mut length: usize = 0;
        if self.check_token(TokenType::OPEN_BRACKET) {
            length = self.tag_array();
        } else {
            self.emitter.emit("TAG ");
        }

        self.match_token(TokenType::IDENTIFIER);
        self.emitter.emit(self.previous_token.get_text());

        // Enforce a charater limit on tag names
        const TAG_CHARACTER_LIMIT: usize = 7;
        if self.previous_token.get_text().len() > TAG_CHARACTER_LIMIT {
            panic!("Tag name {} too long. The limit is {} characters",
                   self.previous_token.get_text(), TAG_CHARACTER_LIMIT);
        }

        self.tags.push(TagDescriptor {
            name: self.previous_token.get_text().to_string(),
            length: length
        });
        self.match_token(TokenType::EQ);

        // Either true or false are acceptable
        if self.check_token(TokenType::TRUE) {
            self.match_token(TokenType::TRUE);
            self.emitter.emit_line(" TRUE");
        } else {
            self.match_token(TokenType::FALSE);
            self.emitter.emit_line(" FALSE");
        }
    }

    fn tag_array(&mut self) -> usize{
        self.match_token(TokenType::OPEN_BRACKET);
        self.match_token(TokenType::NUMBER);

        let length: usize = self.previous_token.get_text().parse().unwrap();
        if length == 0 {
            panic!("Length of tag array must be greater than zero");
        }

        self.emitter.emit("TAG_ARRAY ");
        self.emitter.emit(self.previous_token.get_text());
        self.emitter.emit(" ");

        self.match_token(TokenType::CLOSE_BRACKET);
        length
    }

    fn new_line(&mut self) {
        self.match_token(TokenType::NEWLINE);
        while self.check_token(TokenType::NEWLINE) {
            self.next_token();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statement_tag_1() {
        let source_code = "TAG myTag = TRUE\nTAG myTag = FALSE".to_string();
        let mut par = Parser::new(Lexer::new(source_code.clone()), Emitter::new("test.out"));
        par.program()
    }

    #[test]
    #[should_panic]
    fn test_statement_tag_2() {
        let source_code = "TAG myTag = notAKeyword".to_string();
        let mut par = Parser::new(Lexer::new(source_code.clone()), Emitter::new("test.out"));
        par.program()
    }

    #[test]
    #[should_panic]
    fn test_statement_tag_3() {
        let source_code = "TAG myLongTagName = FALSE".to_string();
        let mut par = Parser::new(Lexer::new(source_code.clone()), Emitter::new("test.out"));
        par.program()
    }

    #[test]
    fn test_statement_task_1() {
        let source_code = "TASK<PERIOD=1000> myTask".to_string();
        let mut par = Parser::new(Lexer::new(source_code.clone()), Emitter::new("test.out"));
        par.program()
    }

    #[test]
    fn test_statement_task_2() {
        let source_code = "TASK<EVENT=myEvent> myTask".to_string();
        let mut par = Parser::new(Lexer::new(source_code.clone()), Emitter::new("test.out"));
        par.program()
    }

    #[test]
    #[should_panic]
    fn test_statement_task_3() {
        let source_code = "TASK<CONTINUOUS> myTask".to_string();
        let mut par = Parser::new(Lexer::new(source_code.clone()), Emitter::new("test.out"));
        par.program()
    }

    #[test]
    fn test_statement_routine_success() {
        let source_code = "ROUTINE Main".to_string();
        let mut par = Parser::new(Lexer::new(source_code.clone()), Emitter::new("test.out"));
        par.program()
    }

    #[test]
    #[should_panic]
    fn test_statement_routine_failure() {
        let source_code = "ROUTINE ".to_string();
        let mut par = Parser::new(Lexer::new(source_code.clone()), Emitter::new("test.out"));
        par.program()
    }

    #[test]
    fn test_statement_rung_1() {
        let source_code = "RUNG".to_string();
        let mut par = Parser::new(Lexer::new(source_code.clone()), Emitter::new("test.out"));
        par.program()
    }

    #[test]
    fn test_statement_rung_2() {
        let source_code = "RUNG myRung".to_string();
        let mut par = Parser::new(Lexer::new(source_code.clone()), Emitter::new("test.out"));
        par.program()
    }

    #[test]
    fn test_statement_instructions() {
        let source_code = "XIC tag\nXIO tag\nOTE tag\nOTL tag\nOTU tag\nJSR routine\nEMIT event\nRET".to_string();
        let mut par = Parser::new(Lexer::new(source_code.clone()), Emitter::new("test.out"));
        
        // Add tag to the symbols to avoid errors
        par.tags.push(TagDescriptor {
            name: "tag".to_string(),
            length: 0
        });
        
        par.program()
    }

    #[test]
    fn test_statement_end() {
        let source_code = "ENDRUNG\nENDROUTINE\nENDTASK".to_string();
        let mut par = Parser::new(Lexer::new(source_code.clone()), Emitter::new("test.out"));
        par.main_flag = true;
        
        par.program()
    }

    #[test]
    fn test_statement_tag_array_1() {
        let source_code = "TAG[10] array = FALSE\nOTE array.0".to_string();
        let mut par = Parser::new(Lexer::new(source_code.clone()), Emitter::new("test.out"));
        par.program()
    }

    #[test]
    #[should_panic(expected="Length of tag array must be greater than zero")]
    fn test_statement_tag_array_2() {
        let source_code = "TAG[0] array = FALSE".to_string();
        let mut par = Parser::new(Lexer::new(source_code.clone()), Emitter::new("test.out"));
        par.program()
    }

    #[test]
    #[should_panic]
    fn test_statement_tag_array_3() {
        let source_code = "TAG[10] array = FALSE\nOTE array".to_string();
        let mut par = Parser::new(Lexer::new(source_code.clone()), Emitter::new("test.out"));
        par.program()
    }

    #[test]
    #[should_panic(expected="Referencing tag array before assignment")]
    fn test_statement_tag_array_4() {
        let source_code = "OTE array.2".to_string();
        let mut par = Parser::new(Lexer::new(source_code.clone()), Emitter::new("test.out"));
        par.program()
    }

    #[test]
    #[should_panic]
    fn test_statement_tag_array_5() {
        let source_code = "TAG[10] array = FALSE\nOTE array.10".to_string();
        let mut par = Parser::new(Lexer::new(source_code.clone()), Emitter::new("test.out"));
        par.program()
    }
}
