use std::borrow::Cow;
use walrus::{CustomSection, IdsToIndices};

pub const WIT_SECTION_NAME: &str = "interface-types";

#[derive(Debug, Clone)]
pub(crate) struct WITCustom(pub Vec<u8>);

impl CustomSection for WITCustom {
    fn name(&self) -> &str {
        WIT_SECTION_NAME
    }

    fn data(&self, _ids_to_indices: &IdsToIndices) -> Cow<[u8]> {
        Cow::Borrowed(&self.0)
    }
}
