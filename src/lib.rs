use std::io::{self, Write};

use std::fs;

use scanner::{build_scanner, Scanner};

mod scanner;
mod token;

pub struct Lox {
    had_error: bool,
    had_runtime_error: bool,
}

pub fn build_lox() -> Lox {
    Lox {
        had_error: false,
        had_runtime_error: false,
    }
}

impl Lox {
    // pub fn run_file(&self, path: &str) {
    //     let file_contents = fs::read_to_string(path).unwrap_or_else(|_| {
    //         writeln!(io::stderr(), "Failed to read file {}", path).unwrap();
    //         String::new()
    //     });
    //     self.run(file_contents);
    // }

    // fn run_prompt() {
    //     loop {
    //         print!("> ");
    //         io::stdout().flush().unwrap();
    //
    //         let mut line = String::new();
    //         io::stdin().read_line(&mut line).unwrap();
    //         run(&line);
    //     }
    // }

    pub fn run(&self, source: String) {
        let mut scanner = build_scanner(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{}", token.to_string());
        }
        println!("EOF  null");
    }

    fn error(&self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&self, line: usize, location: &str, message: &str) {
        writeln!(
            io::stderr(),
            "[line {}] Error {}: {}",
            line,
            location,
            message
        )
        .unwrap();
    }
}
