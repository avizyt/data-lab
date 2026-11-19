use std::collections::HashMap;

use crate::models::SleepRecord;

pub fn avg_age(records: &[SleepRecord]) -> f64 {
    let total_age: u32 = records.iter().map(|record| record.age as u32).sum();

    let avg_age: f64 = if records.is_empty() {
        0.0
    } else {
        total_age as f64 / records.len() as f64
    };

    avg_age as f64
}

// Average Sleep duration
pub fn average_sleep_duration(records: &[SleepRecord]) -> f64 {
    let total_sleep_duration: f64 = records.iter().map(|record| record.sleep_duration).sum();

    let avg_sleep_time = if records.is_empty() {
        0.0 as f64
    } else {
        total_sleep_duration as f64 / records.len() as f64
    };

    avg_sleep_time as f64
}

// Count record by gender
pub fn count_by_gender(records: &[SleepRecord]) -> HashMap<String, usize> {
    let mut gender_map: HashMap<String, usize> = HashMap::new();

    for record in records {
        let count = gender_map.entry(record.gender.clone()).or_insert(0);

        *count += 1;
    }

    gender_map
}

// Finf short sleepers or Sleep lessthan 6 hour

pub fn find_short_sleepers(records: &[SleepRecord]) -> Vec<&SleepRecord> {
    let mut short_sleepers: Vec<&SleepRecord> = Vec::new();

    for record in records {
        if record.sleep_duration < 6.0 {
            short_sleepers.push(record);
        }
    }

    short_sleepers
}
