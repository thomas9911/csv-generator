use std::fs::File;

use csv::{WriterBuilder, Writer};
use fake::{Dummy, Faker};
use fake::faker::name::en::Name;
use chrono::{DateTime, Utc};
use fake::Fake;
use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeSeq;
use std::io::Write;

#[derive(Debug, Dummy, serde::Serialize)]
pub struct Order {
    order_id: usize,
    #[dummy(faker = "Name()")]
    customer: String,
    paid: bool,
    #[dummy(faker = "fake::faker::chrono::en::DateTime()")]
    created_at: DateTime<Utc>,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("Hello, world!");
    for amount in [1000, 10000, 100000] {
        // csv_builder(b',', &format!("data/US-{amount}.csv"), amount)?;
        // csv_builder(b';', &format!("data/EU-{amount}.csv"), amount)?;
        // csv_builder(b'\t', &format!("data/EU-{amount}.tsv"), amount)?;
        json_builder(&format!("data/{amount}.json"), amount)?;
    }


    Ok(())
}


fn csv_builder(delimiter: u8, path: &str, amount: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = WriterBuilder::new().delimiter(delimiter).from_path(path).unwrap();

    for _ in 0..amount {
        writer.serialize(Faker.fake::<Order>())?;
    }

    Ok(())
}

fn json_builder(path: &str, amount: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(path)?;
    let mut writer = serde_json::Serializer::new(file);

    let mut seq = writer.serialize_seq(Some(amount))?;
    for _ in 0..amount {
        seq.serialize_element(&Faker.fake::<Order>())?;
    }
    seq.end()?;

    Ok(())
}
