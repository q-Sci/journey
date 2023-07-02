use std::{thread, time};
use std::io::Write;

pub fn type_text(string: &str) {
    for c in string.chars() {
        print!("{}", c);
        let _ = std::io::stdout().flush(); // fixes some weird issue with delay only occurring after newline
        thread::sleep(time::Duration::from_millis(50));
    }
}
