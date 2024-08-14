use lexer::*;

fn test_lex(input: &str) -> Vec<TokenKind> {
  /*let mut token_kind_vector: Vec<TokenKind> = Vec::new();
  let tokens = lex(input);
  for t in tokens {
    token_kind_vector.push(t.kind);
  }
  token_kind_vector*/

  lex(input).iter().map(|t| t.get_kind()).collect::<Vec<TokenKind>>()

}

#[test]
fn test_01() {
  assert_eq!(test_lex("123"),vec![TokenKind::Digit, TokenKind::Digit, TokenKind::Digit, TokenKind::EOF]);
  let testresult = test_lex("123"); //Samir adding in this to show the tests working via print statement
  println!("Output of test_lex 1: {:?}", testresult);
}

#[test]
fn test_02() {
  assert_eq!(test_lex("abc"),vec![TokenKind::Alpha, TokenKind::Alpha, TokenKind::Alpha, TokenKind::EOF]);
  let testresult = test_lex("abc"); //Samir adding in this to show the tests working via print statement
  println!("Output of test_lex 2: {:?}", testresult);
}

#[test]
fn test_03() {
  assert_eq!(test_lex("hello world"),vec![TokenKind::Alpha, TokenKind::Alpha, TokenKind::Alpha, TokenKind::Alpha, TokenKind::Alpha, TokenKind::WhiteSpace, TokenKind::Alpha, TokenKind::Alpha, TokenKind::Alpha, TokenKind::Alpha, TokenKind::Alpha, TokenKind::EOF]);
  let testresult = test_lex("hello world"); //Samir adding in this to show the tests working via print statement
  println!("Output of test_lex 3: {:?}", testresult);
}

#[test]
fn test_04() {
  assert_eq!(test_lex("true"),vec![TokenKind::True, TokenKind::EOF]);
  let testresult = test_lex("true"); //Samir adding in this to show the tests working via print statement
  println!("Output of test_lex 4: {:?}", testresult);
}

#[test]
fn test_05() {
  assert_eq!(test_lex("false"),vec![TokenKind::False, TokenKind::EOF]);
  let testresult = test_lex("false"); //Samir adding in this to show the tests working via print statement
  println!("Output of test_lex 5: {:?}", testresult);
}

#[test]
fn test_06() {
  assert_eq!(test_lex("let x = 123;"),vec![
    TokenKind::Let, 
    TokenKind::WhiteSpace, 
    TokenKind::Alpha, 
    TokenKind::WhiteSpace,
    TokenKind::Equal,
    TokenKind::WhiteSpace,
    TokenKind::Digit,
    TokenKind::Digit,
    TokenKind::Digit,
    TokenKind::Semicolon,
    TokenKind::EOF,
  ]);
  let testresult = test_lex("let x = 123;"); //Samir adding in this to show the tests working via print statement
  println!("Output of test_lex 6: {:?}", testresult);

}

#[test]
fn test_07() {
  assert_eq!(test_lex(r#"let x = 123;let y="abc";"#),vec![
    TokenKind::Let, 
    TokenKind::WhiteSpace, 
    TokenKind::Alpha, 
    TokenKind::WhiteSpace,
    TokenKind::Equal,
    TokenKind::WhiteSpace,
    TokenKind::Digit,
    TokenKind::Digit,
    TokenKind::Digit,
    TokenKind::Semicolon,
    TokenKind::Let, 
    TokenKind::WhiteSpace, 
    TokenKind::Alpha, 
    TokenKind::Equal,
    TokenKind::Quote,
    TokenKind::Alpha, 
    TokenKind::Alpha, 
    TokenKind::Alpha, 
    TokenKind::Quote,
    TokenKind::Semicolon,
    TokenKind::EOF,
  ]);
  let testresult = test_lex(r#"let x = 123;let y="abc";"#); //Samir adding in this to show the tests working via print statement
  println!("Output of test_lex 7: {:?}", testresult);
}

#[test]
fn test_08() {
  assert_eq!(test_lex(r#"fn main() {}"#),vec![
    TokenKind::Fn, 
    TokenKind::WhiteSpace, 
    TokenKind::Alpha, 
    TokenKind::Alpha,
    TokenKind::Alpha,
    TokenKind::Alpha,
    TokenKind::LeftParen,
    TokenKind::RightParen,
    TokenKind::WhiteSpace,
    TokenKind::LeftCurly,
    TokenKind::RightCurly,
    TokenKind::EOF,
  ]);
  let testresult = test_lex(r#"fn main() {}"#); //Samir adding in this to show the tests working via print statement
  println!("Output of test_lex 8: {:?}", testresult);
}


#[test]
fn test_09() {
  assert_eq!(test_lex(r#"fn foo(a,b,c) {
  let x=a+1;
	let y=bar(c-b);
  return x*y;
}"#),vec![
    TokenKind::Fn, 
    TokenKind::WhiteSpace, 
    TokenKind::Alpha, 
    TokenKind::Alpha,
    TokenKind::Alpha,
    TokenKind::LeftParen,
    TokenKind::Alpha,
    TokenKind::Comma,
    TokenKind::Alpha,
    TokenKind::Comma,
    TokenKind::Alpha,
    TokenKind::RightParen,
    TokenKind::WhiteSpace,
    TokenKind::LeftCurly,
    TokenKind::WhiteSpace,
    TokenKind::WhiteSpace,
    TokenKind::WhiteSpace,
    TokenKind::Let, 
    TokenKind::WhiteSpace, 
    TokenKind::Alpha,
    TokenKind::Equal,
    TokenKind::Alpha,
    TokenKind::Plus,
    TokenKind::Digit,
    TokenKind::Semicolon,
    TokenKind::WhiteSpace, 
    TokenKind::WhiteSpace, 
    TokenKind::Let, 
    TokenKind::WhiteSpace,
    TokenKind::Alpha,
    TokenKind::Equal,
    TokenKind::Alpha,
    TokenKind::Alpha,
    TokenKind::Alpha,
    TokenKind::LeftParen,
    TokenKind::Alpha,
    TokenKind::Dash,
    TokenKind::Alpha,
    TokenKind::RightParen,
    TokenKind::Semicolon,
    TokenKind::WhiteSpace,
    TokenKind::WhiteSpace,
    TokenKind::WhiteSpace,
    TokenKind::Return, 
    TokenKind::WhiteSpace,
    TokenKind::Alpha,
    TokenKind::Other,
    TokenKind::Alpha,
    TokenKind::Semicolon,
    TokenKind::WhiteSpace,
    TokenKind::RightCurly,
    TokenKind::EOF,
  ]);
  let testresult = test_lex(r#"fn foo(a,b,c) {
    let x=a+1;
    let y=bar(c-b);
    return x*y;
  }"#); //Samir adding in this to show the tests working via print statement
  println!("Output of test_lex 8: {:?}", testresult);
}
