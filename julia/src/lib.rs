use julia_sys::*;

unsafe fn jl_astaggedvalue(ptr: *mut jl_value_t) -> *mut jl_taggedvalue_t {
    let p = ptr as *mut jl_taggedvalue_t;
    p.offset(-1)
}

unsafe fn jl_typeof(ptr: *mut jl_value_t) -> *mut jl_datatype_t {
    let tagged = jl_astaggedvalue(ptr);
    let t = (*tagged).header & !15_usize;
    t as *mut jl_datatype_t
}

unsafe fn jl_is_array(ptr: *mut jl_value_t) -> bool {
    let ty = jl_typeof(ptr);
    (*ty).name == jl_array_typename
}

#[no_mangle]
pub extern "C" fn make_twice_array(ptr: *mut jl_value_t) {
    if !unsafe { jl_is_array(ptr) } {
        panic!("This is not an array")
    }
    let array = ptr as *mut jl_array_t;
    let data = unsafe {
        let n = (*array).length;
        let data = (*array).data as *mut f32;
        std::slice::from_raw_parts_mut(data, n)
    };

    for v in data {
        *v *= 2.0;
    }
}
