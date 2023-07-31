mod model;

use std::env;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

fn main() {
    let mut rai = if let Some(purpose) = env::args().nth(1) {
        model::Rai::from_purpose(purpose.as_str())
    } else {
        model::Rai::default()
    };

    println!("Ready.");

    loop {
        if let Some(msg) = rai.conversation.messages.last() {
            println!("< {}", msg.text);
        }
        print!("> ");
        let mut buffer = String::new();
        stdout().flush().unwrap();
        stdin().read_line(&mut buffer).unwrap();

        rai.prompt(buffer.as_str());
    }
}
