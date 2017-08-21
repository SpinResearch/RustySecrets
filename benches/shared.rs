
#![cfg(test)]

pub fn secret_1kb() -> &'static [u8] {
    include_bytes!("resources/1KB.txt")
}

// pub fn secret_1mb() -> &'static [u8] {
//     include_bytes!("resources/1MB.jpg")
// }

// pub fn secret_5mb() -> &'static [u8] {
//     include_bytes!("resources/5MB.jpg")
// }

// pub fn secret_16mb() -> &'static [u8] {
//     include_bytes!("resources/16MB.jpg")
// }
