use std::collections::HashMap;

use crate::{parser::Expr, span::Spanned, token::Token, value::Value};


pub fn print_debug_expr(expr: Spanned<Expr>) {
    fn walk(e: &Spanned<Expr>, indent: usize) {
        let pad = "  ".repeat(indent);
        let span = e.span;

        match &e.data {
            Expr::Value(v) => {
                println!("{pad}Value {:?} @ {}..{}", v, span.start, span.end - 1);
            }
            Expr::Identifier(name) => {
                println!(
                    "{pad}Identifier {:?} @ {}..{}",
                    name,
                    span.start,
                    span.end - 1
                );
            }
            Expr::Binary { op, lhs, rhs } => {
                println!("{pad}Binary {:?} @ {}..{}", op, span.start, span.end - 1);
                println!("{pad}  lhs:");
                walk(lhs, indent + 2);
                println!("{pad}  rhs:");
                walk(rhs, indent + 2);
            }
            Expr::Unary { op, rhs } => {
                println!("{pad}Unary {:?} @ {}..{}", op, span.start, span.end - 1);
                println!("{pad}  rhs:");
                walk(rhs, indent + 2);
            }
            Expr::Ternary {
                cond,
                statement1,
                statement2,
            } => {
                println!("{pad}Ternary @ {}..{}", span.start, span.end - 1);
                println!("{pad}  cond:");
                walk(cond, indent + 2);
                println!("{pad}  then:");
                walk(statement1, indent + 2);
                println!("{pad}  else:");
                walk(statement2, indent + 2);
            }
            Expr::Call { func, args } => {
                println!("{pad}Call @ {}..{}", span.start, span.end - 1);
                println!("{pad}  func:");
                walk(func, indent + 2);

                println!("{pad}  args ({}):", args.len());
                for (i, a) in args.iter().enumerate() {
                    println!("{pad}    [{i}]:");
                    walk(a, indent + 3);
                }
            }
        }
    }

    walk(&expr, 0);
}

pub fn print_debug_tokens(tokens: &[Spanned<Token>]) {
    println!("Tokens:");

    for (i, t) in tokens.iter().enumerate() {
        println!(
            "  [{:02}] {:<20} @ {}..{}",
            i,
            t.data.symbol(),
            t.span.start,
            t.span.end - 1
        );
    }
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

