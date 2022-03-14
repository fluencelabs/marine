//use wasmer_core::types::NativeWasmType;
use crate::WType;
use crate::WValue;


/// Represents a native wasm type.
pub unsafe trait NativeWasmType: Copy + Into<WValue>
    where
        Self: Sized,
{
    /// Type for this `NativeWasmType`.
    const TYPE: WType;

    /// Convert from u64 bites to self.
    fn from_binary(bits: u64) -> Self;

    /// Convert self to u64 binary representation.
    fn to_binary(self) -> u64;
}

unsafe impl NativeWasmType for i32 {
    const TYPE: WType = WType::I32;

    fn from_binary(bits: u64) -> Self {
        bits as _
    }

    fn to_binary(self) -> u64 {
        self as _
    }
}

unsafe impl NativeWasmType for i64 {
    const TYPE: WType = WType::I64;

    fn from_binary(bits: u64) -> Self {
        bits as _
    }

    fn to_binary(self) -> u64 {
        self as _
    }
}

unsafe impl NativeWasmType for f32 {
    const TYPE: WType = WType::F32;

    fn from_binary(bits: u64) -> Self {
        f32::from_bits(bits as u32)
    }

    fn to_binary(self) -> u64 {
        self.to_bits() as _
    }
}

unsafe impl NativeWasmType for f64 {
    const TYPE: WType = WType::F64;

    fn from_binary(bits: u64) -> Self {
        f64::from_bits(bits)
    }

    fn to_binary(self) -> u64 {
        self.to_bits()
    }
}


pub unsafe trait WasmExternType: Copy
    where
        Self: Sized,
{
    /// Native wasm type for this `WasmExternType`.
    type Native: NativeWasmType;

    /// Convert from given `Native` type to self.
    fn from_native(native: Self::Native) -> Self;

    /// Convert self to `Native` type.
    fn to_native(self) -> Self::Native;
}

/// Represents a list of WebAssembly values.
pub trait WasmTypeList {
    /// CStruct type.
    type CStruct;

    /// Array of return values.
    type RetArray: AsMut<[u64]>;

    const NTypes: usize;
    const Types: [WType];

    /// Construct `Self` based on an array of returned values.
    fn from_ret_array(array: Self::RetArray) -> Self;

    /// Generates an empty array that will hold the returned values of
    /// the WebAssembly function.
    fn empty_ret_array() -> Self::RetArray;

    /// Transforms C values into Rust values.
    fn from_c_struct(c_struct: Self::CStruct) -> Self;

    /// Transforms Rust values into C values.
    fn into_c_struct(self) -> Self::CStruct;

    /// Get types of the current values.
    fn types() -> &'static [WType];

    /*
    /// This method is used to distribute the values onto a function,
    /// e.g. `(1, 2).call(func, …)`. This form is unlikely to be used
    /// directly in the code, see the `Func::call` implementation.
    unsafe fn call<Rets>(
        self,
        f: std::ptr::NonNull<wasmer_core::vm::Func>,
        wasm: wasmer_core::typed_func::Wasm,
        ctx: *mut wasmer_core::vm::Ctx,
    ) -> Result<Rets, wasmer_core::error::RuntimeError>
        where
            Rets: WasmTypeList;*/
}


macro_rules! impl_traits {
    ( [$repr:ident] $struct_name:ident, $( $x:ident ),* ) => {
        /// Struct for typed funcs.
        #[repr($repr)]
        pub struct $struct_name< $( $x ),* > ( $( <$x as WasmExternType>::Native ),* )
        where
            $( $x: WasmExternType ),*;

        #[allow(unused_parens)]
        impl< $( $x ),* > WasmTypeList for ( $( $x ),* )
        where
            $( $x: WasmExternType ),*
        {
            type CStruct = $struct_name<$( $x ),*>;

            type RetArray = [u64; count_idents!( $( $x ),* )];

            const NTypes: usize = count_idents!( $( $x ),* );
            const Types: [WType] = [$( $x::Native::TYPE ),*];

            fn from_ret_array(array: Self::RetArray) -> Self {
                #[allow(non_snake_case)]
                let [ $( $x ),* ] = array;

                ( $( WasmExternType::from_native(NativeWasmType::from_binary($x)) ),* )
            }

            fn empty_ret_array() -> Self::RetArray {
                [0; count_idents!( $( $x ),* )]
            }

            fn from_c_struct(c_struct: Self::CStruct) -> Self {
                #[allow(non_snake_case)]
                let $struct_name ( $( $x ),* ) = c_struct;

                ( $( WasmExternType::from_native($x) ),* )
            }

            #[allow(unused_parens, non_snake_case)]
            fn into_c_struct(self) -> Self::CStruct {
                let ( $( $x ),* ) = self;

                $struct_name ( $( WasmExternType::to_native($x) ),* )
            }

            fn types() -> &'static [WType] {
                &Self::Types
            }

            /*
            #[allow(unused_parens, non_snake_case)]
            unsafe fn call<Rets>(
                self,
                f: NonNull<vm::Func>,
                wasm: Wasm,
                ctx: *mut vm::Ctx,
            ) -> Result<Rets, RuntimeError>
            where
                Rets: WasmTypeList
            {
                let ( $( $x ),* ) = self;
                let args = [ $( $x.to_native().to_binary()),* ];
                let mut rets = Rets::empty_ret_array();
                let mut error_out = None;

                if (wasm.invoke)(
                    wasm.trampoline,
                    ctx,
                    f,
                    args.as_ptr(),
                    rets.as_mut().as_mut_ptr(),
                    &mut error_out,
                    wasm.invoke_env
                ) {
                    Ok(Rets::from_ret_array(rets))
                } else {
                    Err(error_out.map_or_else(|| RuntimeError::InvokeError(InvokeError::FailedWithNoError), Into::into))
                }
            }*/
        }
    };
}

macro_rules! count_idents {
    ( $($idents:ident),* ) => {{
        #[allow(dead_code, non_camel_case_types)]
        enum Idents { $($idents,)* __CountIdentsLast }
        const COUNT: usize = Idents::__CountIdentsLast as usize;
        COUNT
    }};
}

macro_rules! wasm_extern_type {
    ($type:ty => $native_type:ty) => {
        unsafe impl WasmExternType for $type {
            type Native = $native_type;

            fn from_native(native: Self::Native) -> Self {
                native as _
            }

            fn to_native(self) -> Self::Native {
                self as _
            }
        }
    };
}

wasm_extern_type!(i8 => i32);
wasm_extern_type!(u8 => i32);
wasm_extern_type!(i16 => i32);
wasm_extern_type!(u16 => i32);
wasm_extern_type!(i32 => i32);
wasm_extern_type!(u32 => i32);
wasm_extern_type!(i64 => i64);
wasm_extern_type!(u64 => i64);
wasm_extern_type!(f32 => f32);
wasm_extern_type!(f64 => f64);

impl_traits!([C] S0,);
impl_traits!([transparent] S1, A);
impl_traits!([C] S2, A, B);
impl_traits!([C] S3, A, B, C);
impl_traits!([C] S4, A, B, C, D);
impl_traits!([C] S5, A, B, C, D, E);
impl_traits!([C] S6, A, B, C, D, E, F);
impl_traits!([C] S7, A, B, C, D, E, F, G);
impl_traits!([C] S8, A, B, C, D, E, F, G, H);
impl_traits!([C] S9, A, B, C, D, E, F, G, H, I);
impl_traits!([C] S10, A, B, C, D, E, F, G, H, I, J);
impl_traits!([C] S11, A, B, C, D, E, F, G, H, I, J, K);
impl_traits!([C] S12, A, B, C, D, E, F, G, H, I, J, K, L);
impl_traits!([C] S13, A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_traits!([C] S14, A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_traits!([C] S15, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_traits!([C] S16, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
impl_traits!([C] S17, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
impl_traits!([C] S18, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
impl_traits!([C] S19, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
impl_traits!([C] S20, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
impl_traits!([C] S21, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
impl_traits!([C] S22, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
impl_traits!([C] S23, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
impl_traits!([C] S24, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
impl_traits!([C] S25, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
impl_traits!([C] S26, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

