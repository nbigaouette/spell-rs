
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


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
