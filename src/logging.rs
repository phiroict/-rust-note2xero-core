use env_logger::Builder;
use uuid::Uuid;
use log::{trace,LevelFilter};
use chrono::Local;
use std::io::Write;

pub fn initialize_logging() {
    let request_id = Uuid::new_v4();
    Builder::new()
        .format(move |buf, record| {
            writeln!(buf,
                     "{} [{}] '{}' - {}",
                     Local::now().format("%Y-%m-%dT%H:%M:%S"),
                     record.level(),
                     &request_id,
                     record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
    trace!("Logging system set up.")
}