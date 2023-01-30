/*
 * Copyright 2023 Fluence Labs Limited
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

use crate::{StoreState, WasmerBackend, WasmerContext, WasmerContextMut};

use marine_wasm_backend_traits::*;

use wasmer::{AsStoreRef, AsStoreMut, FunctionEnv, FunctionEnvMut};

pub struct WasmerCaller<'c> {
    pub(crate) inner: FunctionEnvMut<'c, ()>,
    pub(crate) env: FunctionEnv<StoreState>,
}

impl Caller<WasmerBackend> for WasmerCaller<'_> {
    fn memory(&mut self, _memory_index: u32) -> Option<<WasmerBackend as WasmBackend>::Memory> {
        self.env
            .as_mut(&mut self.inner)
            .current_memory
            .clone()
            .map(Into::into)
    }
}

impl AsContext<WasmerBackend> for WasmerCaller<'_> {
    fn as_context(&self) -> <WasmerBackend as WasmBackend>::Context<'_> {
        WasmerContext {
            inner: self.inner.as_store_ref(),
            env: self.env.clone(),
        }
    }
}

impl AsContextMut<WasmerBackend> for WasmerCaller<'_> {
    fn as_context_mut(&mut self) -> <WasmerBackend as WasmBackend>::ContextMut<'_> {
        WasmerContextMut {
            inner: self.inner.as_store_mut(),
            env: self.env.clone(),
        }
    }
}

macro_rules! impl_func_getter {
    ($args:ty, $rets:ty) => {
        impl FuncGetter<WasmerBackend, $args, $rets> for WasmerCaller<'_> {
            fn get_func(
                &mut self,
                _name: &str,
            ) -> ResolveResult<FuncFromCaller<WasmerBackend, $args, $rets>> {
                todo!()
            }
        }
    };
}

impl_func_getter!((), ());
impl_func_getter!((), i32);
impl_func_getter!(i32, ());
impl_func_getter!(i32, i32);
impl_func_getter!((i32, i32), ());
impl_func_getter!((i32, i32), i32);
