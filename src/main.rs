use std::io;

fn trithemius_cipher(message: &str, shift: u8) -> String {
    let message_chars: Vec<char> = message.chars().collect();

    let mut result: String = String::new();

    for &c in message_chars.iter() {
        if c.is_alphabetic() {
            let base: u8 = if c.is_lowercase() {
                'a' as u8
            } else {
                'A' as u8
            };
            let encrypted_char: char = ((((c as u8 - base) + shift) % 26) + base) as char;

            result.push(encrypted_char);
        } else {
            result.push(c);
        }
    }

    result
}

fn trithemius_decipher(message: &str, shift: u8) -> String {
    let message_chars: Vec<char> = message.chars().collect();

    let mut result: String = String::new();

    for &c in message_chars.iter() {
        if c.is_alphabetic() {
            let base: u8 = if c.is_lowercase() {
                'a' as u8
            } else {
                'A' as u8
            };
            let decrypted_char: char = ((((c as u8 - base) + 26 - shift) % 26) + base) as char;
            result.push(decrypted_char);
        } else {
            result.push(c);
        }
    }

    result
}

fn main() {
    println!("Message to encrypt: ");
    let mut message: String = String::new();
    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read message");

    println!("Enter shift value: ");
    let mut shift_str: String = String::new();
    io::stdin()
        .read_line(&mut shift_str)
        .expect("Failed to read shift");

    let shift: u8 = match shift_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid shift value, Defaulting to 4");
            4
        }
    };

    let encrypted_message: String = trithemius_cipher(&message, shift);
    println!("Encrypted message: {}", encrypted_message);
}
