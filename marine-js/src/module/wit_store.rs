pub(crate) struct WITStore {}

impl it_memory_traits::Store for WITStore {
    type ActualStore<'c> = ();
}