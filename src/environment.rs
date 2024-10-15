pub fn parse() {
    let env_file: String =
        std::fs::read_to_string(".env").expect("Failed to read .env file, make sure it exists");

    for line in env_file.lines() {
        // Everything after a '#' is considered a comment and is skipped.
        if line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.replace('\"', "").split_once('=') {
            // set_var will, apparently, be marked as unsafe in the future.
            // Estimation is the 2024 edition of rust.
            // In our case, using this function is totally safe under our context unless we
            // do something else involving enviroment variables before this function is called.
            // See: https://github.com/rust-lang/rust/issues/27970 and https://github.com/rust-lang/rust/pull/116888
            #[allow(unused_unsafe)]
            unsafe {
                std::env::set_var(key, value);
            }
        }
    }
}
