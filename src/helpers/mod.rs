use chrono::prelude::*;

pub fn current_time () -> String {
    let dt = Utc::now();
    dt.to_rfc3339()
}
