use std::fs::File;

use csv::{WriterBuilder, Writer};
use fake::{Dummy, Faker};
use fake::faker::name::en::Name;
use chrono::{DateTime, Utc};
use fake::Fake;

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
    for amount in [10, 10000, 100000, 1000000] {
        builder(b',', &format!("data/US-{amount}.csv"), amount)?;
        builder(b';', &format!("data/EU-{amount}.csv"), amount)?;
        builder(b'\t', &format!("data/EU-{amount}.tsv"), amount)?;
    }


    Ok(())
}


fn builder(delimiter: u8, path: &str, amount: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = WriterBuilder::new().delimiter(delimiter).from_path(path).unwrap();

    for _ in 0..amount {
        writer.serialize(Faker.fake::<Order>())?;
    }

    Ok(())
}
