use std::collections::HashMap;

use crate::{ error::Error, parser::Expr, span::Spanned, token::Token, value::Value };

pub fn expr_to_text(expr: &Spanned<Expr>) -> String {
    fn walk(e: &Spanned<Expr>, parent_prec: u32) -> String {
        match &e.data {
            Expr::Value(v) => v.symbol(),

            Expr::Identifier(name) => name.clone(),

            Expr::Macro(name) => format!("{{{}}}", name),

            Expr::Unary { op, rhs } => {
                let prec = Token::Operator(*op).lbp();
                let rhs_s = walk(rhs, prec);
                format!("{}{}", op.symbol(), rhs_s)
            }

            Expr::Binary { op, lhs, rhs } => {
                let prec = Token::Operator(*op).lbp();
                let l = walk(lhs, prec);
                let r = walk(rhs, prec + 1);

                let s = format!("{l} {} {r}", op.symbol());

                if prec < parent_prec {
                    format!("({s})")
                } else {
                    s
                }
            }

            Expr::Comma { exprs } => {
                let s = exprs
                    .iter()
                    .map(|e| walk(e, 1))
                    .collect::<Vec<_>>()
                    .join(", ");
                if parent_prec > 1 {
                    format!("({s})")
                } else {
                    s
                }
            }

            Expr::Ternary { cond, statement1, statement2 } => {
                let s = format!(
                    "{} ? {} : {}",
                    walk(cond, 0),
                    walk(statement1, 0),
                    walk(statement2, 0)
                );
                if parent_prec > 0 {
                    format!("({s})")
                } else {
                    s
                }
            }

            Expr::Call { func, args } => {
                let f = walk(func, u32::MAX);
                let a = args
                    .iter()
                    .map(|e| walk(e, 0))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{f}({a})")
            }
        }
    }

    walk(expr, 0)
}

pub fn print_debug_expr(expr: Spanned<Expr>, indent: usize) {
    fn walk(e: &Spanned<Expr>, indent: usize) {
        let pad = "  ".repeat(indent);
        let span = e.span;

        match &e.data {
            Expr::Value(v) => {
                println!("{pad}Value {:?} @ {}..{}", v, span.start, span.end - 1);
            }
            Expr::Identifier(name) => {
                println!("{pad}Identifier {:?} @ {}..{}", name, span.start, span.end - 1);
            }
            Expr::Macro(name) => {
                println!("{pad}Macro {:?} @ {}..{}", name, span.start, span.end - 1);
            }
            Expr::Binary { op, lhs, rhs } => {
                println!("{pad}{:?} @ {}..{}", op, span.start, span.end - 1);
                println!("{pad}  lhs:");
                walk(lhs, indent + 2);
                println!("{pad}  rhs:");
                walk(rhs, indent + 2);
            }
            Expr::Unary { op, rhs } => {
                println!("{pad}{:?} @ {}..{}", op, span.start, span.end - 1);
                println!("{pad}  rhs:");
                walk(rhs, indent + 2);
            }
            Expr::Ternary { cond, statement1, statement2 } => {
                println!("{pad}@ {}..{}", span.start, span.end - 1);
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
            Expr::Comma { exprs } => {
                println!("{pad}Comma Operator @ {}..{}", span.start, span.end - 1);
                println!("{pad}  statements:");
                for (i, a) in exprs.iter().enumerate() {
                    println!("{pad}    [{i}]:");
                    walk(a, indent + 3);
                }
            }
        }
    }

    walk(&expr, indent);
}

pub fn print_debug_tokens(tokens: &[Spanned<Token>]) {
    println!("Tokens:");

    for (i, t) in tokens.iter().enumerate() {
        println!("  [{:02}] {:<20} @ {}..{}", i, t.data.symbol(), t.span.start, t.span.end - 1);
    }
}

pub fn print_debug_user_def_function(functions: &HashMap<String, Box<Spanned<Expr>>>, debug: bool) {
    println!("User-defined functions:");

    if functions.is_empty() {
        println!("  <empty>");
        return;
    }

    let mut names: Vec<_> = functions.keys().collect();
    names.sort();

    for name in names {
        if let Some(expr) = functions.get(name) {
            println!("  MACRO {{{}}} = {}", name, expr_to_text(expr));

            if debug {
                print_debug_expr(*expr.clone(), 2);
            }
        }
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
