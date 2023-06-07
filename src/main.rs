use std::io::{stdin, stdout, Write};

use interpret_deez::{lexer::Lexer, parser::Parser};

fn main() {
    println!(
        "
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

        for stmt in parser {
            println!("{stmt:?}");
        }

        query.truncate(0);
    }

    println!("Bye")
}
