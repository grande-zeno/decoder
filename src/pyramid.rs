use std::collections::BTreeMap;
use color_eyre::eyre::Result;
use crate::Pyramid;

pub fn build_pyramid(cipher: &BTreeMap<u32, String>) -> Result<Pyramid> {
    let mut pyramid: Pyramid = vec![];
    let keys: Vec<u32> = cipher.iter().map(|(key, _)| *key).collect();
    let mut start = 0;
    let mut end = 1;

    for row in 1..=keys.len() {
        if let Some(pyramid_row) = keys.get(start..end) {
            pyramid.push(pyramid_row.to_vec());

            start = end;
            end = start + row + 1;
        } else {
            break;
        }
    }

    Ok(pyramid)
}