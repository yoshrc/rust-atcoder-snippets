//! Utilities.

// BEGIN SNIPPET utils

/// Output values by `println!("{} {} ... {}", value_1, value_2, ..., value_n`)`.
#[macro_export]
macro_rules! echo {
    () => {
        println!()
    };

    ($e: expr $(,)?) => {
        println!("{}", $e)
    };

    ($e: expr, $($es: expr),+ $(,)?) => {
        {
            use std::io::Write;
            let stdout = std::io::stdout();
            let mut handle = stdout.lock();
            write!(handle, "{}", $e).unwrap();
            $(
                write!(handle, " {}", $es).unwrap();
            )+
            writeln!(handle).unwrap();
        }
    };
}

/// Prints "Yes" or "No" according to `result`.
pub fn yn(result: bool) {
    if result {
        println!("Yes");
    } else {
        println!("No");
    }
}

// ABC038 A, ABC038 B, ABC114 A
/// Prints "YES" or "NO" according to `result`.
#[allow(non_snake_case)]
pub fn YN(result: bool) {
    if result {
        println!("YES");
    } else {
        println!("NO");
    }
}

/// Prints the given message with newline and exits the process successfully.
///
/// Useful for exiting after printing "-1" or "No" when it is found that
/// there is no solution for the given input.
pub fn exit(msg: impl std::fmt::Display) -> ! {
    println!("{}", msg);
    std::process::exit(0)
}

/// Make a debug output of the given expression to stderr.
///
/// The output is made only in the local machine, not in the judge server.
///
/// Similar to `dbg` macro in Rust 1.32.0.
#[macro_export]
#[cfg(local)]
macro_rules! dbg {
    () => {
        {
            use std::io::{self, Write};
            writeln!(io::stderr(), "{}: dbg", line!()).unwrap();
        }
    };

    ($e: expr) => {
        {
            use std::io::{self, Write};
            let result = $e;
            writeln!(io::stderr(), "{}: {} = {:?}",
                     line!(), stringify!($e), result)
                .unwrap();
            result
        }
    }
}

/// Make a debug output of the given expression to stderr.
///
/// The output is made only in the local machine, not in the judge server.
///
/// Similar to `dbg` macro in Rust 1.32.0.
#[macro_export]
#[cfg(not(local))]
macro_rules! dbg {
    () => {};
    ($e: expr) => {
        { $e }
    }
}

// END SNIPPET
