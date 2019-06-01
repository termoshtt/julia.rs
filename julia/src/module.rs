use julia_sys::*;

unsafe fn get_symbol(name: &str) -> *mut jl_sym_t {
    jl_symbol(name.as_ptr() as *const i8)
}

pub struct RawModule {
    module: *mut _jl_module_t,
}

impl RawModule {
    pub unsafe fn base() -> Self {
        RawModule {
            module: jl_base_module,
        }
    }

    pub unsafe fn get_global(self, name: &str) -> *mut jl_value_t {
        jl_get_global(self.module, get_symbol(name))
    }
}
