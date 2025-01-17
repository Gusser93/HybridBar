#[macro_export]
/// Logs a [HYBRID] [DEBUG] formatted message to stdout.
macro_rules! log {
    ($msg:expr) => {
        if $crate::environment::try_get_var("HYBRID_LOG", "0") == "1" {
            println!("[HYBRID] [DEBUG] {}", $msg)
        }
    };
}

#[macro_export]
/// Executes a bash command and outputs it to `result`.
macro_rules! execute {
    ($($cmd:expr),*) => {
        {
            let mut result;
            $(
            if $cmd.is_empty() {
                // Return a stack-allocated string containing no content.
                drop(heapless::String::<0>::default());
            }

            result = String::from_utf8_lossy(
                &std::process::Command::new("bash")
                .args(["-c", $cmd])
                .output()
                .unwrap()
                .stdout)
                .to_string();

            // Remove the last character as its a new line.
            result.pop();
            )*

            // Could use drop(&$result) but then Clippy would whine.
            format!("{}", result)
        }
    };
}
