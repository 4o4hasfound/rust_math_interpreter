use std::collections::HashMap;
use std::io::{self, Write};

use crate::evaluater::EvalResultType;
use crate::{evaluater::evaluate_expr, value::Value};

mod error;
mod evaluater;
mod lexer;
mod operator;
mod parser;
mod span;
mod token;
mod value;

pub fn print_error(src: &str, err: &span::Spanned<error::Error>) {
    match &err.data {
        error::Error::LexingError(err) => {
            println!(
                "Lexing Error: {}",
                error::lexing_error::error_to_string(err.clone())
            )
        }
        error::Error::EvalError(err) => {
            println!(
                "Evaluation Error: {}",
                error::eval_error::error_to_string(err.clone())
            )
        }
        _ => {}
    };

    const MAX_WIDTH: usize = 50;

    let span = err.span;
    let src_len = src.len();

    let span_start = span.start.min(src_len);
    let span_end = (span.end - 1).min(src_len);

    // Anchor window at the middle of the span
    let mid = (span_start + span_end) / 2;
    let half = MAX_WIDTH / 2;

    let mut win_start = mid.saturating_sub(half);
    let win_end = (win_start + MAX_WIDTH).min(src_len);

    if win_end - win_start < MAX_WIDTH {
        win_start = win_end.saturating_sub(MAX_WIDTH);
    }

    let snippet = &src[win_start..win_end];

    print!("{}", snippet);

    let mut marker = String::new();

    let caret_start = span_start.saturating_sub(win_start);
    let caret_end = span_end.saturating_sub(win_start).min(snippet.len());

    for i in 0..snippet.len() {
        if i == caret_start || (i == caret_end && caret_start != caret_end) {
            marker.push('^');
        } else {
            marker.push(' ');
        }
    }

    println!("{}", marker);
}

pub fn print_debug_vars(vars: &HashMap<String, Value>) {
    println!("Variables:");

    if vars.is_empty() {
        println!("  <empty>");
        return;
    }

    let mut keys: Vec<_> = vars.keys().collect();
    keys.sort();

    for key in keys {
        if let Some(value) = vars.get(key) {
            println!("  {} = {:?}", key, value);
        }
    }
}

fn main() {
    let mut vars: HashMap<String, Value> = HashMap::new();
    let mut functions: HashMap<String, Box<dyn Fn(Vec<Value>) -> Result<Value, error::Error>>> =
        HashMap::new();
    let mut user_def_functions = HashMap::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.to_lowercase().starts_with("[exit]") {
            break;
        }

        if input.to_lowercase().starts_with("[variables]") {
            print_debug_vars(&vars);
            continue;
        }

        if input.to_lowercase().starts_with("[clear]") {
            vars.clear();
            continue;
        }

        if input.to_lowercase().starts_with("[del]") {
            let v = input.split("]").collect::<Vec<&str>>();
            if v.len() >= 2 {
                let s = v[1].strip_suffix("\r\n");
                if let Some(var_name0) = s
                    && let Some(var_name1) = var_name0.strip_prefix(' ')
                {
                    vars.remove(var_name1);
                }
            }
            continue;
        }

        let debug: bool;

        if input.to_lowercase().starts_with("[debug]") {
            debug = true;
            input = input[7..].to_string();
        } else {
            debug = false;
        }

        let res = parser::parse_string(input.as_str(), debug);

        if let Ok(t) = res {
            let result = evaluate_expr(&t, &mut vars, &mut user_def_functions, &functions);
            if let Ok(mut v) = result {
                let val = match v.result_type() {
                    EvalResultType::Value => v.as_value(),
                    EvalResultType::Ref => v.as_ref().cloned(),
                };

                match val {
                    Some(Value::Boolean(b)) => println!("{}", b),
                    Some(Value::Int(i)) => println!("{}", i),
                    Some(Value::Float(f)) => println!("{}", f),
                    _ => {}
                };
            } else {
                print_error(&input, &result.unwrap_err());
            }
        } else if let Err(err) = res {
            print_error(&input, &err);
        }
    }
}
