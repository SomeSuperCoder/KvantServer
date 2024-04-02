use chrono::Utc;
use chrono::DateTime;

pub fn get_time() -> String {
    let current_time: DateTime<Utc> = Utc::now();
    current_time.format("%Y-%m-%d %H:%M:%S").to_string()
}
