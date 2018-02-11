#![cfg(test)]

pub fn secret_1kb() -> &'static [u8] {
    include_bytes!("resources/1KB.txt")
}
