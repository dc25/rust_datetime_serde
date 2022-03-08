use std::io::BufReader;
use stable_eyre::*;
use chrono::{DateTime, Utc};


#[derive(Debug, Clone, serde::Deserialize)]
pub struct CsvData
{
    pub name: String,
    pub time: DateTime<Utc>,
    pub delta: f32,
}

const CSV: &'static str = "
name,time,delta
foo,2021-06-08T12:29:24.375Z,200.00
";


fn main() -> Result<()> {
    stable_eyre::install()?;

    let bufreader = BufReader::new(CSV.as_bytes());
    let mut rdr = csv::Reader::from_reader(bufreader);

    for result in rdr.deserialize::<CsvData>() {
        if let Ok(record) = result {
            println!("{:?}", record);
        } else {
            println!("Error");
        }
    }

    Ok(())

}
