use anyhow::Result;
use std::io::{stdin, stdout, Write};

use interpret_deez::{
    evaluator::object::environment::Environment, evaluator::object::Object, evaluator::Evaluate,
    lexer::Lexer, parser::Parser,
};

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
    let env = Environment::new_heap(None);

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

        if query == "exit\n" || query.is_empty() {
            break;
        }

        let lexer = Lexer::new(&query);
        let parser = Parser::new(lexer);
        let statements_res: Result<Vec<_>> = parser.collect();

        match statements_res {
            Ok(stmts) => match stmts.eval_return(env.clone()) {
                Ok(Object::Null) => (),
                Ok(x) => println!("{x}"),
                Err(e) => {
                    println!("{monkey_face_2}");
                    println!("Evaluation error:\n    {e}");
                }
            },
            Err(e) => {
                println!("{monkey_face_2}");
                println!("Parsing error:\n    {e:?}");
            }
        }

        query.truncate(0);
    }

    println!("Bye")
}
