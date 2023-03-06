pub use multimap::MultiMap;

pub fn custom_sections(bytes: &[u8]) -> Result<MultiMap<String, Vec<u8>>, String> {
    use wasmparser::{Parser, Payload};
    Parser::new(0)
        .parse_all(bytes)
        .filter_map(|payload| {
            let payload = match payload {
                Ok(payload) => payload,
                Err(e) => return Some(Err(e.to_string())),
            };
            match payload {
                Payload::CustomSection(reader) => {
                    let name = reader.name().to_string();
                    let data = reader.data().to_vec();
                    Some(Ok((name, data)))
                }
                _ => None,
            }
        })
        .collect()
}
