use std::io::{stdin, stdout, Write};

use interpret_deez::{evaluator::Evaluate, lexer::Lexer, object::Object, parser::Parser};

fn main() {
    let monkey_face_1 = r#"                __,__
       .--.  .-"     "-.  .--.
      / .. \/  .-. .-.  \/ .. \
     | |  '|  /   Y   \  |'  | |
     | \   \  \ 0 | 0 /  /   / |
      \ '- ,\.-"""""""-./, -' /
       ''-' /_   ^ ^   _\ '-''
           |  \._   _./  |
           \   \ '~' /   /
            '._ '-=-' _.'
               '-----'
    "#;
    let monkey_face_2 = r#"                __,__
       .--.  .-"     "-.  .--.
      / .. \/  .-. .-.  \/ .. \
     | |  '|  /___Y___\  |'  | |
     | \   \  \ 0 | 0 /  /   / |
      \ '- ,\.-"""""""-./, -' /
       ''-' /    ^ ^    \ '-''
           |   .-'~'-.   |
           \ _/-------\_ /
            '._       _.'
               '-----'
    "#;

    println!(
        "{monkey_face_1}
Monke REPL v0.0.0
Author: Carlos Ruiz Herrera
Type `exit` to leave.
"
    );

    let mut query = String::new();

    loop {
        print!("> ");
        if let Err(err) = stdout().flush() {
            println!("Stdout error: {err}");
            break;
        }

        if let Err(err) = stdin().read_line(&mut query) {
            println!("Stdin error: {err}");
            break;
        }

        if query == "exit\n" || query == "" {
            break;
        }

        let lexer = Lexer::new(&query);
        let parser = Parser::new(lexer);
        let mut error_shown = false;
        let mut result = Object::Null;

        for stmt in parser {
            match stmt {
                Ok(stmt) => {
                    result = stmt.eval();
                }
                Err(err) => {
                    if !error_shown {
                        println!("{monkey_face_2}");
                        error_shown = true;
                    }
                    println!("Parsing error: {}", err)
                }
            }
        }
        println!("{result}");

        query.truncate(0);
    }

    println!("Bye")
}
