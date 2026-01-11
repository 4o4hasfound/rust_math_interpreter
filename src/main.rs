use std::collections::HashMap;
use std::io::{ self, Write };

use crate::debug::{ print_debug_user_def_function, print_debug_vars };
use crate::error::Error;
use crate::evaluater::EvalResultType;
use crate::parser::{ Expr, parse_string };
use crate::span::Spanned;
use crate::{ evaluater::evaluate_expr, value::Value };

mod error;
mod evaluater;
mod lexer;
mod operator;
mod parser;
mod span;
mod token;
mod value;
mod debug;
mod functions;

fn print_error(src: &str, err: &span::Spanned<error::Error>) {
    match &err.data {
        error::Error::LexingError(err) => {
            println!("Lexing Error: {}", error::lexing_error::error_to_string(err.clone()));
        }
        error::Error::EvalError(err) => {
            println!("Evaluation Error: {}", error::eval_error::error_to_string(err.clone()));
        }
        _ => {
            println!("Unexpected Error");
        }
    }

    const MAX_WIDTH: usize = 50;

    let span = err.span;
    let src_len = src.len();

    let span_start = span.start.min(src_len);
    let span_end = (span.end - 1).min(src_len);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CommandResult {
    End,
    Continue,
    None,
}

fn eval_command<'a>(
    input: &String,
    vars: &mut HashMap<String, Value>,
    functions: &HashMap<String, fn(&Vec<Value>) -> Result<Value, error::Error>>,
    user_def_functions: &'a mut HashMap<String, Box<Spanned<Expr>>>,
    debug: bool
) -> CommandResult {
    if input.to_lowercase().starts_with("[exit]") {
        CommandResult::End
    } else if input.to_lowercase().starts_with("[variables]") {
        print_debug_vars(&vars);
        CommandResult::Continue
    } else if input.to_lowercase().starts_with("[clear]") {
        vars.clear();
        CommandResult::Continue
    } else if input.to_lowercase().starts_with("[defs]") {
        print_debug_user_def_function(&user_def_functions, debug);
        CommandResult::Continue
    } else if input.to_lowercase().starts_with("[def ") {
        let command_end = input.find("]");
        let names: String;
        let expr_str: String;
        match command_end {
            Some(n) => {
                names = input[5..n].trim_start().to_string();
                expr_str = input[n + 1..].trim_start().to_string();
            }
            None => {
                return CommandResult::None;
            }
        }

        let expr = parse_string(&expr_str, debug);

        if let Ok(expr_ok) = expr {
            for name in names.split_whitespace() {
                user_def_functions.insert(name.to_string(), expr_ok.clone());
                println!("  MACRO(s) {{{}}} = {}", name, expr_str);
            }
        } else if let Err(err) = expr {
            print_error(&expr_str, &err);
        }

        CommandResult::Continue
    } else if input.to_lowercase().starts_with("[del]") {
        let v = input.split("]").collect::<Vec<&str>>();
        if v.len() >= 2 {
            let s = v[1].strip_suffix("\r\n");
            if let Some(var_name0) = s && let Some(var_name1) = var_name0.strip_prefix(' ') {
                vars.remove(var_name1);
            }
        }
        CommandResult::Continue
    } else {
        CommandResult::None
    }
}

fn main() {
    let mut vars: HashMap<String, Value> = HashMap::new();
    type Func = fn(&Vec<Value>) -> Result<Value, Error>;
    let functions: HashMap<String, fn(&Vec<Value>) -> Result<Value, error::Error>> = HashMap::from([
        ("to_bool".to_string(), functions::to_bool as Func),
        ("to_int".to_string(), functions::to_int as Func),
        ("to_float".to_string(), functions::to_float as Func),
        ("any".to_string(), functions::any as Func),
        ("all".to_string(), functions::all as Func),
        ("max".to_string(), functions::max as Func),
        ("min".to_string(), functions::min as Func),
        ("clamp".to_string(), functions::clamp as Func),
    ]);
    let mut user_def_functions = HashMap::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let debug: bool;
        if input.to_lowercase().starts_with("[debug]") {
            debug = true;
            input = input[7..].trim_start().to_string();
        } else {
            debug = false;
        }

        let command_res = eval_command(
            &input,
            &mut vars,
            &functions,
            &mut user_def_functions,
            debug
        );

        if command_res == CommandResult::Continue {
            continue;
        } else if command_res == CommandResult::End {
            break;
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
                }
            } else {
                print_error(&input, &result.unwrap_err());
            }
        } else if let Err(err) = res {
            print_error(&input, &err);
        }
    }
}
