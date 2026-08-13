#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(String),

    Pipe,      // |
    Semicolon, // ;

    RedirectOut, // >
    RedirectIn,  // <
    AppendOut,   // >>

    Background, // &
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut current = String::new();

    let push_word = |tokens: &mut Vec<Token>, word: &mut String| {
        if !word.is_empty() {
            tokens.push(Token::Word(word.clone()));

            word.clear();
        }
    };

    let chars: Vec<char> = input.chars().collect();

    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            ' ' | '\t' => {
                push_word(&mut tokens, &mut current);
            }

            '|' => {
                push_word(&mut tokens, &mut current);

                tokens.push(Token::Pipe);
            }

            ';' => {
                push_word(&mut tokens, &mut current);

                tokens.push(Token::Semicolon);
            }

            '>' => {
                push_word(&mut tokens, &mut current);

                if i + 1 < chars.len() && chars[i + 1] == '>' {
                    tokens.push(Token::AppendOut);
                    i += 1;
                } else {
                    tokens.push(Token::RedirectOut);
                }
            }

            '<' => {
                push_word(&mut tokens, &mut current);

                tokens.push(Token::RedirectIn);
            }

            '&' => {
                push_word(&mut tokens, &mut current);

                tokens.push(Token::Background);
            }

            c => {
                current.push(c);
            }
        }

        i += 1;
    }

    push_word(&mut tokens, &mut current);

    tokens
}
