use crate::models::SleepRecord;
use std::error::Error;

pub fn read_csv(path: &str) -> Result<Vec<SleepRecord>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    let mut records = Vec::new();

    for result in reader.deserialize() {
        let record: SleepRecord = result?;
        records.push(record);
    }

    Ok(records)
}
