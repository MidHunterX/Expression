use log2;
use std::env;

/// Initialize Logger
/// Logs to `~/.config/expression/expression.log` by default
/// Logs to stdout if `RUST_LOG` environment variable is set
pub fn init() -> log2::Handle {
    // Log: file + stdout Debug Logs
    let devel_mode = env::var("RUST_LOG").is_ok();
    let builder = if devel_mode {
        log2::stdout().level(env::var("RUST_LOG").unwrap())
    } else {
        let logfile = dirs::state_dir()
            .map(|path| path.join("expression/expression.log"))
            .unwrap();
        log2::open(logfile.to_str().unwrap()).level("info")
    };
    return builder.start();
}
