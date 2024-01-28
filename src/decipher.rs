use std::collections::BTreeMap;
use color_eyre::eyre::Result;
use crate::Pyramid;

pub fn decipher(cipher: BTreeMap<u32, String>, pyramid: Pyramid) -> Result<String> {
    let mut decrypted_message = String::new();

    for pyramid_row in pyramid {
        if let Some(code) = pyramid_row.last() {
            if let Some(message) = cipher.get(code) {
                let formatted_message = format!("{message} ");
                decrypted_message.push_str(&formatted_message);
            }
        }
    }

    Ok(decrypted_message)
}