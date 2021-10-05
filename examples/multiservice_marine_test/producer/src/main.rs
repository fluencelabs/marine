use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;

module_manifest!();

pub fn main() {}

#[marine]
pub struct Data {
    pub name: String,
}

#[marine]
pub struct Input {
    pub first_name: String,
    pub last_name: String,
}

#[marine]
pub fn produce(data: Input) -> Data {
    Data {
        name: format!("{} {}", data.first_name, data.last_name),
    }
}
