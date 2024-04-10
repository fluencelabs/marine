/*
 * Copyright 2024 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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
