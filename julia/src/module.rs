//! Unsafe wrapper for Julia modules

use julia_sys::*;

pub struct Symbol {
    ptr: *mut jl_sym_t,
}

pub struct Value {
    ptr: *mut jl_value_t,
}

/// Primitve types in Rust and correspondings
trait Primtive {
    fn make_box(self) -> Value;
}

macro_rules! impl_primivie {
    ($typename:ty, $jl_box:ident) => {
        impl Primtive for $typename {
            fn make_box(self) -> Value {
                Value {
                    ptr: unsafe { $jl_box(self) },
                }
            }
        }
    };
}

impl_primivie!(i8, jl_box_int8);
impl_primivie!(i16, jl_box_int16);
impl_primivie!(i32, jl_box_int32);
impl_primivie!(i64, jl_box_int64);
impl_primivie!(f32, jl_box_float32);
impl_primivie!(f64, jl_box_float64);

fn get_symbol(name: &str) -> Symbol {
    let ptr = unsafe { jl_symbol(name.as_ptr() as *const i8) };
    Symbol { ptr }
}

pub struct RawModule {
    module: *mut _jl_module_t,
}

impl RawModule {
    pub fn base() -> Self {
        RawModule {
            module: unsafe { jl_base_module },
        }
    }

    pub fn get_global(self, name: &str) -> Value {
        let ptr = unsafe { jl_get_global(self.module, get_symbol(name).ptr) };
        Value { ptr }
    }

    pub fn get_function(self, name: &str) -> Function {
        let value = self.get_global(name);
        Function { ptr: value.ptr }
    }
}

pub struct Function {
    ptr: *mut jl_function_t,
}

impl Function {
    pub fn call(&self, args: &[&mut Value]) -> Value {
        let mut argv: Vec<*mut jl_value_t> = args.iter().map(|a| a.ptr).collect();
        let ptr = unsafe { jl_call(self.ptr, argv.as_mut_ptr(), argv.len() as i32) };
        Value { ptr }
    }

    pub fn call0(&self) -> Value {
        let ptr = unsafe { jl_call0(self.ptr) };
        Value { ptr }
    }

    pub fn call1(&self, arg: &mut Value) -> Value {
        let ptr = unsafe { jl_call1(self.ptr, arg.ptr) };
        Value { ptr }
    }

    pub fn call2(&self, arg1: &mut Value, arg2: &mut Value) -> Value {
        let ptr = unsafe { jl_call2(self.ptr, arg1.ptr, arg2.ptr) };
        Value { ptr }
    }

    pub fn call3(&self, arg1: &mut Value, arg2: &mut Value, arg3: &mut Value) -> Value {
        let ptr = unsafe { jl_call3(self.ptr, arg1.ptr, arg2.ptr, arg3.ptr) };
        Value { ptr }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_string() {
        unsafe {
            jl_eval_string("print(sqrt(2.0))".as_ptr() as *const i8);
        }
    }
}
