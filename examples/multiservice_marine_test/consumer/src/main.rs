use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;

module_manifest!();

pub fn main() {}

#[marine]
pub struct Data {
    pub name: String,
}

#[marine]
pub fn consume(data: Data) -> String {
    data.name
}
