use std::{collections::BTreeMap, fs};
use color_eyre::eyre::Result;

pub fn place_cipher_in_btreemap() -> Result<BTreeMap<u32, String>> {
    let mut cipher = BTreeMap::new();

    let contents = fs::read_to_string("message.txt")?;

    for line in contents.lines() {
        let mut iter = line.split_whitespace();

        let key = iter.next().unwrap().parse::<u32>().unwrap();
        let message = iter.next().unwrap().to_string();

        cipher.insert(key, message);
    }

    Ok(cipher)
}