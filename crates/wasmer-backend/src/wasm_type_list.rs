use std::ptr::NonNull;
use wasmer_core::error::RuntimeError;
use wasmer_core::typed_func::{Wasm};
use wasmer_core::types::{NativeWasmType, Type};
use wasmer_core::vm::Ctx;
use marine_wasm_backend_traits::WType;
use marine_wasm_backend_traits::WValue;
use marine_wasm_backend_traits::Tuple;
use marine_wasm_backend_traits::WasmTypeList;


struct Helper<T: WasmTypeList> (T);



impl<T: WasmTypeList> wasmer_core::typed_func::WasmTypeList for Helper<T> {
    type CStruct = T::CStruct;
    type RetArray = T::RetArray;

    fn from_ret_array(array: Self::RetArray) -> Self {
        Self(T::from_ret_array(array))
    }

    fn empty_ret_array() -> Self::RetArray {
        T::empty_ret_array()
    }

    fn from_c_struct(c_struct: Self::CStruct) -> Self {
        Self(T::from_c_struct(c_struct))
    }

    fn into_c_struct(self) -> Self::CStruct {
        self.into_c_struct()
    }

    fn types() -> &'static [Type] {
        T::types() /// how to convert?
    }

    unsafe fn call<Rets>(self,
                         f: NonNull<wasmer_core::prelude::vm::Func>,
                         wasm: Wasm,
                         ctx: *mut Ctx
    ) -> Result<Rets, RuntimeError> where Rets: wasmer_core::typed_func::WasmTypeList {
        // how to implement?
    }
}
/*
const fn gen_types(wtypes: &'static [WType]) -> &'static [Type] {
    let types = [Type::I32; wtypes.len()];
    &types
}

macro_rules! impl_traits {
    ( [$repr:ident] $struct_name:ident, $( $x:ident ),* ) => {
        /// Struct for typed funcs.
        #[repr($repr)]
        pub struct $struct_name< $( $x ),* > ( $( <$x as WasmExternType>::Native ),* )
        where
            $( $x: WasmExternType ),*;

        #[allow(unused_parens)]
        impl< $( $x ),* > wasmer_core::typed_func::WasmTypeList for Helper<( $( $x ),* )>
        where
            $( $x: WasmExternType ),*
        {
            type CStruct = $struct_name<$( $x ),*>;

            type RetArray = [u64; count_idents!( $( $x ),* )];

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

            fn types() -> &'static [Type] {
                &[$( $x::Native::WTYPE ),*]
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
/*
        #[allow(unused_parens)]
        impl< $( $x, )* Rets, Trap, FN > HostFunction<ExplicitVmCtx, ( $( $x ),* ), Rets> for FN
        where
            $( $x: WasmExternType, )*
            Rets: WasmTypeList,
            Trap: TrapEarly<Rets>,
            FN: Fn(&mut vm::Ctx $( , $x )*) -> Trap + 'static + Send,
        {
            #[allow(non_snake_case)]
            fn to_raw(self) -> (NonNull<vm::Func>, Option<NonNull<vm::FuncEnv>>) {
                // The `wrap` function is a wrapper around the
                // imported function. It manages the argument passed
                // to the imported function (in this case, the
                // `vmctx` along with the regular WebAssembly
                // arguments), and it manages the trapping.
                //
                // It is also required for the LLVM backend to be
                // able to unwind through this function.
                extern fn wrap<$( $x, )* Rets, Trap, FN>(
                    vmctx: &vm::Ctx $( , $x: <$x as WasmExternType>::Native )*
                ) -> Rets::CStruct
                where
                    $( $x: WasmExternType, )*
                    Rets: WasmTypeList,
                    Trap: TrapEarly<Rets>,
                    FN: Fn(&mut vm::Ctx, $( $x, )*) -> Trap,
                {
                    // Get the pointer to this `wrap` function.
                    let self_pointer = wrap::<$( $x, )* Rets, Trap, FN> as *const vm::Func;

                    // Get the collection of imported functions.
                    let vm_imported_functions = unsafe { &(*vmctx.import_backing).vm_functions };

                    // Retrieve the `vm::FuncCtx`.
                    let mut func_ctx: NonNull<vm::FuncCtx> = vm_imported_functions
                        .iter()
                        .find_map(|(_, imported_func)| {
                            if imported_func.func == self_pointer {
                                Some(imported_func.func_ctx)
                            } else {
                                None
                            }
                        })
                        .expect("Import backing is not well-formed, cannot find `func_ctx`.");
                    let func_ctx = unsafe { func_ctx.as_mut() };

                    // Extract `vm::Ctx` from `vm::FuncCtx`. The
                    // pointer is always non-null.
                    let vmctx = unsafe { func_ctx.vmctx.as_mut() };

                    // Extract `vm::FuncEnv` from `vm::FuncCtx`.
                    let func_env = func_ctx.func_env;

                    let func: &FN = match func_env {
                        // The imported function is a regular
                        // function, a closure without a captured
                        // environment, or a closure with a captured
                        // environment.
                        Some(func_env) => unsafe {
                            let func: NonNull<FN> = func_env.cast();

                            &*func.as_ptr()
                        },

                        // This branch is supposed to be unreachable.
                        None => unreachable!()
                    };

                    // Catch unwind in case of errors.
                    let err = match panic::catch_unwind(
                        panic::AssertUnwindSafe(
                            || {
                                func(vmctx $( , WasmExternType::from_native($x) )* ).report()
                                //   ^^^^^ The imported function
                                //         expects `vm::Ctx` as first
                                //         argument; provide it.
                            }
                        )
                    ) {
                        Ok(Ok(returns)) => return returns.into_c_struct(),
                        Ok(Err(err)) => {
                            let b: Box<_> = err.into();
                            RuntimeError::User(b as Box<dyn Any + Send>)
                        },
                        // TODO(blocking): this line is wrong!
                        Err(err) => RuntimeError::User(err),
                    };

                    // At this point, there is an error that needs to
                    // be trapped.
                    unsafe {
                        (&*vmctx.module).runnable_module.do_early_trap(err)
                    }
                }

                // Extract the captured environment of the imported
                // function if any.
                let func_env: Option<NonNull<vm::FuncEnv>> =
                    // `FN` is a function pointer, or a closure
                    // _without_ a captured environment.
                    if mem::size_of::<Self>() == 0 {
                        NonNull::new(&self as *const _ as *mut vm::FuncEnv)
                    }
                    // `FN` is a closure _with_ a captured
                    // environment.
                    else {
                        NonNull::new(Box::into_raw(Box::new(self))).map(NonNull::cast)
                    };

                (
                    NonNull::new(wrap::<$( $x, )* Rets, Trap, Self> as *mut vm::Func).unwrap(),
                    func_env
                )
            }
        }

        #[allow(unused_parens)]
        impl< $( $x, )* Rets, Trap, FN > HostFunction<ImplicitVmCtx, ( $( $x ),* ), Rets> for FN
        where
            $( $x: WasmExternType, )*
            Rets: WasmTypeList,
            Trap: TrapEarly<Rets>,
            FN: Fn($( $x, )*) -> Trap + 'static + Send,
        {
            #[allow(non_snake_case)]
            fn to_raw(self) -> (NonNull<vm::Func>, Option<NonNull<vm::FuncEnv>>) {
                // The `wrap` function is a wrapper around the
                // imported function. It manages the argument passed
                // to the imported function (in this case, only the
                // regular WebAssembly arguments), and it manages the
                // trapping.
                //
                // It is also required for the LLVM backend to be
                // able to unwind through this function.
                extern fn wrap<$( $x, )* Rets, Trap, FN>(
                    vmctx: &vm::Ctx $( , $x: <$x as WasmExternType>::Native )*
                ) -> Rets::CStruct
                where
                    $( $x: WasmExternType, )*
                    Rets: WasmTypeList,
                    Trap: TrapEarly<Rets>,
                    FN: Fn($( $x, )*) -> Trap,
                {
                    // Get the pointer to this `wrap` function.
                    let self_pointer = wrap::<$( $x, )* Rets, Trap, FN> as *const vm::Func;

                    // Get the collection of imported functions.
                    let vm_imported_functions = unsafe { &(*vmctx.import_backing).vm_functions };

                    // Retrieve the `vm::FuncCtx`.
                    let mut func_ctx: NonNull<vm::FuncCtx> = vm_imported_functions
                        .iter()
                        .find_map(|(_, imported_func)| {
                            if imported_func.func == self_pointer {
                                Some(imported_func.func_ctx)
                            } else {
                                None
                            }
                        })
                        .expect("Import backing is not well-formed, cannot find `func_ctx`.");
                    let func_ctx = unsafe { func_ctx.as_mut() };

                    // Extract `vm::Ctx` from `vm::FuncCtx`. The
                    // pointer is always non-null.
                    let vmctx = unsafe { func_ctx.vmctx.as_mut() };

                    // Extract `vm::FuncEnv` from `vm::FuncCtx`.
                    let func_env = func_ctx.func_env;

                    let func: &FN = match func_env {
                        // The imported function is a regular
                        // function, a closure without a captured
                        // environment, or a closure with a captured
                        // environment.
                        Some(func_env) => unsafe {
                            let func: NonNull<FN> = func_env.cast();

                            &*func.as_ptr()
                        },

                        // This branch is supposed to be unreachable.
                        None => unreachable!()
                    };

                    // Catch unwind in case of errors.
                    let err = match panic::catch_unwind(
                        panic::AssertUnwindSafe(
                            || {
                                func($( WasmExternType::from_native($x), )* ).report()
                            }
                        )
                    ) {
                        Ok(Ok(returns)) => return returns.into_c_struct(),
                        Ok(Err(err)) => {
                            let b: Box<_> = err.into();
                            RuntimeError::User(b as Box<dyn Any + Send>)
                        },
                        // TODO(blocking): this line is wrong!
                        Err(err) => RuntimeError::User(err),
                    };

                    // At this point, there is an error that needs to
                    // be trapped.
                    unsafe {
                        (&*vmctx.module).runnable_module.do_early_trap(err)
                    }
                }

                // Extract the captured environment of the imported
                // function if any.
                let func_env: Option<NonNull<vm::FuncEnv>> =
                    // `FN` is a function pointer, or a closure
                    // _without_ a captured environment.
                    if mem::size_of::<Self>() == 0 {
                        NonNull::new(&self as *const _ as *mut vm::FuncEnv)
                    }
                    // `FN` is a closure _with_ a captured
                    // environment.
                    else {
                        NonNull::new(Box::into_raw(Box::new(self))).map(NonNull::cast)
                    };

                (
                    NonNull::new(wrap::<$( $x, )* Rets, Trap, Self> as *mut vm::Func).unwrap(),
                    func_env
                )
            }
        }

        #[allow(unused_parens)]
        impl<'a $( , $x )*, Rets> Func<'a, ( $( $x ),* ), Rets, Wasm>
        where
            $( $x: WasmExternType, )*
            Rets: WasmTypeList,
        {
            /// Call the typed func and return results.
            #[allow(non_snake_case, clippy::too_many_arguments)]
            pub fn call(&self, $( $x: $x, )* ) -> Result<Rets, RuntimeError> {
                #[allow(unused_parens)]
                unsafe {
                    <( $( $x ),* ) as WasmTypeList>::call(
                        ( $( $x ),* ),
                        self.func,
                        self.inner,
                        self.vmctx
                    )
                }
            }
        }*/
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

*/