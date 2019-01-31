use julia::*;

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
