use std::io::Read;

use console::Term;

pub fn get_char() -> Result<char, std::io::Error> {
    let term = Term::stdout();

    term.read_char()
}
