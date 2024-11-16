pub use tracing::{debug, error, info, instrument, warn};
use tracing_subscriber::fmt;
pub struct Tracing {}

impl Tracing {
    pub fn init() {
        let format = fmt::format()
            .compact()
            .with_file(true)
            .with_target(false)
            .without_time();

        tracing_subscriber::fmt().event_format(format).init();
    }
}
