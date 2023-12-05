use marine_wasm_backend_traits::WValue;

use futures::future::BoxFuture;
use futures::task::waker_ref;
use futures::task::ArcWake;

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;

struct DummyTask();
impl ArcWake for DummyTask {
    fn wake_by_ref(_arc_self: &Arc<Self>) {}
}

pub(crate) fn execute_future_blocking(
    mut future: BoxFuture<anyhow::Result<Vec<WValue>>>,
) -> anyhow::Result<Vec<WValue>> {
    let task = Arc::new(DummyTask());
    let waker = waker_ref(&task);
    let context = &mut Context::from_waker(&waker);

    #[allow(unused_assignments)]
    let mut result: anyhow::Result<Vec<WValue>> = Ok(Vec::default());
    loop {
        match Pin::new(&mut future).poll(context) {
            Poll::Pending => continue,
            Poll::Ready(value) => {
                result = value;
                break;
            }
        }
    }

    result
}
