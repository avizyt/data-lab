use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SleepRecord {
    #[serde(rename = "Person ID")]
    pub person_id: u32,

    #[serde(rename = "Gender")]
    pub gender: String,

    #[serde(rename = "Age")]
    pub age: u8,

    #[serde(rename = "Occupation")]
    pub occupation: String,

    #[serde(rename = "Sleep Duration")]
    pub sleep_duration: f64,

    #[serde(rename = "Quality of Sleep")]
    pub quality_of_sleep: u8,

    #[serde(rename = "Physical Activity Level")]
    pub physical_activity_level: u8,

    #[serde(rename = "Stress Level")]
    pub stress_level: u8,

    #[serde(rename = "BMI Category")]
    pub bmi_category: String,

    #[serde(rename = "Blood Pressure")]
    pub blood_pressure: String,

    #[serde(rename = "Heart Rate")]
    pub heart_rate: u8,

    #[serde(rename = "Daily Steps")]
    pub daily_steps: u32,

    #[serde(rename = "Sleep Disorder")]
    pub sleep_disorder: String,
}
