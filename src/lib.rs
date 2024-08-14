#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    start_line: u32,
    end_line: u32,
    start_col: u32,
    end_col: u32,
}

impl Token {
    pub fn new() -> Token {
        Token {
            kind: TokenKind::Other,
            lexeme: "".to_string(),
            start_line: 0,
            end_line: 0,
            start_col: 0,
            end_col: 0,
        }
    }

    pub fn get_kind(&self) -> TokenKind {
        self.kind
    }

    pub fn set_kind(&mut self, new_kind: TokenKind) {
        self.kind = new_kind;
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    // Keywords
    True,
    False,
    Fn,
    Return,
    Let,
    //------
    Alpha,
    Digit,
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    Equal,
    Plus,
    Dash,
    Quote,
    WhiteSpace,
    Semicolon,
    Comma,
    Other,
    EOF,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new(); //our return vector
    let mut sl = 0; //start line, end line, start column, and end column
    let mut el = 0; 
    let mut sc = 0;
    let mut counter = 0; //this keeps track of the byte we are at

    while counter < input.len() {
        // Step 1. Loop over each byte of the input
        //print!("{}", character); for future reference, this is how to print a character, {} is the placeholder
        let mut new_token = Token::new();
        let current_char = input.chars().nth(counter);
        new_token.start_line = sl; // the row doesn't change unless a new line is encountered
        new_token.end_line = el; // the row doesn't change unless a new line is encountered
        // Step 2. Match byte against TokenKinds
        if
            (input.chars().nth(counter) == Some('t') || input.chars().nth(counter) == Some('T')) &&
            input.chars().nth(counter + 1) == Some('r') &&
            input.chars().nth(counter + 2) == Some('u') &&
            input.chars().nth(counter + 3) == Some('e')
        {
            counter += 4;
            new_token.kind = TokenKind::True;
            new_token.lexeme = "True".to_string();
        } else if
            (input.chars().nth(counter) == Some('f') || input.chars().nth(counter) == Some('F')) &&
            input.chars().nth(counter + 1) == Some('n')
        {
            counter += 2;
            new_token.kind = TokenKind::Fn;
            new_token.lexeme = "Fn".to_string();
        } else if
            (input.chars().nth(counter) == Some('f') || input.chars().nth(counter) == Some('F')) &&
            input.chars().nth(counter + 1) == Some('a') &&
            input.chars().nth(counter + 2) == Some('l') &&
            input.chars().nth(counter + 3) == Some('s') &&
            input.chars().nth(counter + 4) == Some('e')
        {
            counter += 5;
            new_token.kind = TokenKind::False;
            new_token.lexeme = "False".to_string();
        } else if
            (input.chars().nth(counter) == Some('R') || input.chars().nth(counter) == Some('r')) &&
            input.chars().nth(counter + 1) == Some('e') &&
            input.chars().nth(counter + 2) == Some('t') &&
            input.chars().nth(counter + 3) == Some('u') &&
            input.chars().nth(counter + 4) == Some('r') &&
            input.chars().nth(counter + 5) == Some('n')
        {
            counter += 6;
            new_token.kind = TokenKind::Return;
            new_token.lexeme = "Return".to_string();
        } else if
            (input.chars().nth(counter) == Some('l') || input.chars().nth(counter) == Some('L')) &&
            input.chars().nth(counter + 1) == Some('e') &&
            input.chars().nth(counter + 2) == Some('t')
        {
            counter += 3;
            new_token.kind = TokenKind::Let;
            new_token.lexeme = "Let".to_string();
        } else {
            //we've CHECKED FOR ALL THE KEYWORDS ABOVE
            match current_char {
                Some('A'..='Z') | Some('a'..='z') => {
                    new_token.kind = TokenKind::Alpha;
                    counter += 1;
                    new_token.end_col += counter as u32; // the column ends at the length of the word
                }
                Some('0'..='9') => {
                    new_token.kind = TokenKind::Digit;
                    counter += 1;
                    new_token.end_col += counter as u32; // the column ends at the length of the word
                }
                Some('(') => {
                    new_token.kind = TokenKind::LeftParen;
                    counter += 1;
                    new_token.end_col += counter as u32; // the column ends at the length of the word
                }
                Some(')') => {
                    new_token.kind = TokenKind::RightParen;
                    counter += 1;
                    new_token.end_col += counter as u32; // the column ends at the length of the word
                }
                Some('{') => {
                    new_token.kind = TokenKind::LeftCurly;
                    counter += 1;
                    new_token.end_col += counter as u32; // the column ends at the length of the word
                }
                Some('}') => {
                    new_token.kind = TokenKind::RightCurly;
                    counter += 1;
                    new_token.end_col += counter as u32; // the column ends at the length of the word
                }
                Some('=') => {
                    new_token.kind = TokenKind::Equal;
                    counter += 1;
                    new_token.end_col += counter as u32; // the column ends at the length of the word
                }
                Some('+') => {
                    new_token.kind = TokenKind::Plus;
                    counter += 1;
                    new_token.end_col += counter as u32; // the column ends at the length of the word
                }
                Some('-') => {
                    new_token.kind = TokenKind::Dash;
                    counter += 1;
                    new_token.end_col += counter as u32; // the column ends at the length of the word
                }
                Some('"') => {
                    new_token.kind = TokenKind::Quote;
                    counter += 1;
                    new_token.end_col += counter as u32; // the column ends at the length of the word
                }
                Some(' ') => {
                    new_token.kind = TokenKind::WhiteSpace;
                    counter += 1;
                    new_token.end_col += counter as u32; // the column ends at the length of the word
                }
                Some('\t') => {
                    new_token.kind = TokenKind::WhiteSpace;
                    counter += 1;
                    new_token.end_col += 4; // the column ends at the length of the word
                }
                Some('\n') => {
                    new_token.kind = TokenKind::WhiteSpace;
                    sl += 1; //only part where we have to go to a new row
                    el += 1;
                    counter += 1;
                    sc = 0; // the column is reset
                    new_token.end_col = 0; // the column is reset
                }
                Some(';') => {
                    new_token.kind = TokenKind::Semicolon;
                    counter += 1;
                    new_token.end_col += counter as u32; // the column ends at the length of the word
                }
                Some(',') => {
                    new_token.kind = TokenKind::Comma;
                    counter += 1;
                    new_token.end_col += counter as u32; // the column ends at the length of the word
                }
                _ => {
                    new_token.kind = TokenKind::Other;
                    counter += 1;
                    new_token.end_col += counter as u32; // the column ends at the length of the word

                }
            }
        }
        // Step 3. Form the Token, keep track of lines and cols
        new_token.start_col = sc; // the start of the column doesn't change
        sc += counter as u32; //for the next iteration of the loop, we'll start at a new start column
        tokens.push(new_token); //push this new token in
    }
    let mut last_token = Token::new(); //make the EOF token
    last_token.set_kind(TokenKind::EOF); //set the kind of the EOF, we don't care about the other fields
    tokens.push(last_token); //push it into the vector
    return tokens; //return our final vector
}