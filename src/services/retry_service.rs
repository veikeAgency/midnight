use chrono::{DateTime, Duration, Utc};

pub fn next_retry_time(attempt_count: i32, now: DateTime<Utc>) -> DateTime<Utc> {
    match attempt_count {
        0 => now + Duration::minutes(1),
        1 => now + Duration::minutes(5),
        _ => now + Duration::minutes(15),
    }
}
