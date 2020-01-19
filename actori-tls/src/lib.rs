//! SSL Services
#![deny(rust_2018_idioms, warnings)]
#![allow(clippy::type_complexity)]

use std::sync::atomic::{AtomicUsize, Ordering};

use actori_utils::counter::Counter;

#[cfg(feature = "openssl")]
pub mod openssl;

#[cfg(feature = "rustls")]
pub mod rustls;

#[cfg(feature = "nativetls")]
pub mod nativetls;

/// Sets the maximum per-worker concurrent ssl connection establish process.
///
/// All listeners will stop accepting connections when this limit is
/// reached. It can be used to limit the global SSL CPU usage.
///
/// By default max connections is set to a 256.
pub fn max_concurrent_ssl_connect(num: usize) {
    MAX_CONN.store(num, Ordering::Relaxed);
}

pub(crate) static MAX_CONN: AtomicUsize = AtomicUsize::new(256);

thread_local! {
    static MAX_CONN_COUNTER: Counter = Counter::new(MAX_CONN.load(Ordering::Relaxed));
}

/// Ssl error combinded with service error.
#[derive(Debug)]
pub enum SslError<E1, E2> {
    Ssl(E1),
    Service(E2),
}
