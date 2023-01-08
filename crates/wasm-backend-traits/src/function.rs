use crate::{AsContextMut, CallResult, FuncSig, WasmBackend, WValue};

pub trait Function<WB: WasmBackend>: Send + Sync {
    fn new<F>(store: &mut impl AsContextMut<WB>, sig: FuncSig, func: F) -> Self
    where
        F: for<'c> Fn(&[WValue]) -> Vec<WValue> + Sync + Send + 'static;

    fn new_with_ctx<F>(store: &mut impl AsContextMut<WB>, sig: FuncSig, func: F) -> Self
    where
        F: for<'c> Fn(<WB as WasmBackend>::Caller<'c>, &[WValue]) -> Vec<WValue>
            + Sync
            + Send
            + 'static;

    fn new_typed<Params, Results>(store: &mut impl AsContextMut<WB>, func: impl IntoFunc<WB, Params, Results>) -> Self;

    fn signature<'c>(&self, store: &mut impl AsContextMut<WB>) -> &FuncSig;

    fn call<'c>(
        &self,
        store: &mut impl AsContextMut<WB>, // <- Store or ExportContext. Need to be able to extract wasmtime::StoreContextMut from them. Same for many methods.
        args: &[WValue],
    ) -> CallResult<Vec<WValue>>;
}

pub trait WasmType {
    type Ty;
}


pub trait WasmTypeList {
    type Tuple;
}

impl WasmType for i32 {
    type Ty = i32;
}

impl WasmType for i64 {
    type Ty = i32;
}

impl WasmType for u32 {
    type Ty = i32;
}

impl WasmType for u64 {
    type Ty = i32;
}

pub trait IntoFunc<WB: WasmBackend, Params, Results> {
    fn into_func(self, ctx: &mut impl AsContextMut<WB>) -> <WB as WasmBackend>::Function;
}

macro_rules! impl_into_func {
    ($num:tt $(args:ident)*) => {
        impl<WB: WasmBackend, F, $($args,)* R> IntoFunc<WB, $($args,)* R> for F
        where
            F: Fn($($args,)*) -> R + Send + Sync + 'static,
            R: WasmType,
            $($args: WasmType,)*
        {

        }
    };
}

// almost copypasted from Wasmtime
macro_rules! impl_for_each_function_signature {
    ($mac:ident) => {
        $mac!(0);
        $mac!(1 A1);
        $mac!(2 A1 A2);
        $mac!(3 A1 A2 A3);
        $mac!(4 A1 A2 A3 A4);
        $mac!(5 A1 A2 A3 A4 A5);
        $mac!(6 A1 A2 A3 A4 A5 A6);
        $mac!(7 A1 A2 A3 A4 A5 A6 A7);
        $mac!(8 A1 A2 A3 A4 A5 A6 A7 A8);
        $mac!(9 A1 A2 A3 A4 A5 A6 A7 A8 A9);
        $mac!(10 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10);
        $mac!(11 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11);
        $mac!(12 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12);
        $mac!(13 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13);
        $mac!(14 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 A14);
        $mac!(15 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 A14 A15);
        $mac!(16 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 A14 A15 A16);
    };
}

macro_rules! derive_for_each_function_signature {
    ($mac:ident, $trait_name:ident) => {
        pub trait $trait_name<A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16,>:
        $mac!(0) +
        $mac!(1 A1) +
        $mac!(2 A1 A2)
        $mac!(3 A1 A2 A3) +
        $mac!(4 A1 A2 A3 A4) +
        $mac!(5 A1 A2 A3 A4 A5) +
        $mac!(6 A1 A2 A3 A4 A5 A6) +
        $mac!(7 A1 A2 A3 A4 A5 A6 A7) +
        $mac!(8 A1 A2 A3 A4 A5 A6 A7 A8) +
        $mac!(9 A1 A2 A3 A4 A5 A6 A7 A8 A9) +
        $mac!(10 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10) +
        $mac!(11 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11) +
        $mac!(12 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12) +
        $mac!(13 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13) +
        $mac!(14 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 A14) +
        $mac!(15 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 A14 A15) +
        $mac!(16 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 A14 A15 A16)
    };
}
