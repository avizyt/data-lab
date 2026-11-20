mod analysis;
mod models;
mod reader;

use analysis::average_sleep_duration;
use analysis::avg_age;
use reader::read_csv;

use crate::analysis::average_sleep_quality_by_occupation;
use crate::analysis::average_stress_by_occupation;
use crate::analysis::count_by_gender;
use crate::analysis::find_short_sleepers;
use crate::analysis::find_unique_value;
use crate::analysis::sleep_quality_by_gender;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "data/Sleep_health_and_lifestyle_dataset.csv";
    let records = read_csv(path)?;

    println!("===== DATASET INFORMATION =====");

    println!("Total records: {}", records.len());
    //
    //     println!("\n===== FIRST RECORD =====");
    //
    //     if let Some(record) = records.first() {
    //         println!("{:#?}", record);
    //     }

    println!("\n===== Basic INSPECTION =====");

    let average_age = avg_age(&records);
    println!("Average Age: {:.2} years", average_age);

    let average_sleep = average_sleep_duration(&records);

    println!("Average sleep duration: {:.2} hours", average_sleep);

    let count_by_gender = count_by_gender(&records);
    println!("Count By Gender");
    for item in count_by_gender {
        println!("{:?} : {:?}", item.0, item.1);
    }

    let short_sleeper = find_short_sleepers(&records);
    println!("Short Sleeps");

    for record in short_sleeper {
        println!("{:?}", record)
    }

    // Occupations
    let occupation = find_unique_value(&records, |record| record.occupation.clone());

    for occu in occupation {
        println!("{occu}");
    }

    // Gender
    let genders = find_unique_value(&records, |record| record.gender.clone());

    for gender in genders {
        println!("{gender}");
    }

    // average sleep quality by ocuupations
    //
    println!("\n========== SLEEP QUALITY BY OCCUPATION ==========");
    let avg_slp_quality = average_sleep_quality_by_occupation(&records);

    for item in avg_slp_quality {
        println!("{:?} -> {:?}", item.0, item.1);
    }
    // average stress by ocuupations
    //
    println!("\n========== STRESS BY OCCUPATION ==========");
    let avg_stress = average_stress_by_occupation(&records);

    for item in avg_stress {
        println!("{:?} -> {:?}", item.0, item.1);
    }
    // Sleep quality by gender
    //
    println!("\n========== Sleep Quality By Gender ==========");
    let sleep_qualities = sleep_quality_by_gender(&records);

    for item in sleep_qualities {
        println!("{:?} -> {:?}", item.0, item.1);
    }

    Ok(())
}
