use color_eyre::eyre::Result;
use decoder::{cipher, decipher, pyramid};

fn main() -> Result<()> {
    color_eyre::install()?;

    let cipher = cipher::place_cipher_in_btreemap()?;

    let pyramid = pyramid::build_pyramid(&cipher)?;

    let decrypted_message = decipher::decipher(cipher, pyramid)?;

    println!("{decrypted_message}");

    Ok(())
}


