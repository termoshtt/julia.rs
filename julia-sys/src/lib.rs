//! Bare wrapper for Julia C API

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
include!("julia.rs");

pub unsafe fn jl_astaggedvalue(ptr: *mut jl_value_t) -> *mut jl_taggedvalue_t {
    let p = ptr as *mut jl_taggedvalue_t;
    p.offset(-1)
}

pub unsafe fn jl_typeof(ptr: *mut jl_value_t) -> *mut jl_datatype_t {
    let tagged = jl_astaggedvalue(ptr);
    let t = (*tagged).header & !15_usize;
    t as *mut jl_datatype_t
}

pub unsafe fn jl_is_array(ptr: *mut jl_value_t) -> bool {
    let ty = jl_typeof(ptr);
    (*ty).name == jl_array_typename
}
