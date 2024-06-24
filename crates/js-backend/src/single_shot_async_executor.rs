/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
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
