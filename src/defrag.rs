use crate::raw;

/// .
///
/// # Panics
///
/// Panics if `RedisModule_DefragAlloc` is missing in redismodule.h
pub fn defrag_alloc(
    func: *const ::std::os::raw::c_void,
) {
        unsafe {
            raw::RedisModule_DefragAlloc
                .expect("RedisModule_DefragAlloc is not available.")(std::ptr::null_mut(), func as *mut ::std::os::raw::c_void);
        }
}

