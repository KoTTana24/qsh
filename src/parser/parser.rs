use super::ast::*;
use super::lexer::Token;

pub fn parse(tokens: &[Token]) -> Option<Statement> {
    parse_sequence(tokens)
}

// Sequence parsing
fn parse_sequence(tokens: &[Token]) -> Option<Statement> {
    let mut statements = Vec::new();

    let mut start = 0;

    for (index, token) in tokens.iter().enumerate() {
        if matches!(token, Token::Semicolon) {
            if let Some(statement) = parse_logic(&tokens[start..index]) {
                statements.push(statement);
            }

            start = index + 1;
        }
    }

    if start < tokens.len() {
        if let Some(statement) = parse_logic(&tokens[start..]) {
            statements.push(statement);
        }
    }

    if statements.len() == 1 {
        Some(statements.remove(0))
    } else if !statements.is_empty() {
        Some(Statement::Sequence(statements))
    } else {
        None
    }
}

// Pipeline parsing
fn parse_pipeline(tokens: &[Token]) -> Option<Statement> {
    let mut commands = Vec::new();

    let mut start = 0;

    for (index, token) in tokens.iter().enumerate() {
        if matches!(token, Token::Pipe) {
            if let Some(command) = parse_command(&tokens[start..index]) {
                commands.push(command);
            }

            start = index + 1;
        }
    }

    if start < tokens.len() {
        if let Some(command) = parse_command(&tokens[start..]) {
            commands.push(command);
        }
    }

    if commands.len() == 1 {
        Some(Statement::Command(commands.remove(0)))
    } else if !commands.is_empty() {
        Some(Statement::Pipeline(Pipeline { commands }))
    } else {
        None
    }
}

// echo hello > file.txt
fn parse_command(tokens: &[Token]) -> Option<Command> {
    let mut command = Command {
        program: String::new(),

        args: Vec::new(),

        stdin: None,

        stdout: None,
    };

    let mut index = 0;

    while index < tokens.len() {
        match &tokens[index] {
            Token::Word { value, expand } => {
                let word = if *expand {
                    crate::expand::expand_word(value)
                } else {
                    value.clone()
                };

                if command.program.is_empty() {
                    command.program = word;
                } else {
                    command.args.push(word);
                }
            }

            Token::RedirectOut => {
                index += 1;

                if let Some(Token::Word { value, .. }) = tokens.get(index) {
                    command.stdout = Some(Redirect::Write(value.clone()));
                }
            }

            Token::AppendOut => {
                index += 1;

                if let Some(Token::Word { value, .. }) = tokens.get(index) {
                    command.stdout = Some(Redirect::Append(value.clone()));
                }
            }

            Token::RedirectIn => {
                index += 1;

                if let Some(Token::Word { value, .. }) = tokens.get(index) {
                    command.stdin = Some(value.clone());
                }
            }

            _ => {}
        }

        index += 1;
    }

    if command.program.is_empty() {
        None
    } else {
        Some(command)
    }
}

fn parse_logic(tokens: &[Token]) -> Option<Statement> {
    for (index, token) in tokens.iter().enumerate() {
        match token {
            Token::And => {
                let left = parse_logic(&tokens[..index])?;

                let right = parse_logic(&tokens[index + 1..])?;

                return Some(Statement::And(Box::new(left), Box::new(right)));
            }

            Token::Or => {
                let left = parse_logic(&tokens[..index])?;

                let right = parse_logic(&tokens[index + 1..])?;

                return Some(Statement::Or(Box::new(left), Box::new(right)));
            }

            _ => {}
        }
    }

    parse_pipeline(tokens)
}
