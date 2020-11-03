mod client;

mod wstream;
mod list_stream;

pub use client::K8Client;


cfg_if::cfg_if! {
    if #[cfg(feature = "native_tls")] {
        mod config_native;
        use config_native::*;
    } else if #[cfg(feature = "rust_tls")] {
        mod config_rustls;
        use config_rustls::*;
    }
}

use list_stream::*;

pub mod http {
    pub use ::http::header;
    pub use ::http::status;
    pub use ::http::Error;
    pub use hyper::Uri;
}

pub mod prelude {
    pub use hyper::Body;
    pub use hyper::Request;
}

mod executor {

    use futures_util::future::Future;
    use hyper::rt::Executor;

    use fluvio_future::task::spawn;

    pub(crate) struct FluvioHyperExecutor;

    impl<F: Future + Send + 'static> Executor<F> for  FluvioHyperExecutor
    {
        fn execute(&self, fut: F) {
            spawn(async { drop(fut.await) });
        }
    }

}