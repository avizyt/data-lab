use std::collections::{HashMap, HashSet};

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

pub fn find_unique_value<F>(records: &[SleepRecord], extractor: F) -> HashSet<String>
where
    F: Fn(&SleepRecord) -> String,
{
    records.iter().map(extractor).collect()
}

// Average Sleep quality by Occupation

pub fn average_sleep_quality_by_occupation(records: &[SleepRecord]) -> HashMap<String, f64> {
    // let occupations = find_unique_value(records, |record| record.occupation.clone());
    let mut occu_map: HashMap<String, (u64, usize)> = HashMap::new();

    for record in records {
        let stats = occu_map.entry(record.occupation.clone()).or_insert((0, 0));

        stats.0 += record.quality_of_sleep as u64;
        stats.1 += 1;
    }

    occu_map
        .into_iter()
        .map(|(occu, (total, count))| (occu, total as f64 / count as f64))
        .collect()
}

// Average Stress by occupation
pub fn average_stress_by_occupation(records: &[SleepRecord]) -> HashMap<String, f64> {
    // let occupations = find_unique_value(records, |record| record.occupation.clone());
    let mut occu_map: HashMap<String, (u64, usize)> = HashMap::new();

    for record in records {
        let stats = occu_map.entry(record.occupation.clone()).or_insert((0, 0));

        stats.0 += record.stress_level as u64;
        stats.1 += 1;
    }

    occu_map
        .into_iter()
        .map(|(occu, (total, count))| (occu, total as f64 / count as f64))
        .collect()
}
