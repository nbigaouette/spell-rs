use spell::map::LcsMap;

#[derive(Debug)]
pub struct _Map {
    map: LcsMap,
}

fn _init_env_logger() {
    std::env::var("RUST_LOG")
        .or_else(|_| -> Result<String, ()> {
            let rust_log =
                "pyspellrs=debug,spell=debug"
                    .to_string();
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