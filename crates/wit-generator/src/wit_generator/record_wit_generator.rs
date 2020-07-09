use super::WITGenerator;
use super::Interfaces;

use fluence_sdk_wit::AstRecordItem;

impl WITGenerator for AstRecordItem {
    fn generate_wit<'a>(&'a self, _interfaces: &mut Interfaces<'a>) {
        unimplemented!()
    }
}
