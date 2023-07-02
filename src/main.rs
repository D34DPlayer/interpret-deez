use std::io::{stdin, stdout, Write};

use interpret_deez::{lexer::Lexer, parser::Parser};

fn main() {
    println!(
        "
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
                Err(err) => println!("Error: {}", err),
            }
        }

        query.truncate(0);
    }

    println!("Bye")
}
