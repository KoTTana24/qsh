use super::ast::*;
use super::lexer::Token;

pub fn parse(tokens: &[Token]) -> Option<Statement> {
    let mut commands = Vec::new();

    let mut current = Command {
        program: String::new(),
        args: Vec::new(),
        stdin: None,
        stdout: None,
    };

    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Word(word) => {
                if current.program.is_empty() {
                    current.program = word.clone();
                } else {
                    current.args.push(word.clone());
                }
            }

            Token::RedirectOut => {
                i += 1;

                if let Some(Token::Word(file)) = tokens.get(i) {
                    current.stdout = Some(Redirect::Write(file.clone()));
                }
            }

            Token::AppendOut => {
                i += 1;

                if let Some(Token::Word(file)) = tokens.get(i) {
                    current.stdout = Some(Redirect::Append(file.clone()));
                }
            }

            Token::Pipe => {
                commands.push(current);

                current = Command {
                    program: String::new(),
                    args: Vec::new(),
                    stdin: None,
                    stdout: None,
                };
            }

            _ => {}
        }

        i += 1;
    }

    if !current.program.is_empty() {
        commands.push(current);
    }

    if commands.len() == 1 {
        Some(Statement::Command(commands.remove(0)))
    } else if !commands.is_empty() {
        Some(Statement::Pipeline(Pipeline { commands }))
    } else {
        None
    }
}
