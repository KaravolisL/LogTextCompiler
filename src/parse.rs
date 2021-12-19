use crate::{lexer::{Lexer, Token, TokenType}, emitter::Emitter};


pub struct Parser<'a> {
    lexer: Lexer,
    emitter: Emitter<'a>,

    previous_token: Token,
    current_token: Token,
    peek_token: Token
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer, emitter: Emitter<'a>) -> Parser<'a> {
        let mut parser = Parser {
            lexer: lexer,
            emitter: emitter,
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
                // self.routine();
            },
            &TokenType::RUNG => {
                self.next_token();
                // self.rung();
            },
            &TokenType::XIC | &TokenType::XIO | &TokenType::OTE | 
            &TokenType::OTL | &TokenType::OTU | &TokenType::JSR | 
            &TokenType::RET | &TokenType::EMIT => {
                self.next_token();
                // self.instruction();
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

    fn tag(&mut self) {
        // Determine if this is a tag array or a single tag
        let mut _length: u32 = 0;
        if self.check_token(TokenType::OPEN_BRACKET) {
            _length = self.tag_array();
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

    fn end_rung(&mut self) {

    }

    fn end_routine(&mut self) {

    }

    fn end_task(&mut self) {
        self.emitter.emit_line("}");
    }

    fn tag_array(&mut self) -> u32{
        self.match_token(TokenType::OPEN_BRACKET);
        self.match_token(TokenType::NUMBER);

        let length: u32 = self.previous_token.get_text().parse().unwrap();
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
}
