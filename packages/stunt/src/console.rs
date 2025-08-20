//! Access the browser console.

/// Log to the console using standard rust formatting.
///
/// ## Example
/// ```rust,no_run
/// log!("this is a log");
/// ```
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        ::stunt::console::_log(format!($($arg)*));
    }
}

/// console log.
pub fn _log(log: String) {
    web_sys::console::log_1(&log.into());
}

pub use log;

/// Warn to the console using standard rust formatting.
///
/// ## Example
/// ```rust,no_run
/// warn!("this is a warning");
/// ```
#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {
        ::stunt::console::_warn(format!($($arg)*));
    }
}

/// console warn.
pub fn _warn(warning: String) {
    web_sys::console::warn_1(&warning.into());
}

pub use warning;

/// Error to the console using standard rust formatting.
///
/// ## Example
/// ```rust,no_run
/// error!("this is an error");
/// ```
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        ::stunt::console::_error(format!($($arg)*));
    }
}

/// console error.
pub fn _error(error: String) {
    web_sys::console::error_1(&error.into());
}

pub use error;


