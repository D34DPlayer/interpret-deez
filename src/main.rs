use std::io::{stdin, stdout, Write};

use interpret_deez::{lexer::Lexer, parser::Parser};

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

    println!("");

    println!(
        "
{monkey_face_1}
Monke REPL v0.0.0
Author: Carlos Ruiz Herrera
Type `exit` to leave.
Type `verbose` to toggle verbose mode.
"
    );

    let mut query = String::new();
    let mut verbose = false;

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
        } else if query == "verbose\n" {
            verbose = !verbose;
            println!("Verbose mode: {}", verbose);
            query.truncate(0);
            continue;
        }

        let lexer = Lexer::new(&query);
        let parser = Parser::new(lexer);

        for stmt in parser {
            match stmt {
                Ok(stmt) => {
                    if verbose {
                        println!("AST: {:?}", stmt);
                    }
                    println!("{}", stmt);
                }
                Err(err) => println!(
                    "
{monkey_face_2}
Error: {}
                ",
                    err
                ),
            }
        }

        query.truncate(0);
    }

    println!("Bye")
}
