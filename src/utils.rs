use std::io::Read;

pub fn get_char() -> u8 {
    let input = std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .unwrap();

    print!("{}", input);

    input
}
