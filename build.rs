#[cfg(all(feature = "driver", not(any(feature = "rustls", feature = "native"))))]
compile_error!(
    "You have the `driver` feature enabled: \
    either the `rustls` or `native` feature must be
    selected to let Songbird's driver use websockets.\n\
    - `rustls` uses Rustls, a pure Rust TLS-implemenation.\n\
    - `native` uses SChannel on Windows, Secure Transport on macOS, \
    and OpenSSL on other platforms.\n\
    If you are unsure, go with `rustls`."
);

fn main() {}
