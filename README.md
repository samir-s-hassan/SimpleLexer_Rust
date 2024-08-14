# SimpleLexer_Rust

Created a lexer for the Asa compiler that classifies ASCII input into specific tokens like keywords, symbols, and characters. Implemented the lex() function to handle both single-byte and multi-byte tokens and manage whitespace and end-of-file markers. Validated functionality with thorough testing to ensure accuracy. 

This does not contain a main.rs file or properly configured binary target; project was done with test cases passing in mind.


## Required Features

The following functionality is implemented:

- [X] The lex() function correctly categorizes input bytes into tokens based on the defined categories, including Alpha, Digit, Whitespace, Grouping Symbols, Operators, Equal, Quote, Semicolon, Keyword, EOF, and Other
- [X] Properly identifies and maps single-byte tokens such as whitespace, grouping symbols, operators, the equal sign, the quote, and the semicolon
- [X] Recognizes and maps multi-byte keywords (true, false, fn, return, let) as single tokens, ensuring that these keywords are not split into individual characters
- [X] Appends an EOF token at the end of the token vector to signify the end of the input
- [X] Classifies any byte that does not fit into the predefined categories as an Other token
- [X] Ensures that the lex() function passes all the tests provided in tests/lex.rs, including edge cases and different input scenarios

## How to run

1. Run "cargo build" to compile the project
2. Run "cargo test" to check the passing test cases
3. Since there is no main.rs and this is not an application, running "cargo test" is enough to verify the lexer is working with our test case inputs
4. Enjoy the project!

## Video Walkthrough

N/A

## Notes

N/A

## License

Copyright 2024 Samir Hassan

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

> http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
