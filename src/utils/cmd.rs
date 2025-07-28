use std::io;
use std::process::{Command, Output};

/// Execute a shell command and return the result
/// # Arguments
/// * `cmd` - The command string to execute
/// # Returns
/// * `Result<Output, io::Error>` - The command output or an error
/// # Example
/// ```
/// use expression::utils::cmd::execute;
///
/// let result = execute("ls -la");
/// match result {
///     Ok(output) => {
///         println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
///         println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
///         println!("status: {}", output.status);
///     }
///     Err(e) => eprintln!("Error executing command: {}", e),
/// }
/// ```
pub fn execute(cmd: &str) -> Result<Output, io::Error> {
    let (shell, flag) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("bash", "-c")
    };

    Command::new(shell).arg(flag).arg(cmd).output()
}
