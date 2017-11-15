use std::collections::LinkedList as LL;
fn main() {
    let mut stack = LL::new();

    fn operator<F: FnOnce(&mut LL<f64>, f64, f64) -> ()>(stack: &mut LL<f64>, op: F) {
        match (stack.pop_back(), stack.pop_back()) {
            (Some(l), Some(r)) => op(stack, l, r),
            (None, _) => println!("No left operant"),
            (Some(a), None) => {
                stack.push_back(a);
                println!("No right operant")
            },
        }
    }

    for arg in ::std::env::args().skip(1) {
        match arg.as_ref() {
            "+" => operator(&mut stack, |s, l, r| s.push_back(r + l)),
            "-" => operator(&mut stack, |s, l, r| s.push_back(r - l)),
            "*" => operator(&mut stack, |s, l, r| s.push_back(r * l)),
            "/" => operator(&mut stack, |s, l, r| if l == 0.0 {
                s.push_back(l);
                s.push_back(r);
                println!("Cannot divide by zero");
            } else {
                s.push_back(r / l);
            }),

            "p" => if let Some(f) = stack.pop_back() {
                println!("{}", f);
            },

            "q" => ::std::process::exit(1),

            _ => if let Ok(s) = ::std::str::FromStr::from_str(&arg) {
                stack.push_back(s);
            } else {
                println!("Can not parse: '{}'", arg);
            },
        }
    }
}
