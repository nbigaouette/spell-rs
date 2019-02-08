use std::{ffi::{CStr, CString}, os::raw::c_char};

use spell::{map::LcsMap, object::LcsObject, tokenize};

#[derive(Debug)]
pub struct _Map {
    map: LcsMap,
}

#[derive(Debug)]
pub struct _Object {
    object: LcsObject,
}

fn _init_env_logger() {
    std::env::var("RUST_LOG")
        .or_else(|_| -> Result<String, ()> {
            let rust_log = "pyspellrs=debug,spell=debug".to_string();
            println!("Environment variable 'RUST_LOG' not set.");
            println!("Setting to: {}", rust_log);
            std::env::set_var("RUST_LOG", &rust_log);
            Ok(rust_log)
        })
        .unwrap();
    let _ = env_logger::try_init();
}

#[no_mangle]
pub unsafe extern "C" fn init_env_logger() {
    _init_env_logger();
}

#[no_mangle]
pub unsafe extern "C" fn new_map() -> *const _Map {
    log::debug!("Creating new map...");
    let map = LcsMap::new();

    let map_ptr = Box::into_raw(Box::new(_Map { map })) as *const _Map;
    log::debug!("Created map: {:?}", map_ptr);

    map_ptr
}

#[no_mangle]
pub unsafe extern "C" fn insert_in_map(map_ptr: *const _Map, line: *const c_char) {
    log::debug!("Inserting line in map...");

    if map_ptr.is_null() {
        log::error!("ERROR: Passed a null pointer for the map");
    } else if line.is_null() {
        log::error!("ERROR: Passed a null pointer for the line");
    } else {
        let c_str = CStr::from_ptr(line);

        match c_str.to_str() {
            Ok(line) => {
                log::debug!("Inserting line into map: {:?}", line);

                // We convert back to an `LcsMap` (from an `_Map` pointer) but we need to prevent
                // it from being dropped when going out of scope.
                let mut map = Box::from_raw(map_ptr as *mut _Map);

                map.map.insert(line);
                println!("map: {:?}", map);

                // Don't drop the map!
                std::mem::forget(map);
            }
            Err(e) => {
                log::error!("Given line is not a valid UTF-8 string {:?}", e);
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_match(map_ptr: *const _Map, line: *const c_char) -> *const _Object {
    log::debug!("Matching line in map...");

    if map_ptr.is_null() {
        log::error!("ERROR: Passed a null pointer for the map");
        std::ptr::null()
    } else if line.is_null() {
        log::error!("ERROR: Passed a null pointer for the line");
        std::ptr::null()
    } else {
        let c_str = CStr::from_ptr(line);

        match c_str.to_str() {
            Ok(line) => {
                log::debug!("Matching line into map: {:?}", line);

                // We convert back to an `LcsMap` (from an `_Map` pointer) but we need to prevent
                // it from being dropped when going out of scope.
                let map = Box::from_raw(map_ptr as *mut _Map);

                let tokens = tokenize(&line, map.map.delimiters.as_slice())
                    .map(|token| token.to_string())
                    .collect();

                let object_ptr = match map.map.get_match(&tokens) {
                    Some(lcs_objects) => {
                        log::info!("Line found in map: {:?}", lcs_objects);

                        Box::into_raw(Box::new(_Object {
                            object: LcsObject {
                                tokens: lcs_objects.tokens.clone(),
                                lines_ids: lcs_objects.lines_ids.clone(),
                            },
                        })) as *const _Object
                    }
                    None => {
                        log::warn!("Line not found in map");
                        std::ptr::null()
                    }
                };

                // Don't drop the map!
                std::mem::forget(map);

                object_ptr
            }
            Err(e) => {
                log::error!("Given line is not a valid UTF-8 string {:?}", e);
                std::ptr::null()
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn free_map(map_ptr: *const _Map) {
    log::debug!("Attempting to drop map {:?}", map_ptr);
    if map_ptr.is_null() {
        log::warn!("Attempted to drop null pointer. Skipping.");
    } else {
        log::debug!("Dropping map...");
        let map = Box::from_raw(map_ptr as *mut _Map);
        std::mem::drop(map);
    }
}

#[no_mangle]
pub unsafe extern "C" fn object_tokens_len(object_ptr: *const _Object) -> u64 {
    if object_ptr.is_null() {
        0
    } else {
        let object = Box::from_raw(object_ptr as *mut _Object);

        let length = object.object.tokens.len();

        std::mem::forget(object);

        length as u64
    }
}

#[no_mangle]
pub unsafe extern "C" fn object_lines_ids_len(object_ptr: *const _Object) -> u64 {
    if object_ptr.is_null() {
        0
    } else {
        let object = Box::from_raw(object_ptr as *mut _Object);

        let length = object.object.lines_ids.len();

        std::mem::forget(object);

        length as u64
    }
}

#[no_mangle]
pub unsafe extern "C" fn object_ith_token(object_ptr: *const _Object, i: u64) -> *const c_char {
    if object_ptr.is_null() {
        std::ptr::null()
    } else {
        let object = Box::from_raw(object_ptr as *mut _Object);

        let i = i as usize;
        let token_str_ptr = if i < object.object.tokens.len() {
            // Safe to unwrap since we are inside a bound check
            let c_str = CString::new(object.object.tokens.get(i).unwrap().clone()).unwrap();
            c_str.into_raw()
        } else {
            std::ptr::null()
        };

        std::mem::forget(object);

        token_str_ptr
    }
}

#[no_mangle]
pub unsafe extern "C" fn object_ith_line_id(object_ptr: *const _Object, i: u64) -> usize {
    if object_ptr.is_null() {
        log::error!("Null pointer passed to object_ith_line_id(), returning 0 instead of crashing.");
        0
    } else {
        let object = Box::from_raw(object_ptr as *mut _Object);

        let i = i as usize;
        let line_id_str_ptr = if i < object.object.lines_ids.len() {
            // Safe to unwrap since we are inside a bound check
            *object.object.lines_ids.get(i).unwrap()
        } else {
            log::error!("Index passed to object_ith_line_id() larger than length, returning 0 instead of crashing.");
            0
        };

        std::mem::forget(object);

        line_id_str_ptr
    }
}

#[no_mangle]
pub unsafe extern "C" fn free_object(object_ptr: *const _Object) {
    log::debug!("Attempting to drop object {:?}", object_ptr);
    if object_ptr.is_null() {
        log::warn!("Attempted to drop null pointer. Skipping.");
    } else {
        log::debug!("Dropping object...");
        let object = Box::from_raw(object_ptr as *mut _Object);
        std::mem::drop(object);
    }
}

// #[no_mangle]
// pub unsafe extern "C" fn with_delimiters() -> *const _Map {
//     // delimiters: Vec<char>
//     // let map = LcsMap::with_delimiters(vec![' ', ',']);

//     Box::into_raw(Box::new(_Map { map })) as *const _Map
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
