fn main() {
    let mut stack = vec![];

    fn operator<F: FnOnce(&mut Vec<f64>, f64, f64) -> ()>(stack: &mut Vec<f64>, op: F) {
        match (stack.pop(), stack.pop()) {
            (Some(l), Some(r)) => op(stack, l, r),
            (None, _) => println!("No left operant"),
            (Some(a), None) => {
                stack.push(a);
                println!("No right operant")
            },
        }
    }

    for arg in ::std::env::args().skip(1) {
        match arg.as_ref() {
            "+" => operator(&mut stack, |s, l, r| s.push(r + l)),
            "-" => operator(&mut stack, |s, l, r| s.push(r - l)),
            "*" => operator(&mut stack, |s, l, r| s.push(r * l)),
            "/" => operator(&mut stack, |s, l, r| if l == 0.0 {
                s.push(l);
                s.push(r);
                println!("Cannot divide by zero");
            } else {
                s.push(r / l);
            }),

            "p" => if let Some(f) = stack.pop() {
                println!("{}", f);
            },

            "q" => ::std::process::exit(1),

            _ => if let Ok(s) = ::std::str::FromStr::from_str(&arg) {
                stack.push(s);
            } else {
                println!("Can not parse: '{}'", arg);
            },
        }
    }
}
