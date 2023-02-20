use time::{OffsetDateTime, serde as timeserde};
use serde::{Serialize, Deserialize};
use std::{fmt::Display, fs::{OpenOptions, read, write}, io::{stdin, stdout, Read}};
use serde_json;





timeserde::format_description!(custom_datetime_format, OffsetDateTime, "[day]/[month]/[year]-[hour]:[minute]:[second]");


#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    data : String,
    #[serde(with = "custom_datetime_format")]
    date : OffsetDateTime,
}

impl Entry {
    const DATA_FILEPATH : &'static str = "data.json";

    fn display(&self) -> String {
        return format!("--- {}/{}/{} ---\n{}", self.date.day(), self.date.month(), self.date.year(), self.data);
    }

    fn new(datetime : OffsetDateTime) -> Self {
        return Entry { 
            data: String::new(),
            date: datetime,
        };
    }
    
    fn load_all() -> Vec<Self> {
        let mut entries : Vec<Self> = Vec::new();
        let mut file_data : String = String::new();

        let mut file = OpenOptions::new().create(true).append(true).read(true).open(Entry::DATA_FILEPATH).unwrap();
        file.read_to_string(&mut file_data).unwrap_or(0);

        let element_vector : Vec<&str> = file_data.trim().split("\n").collect();

        for element in element_vector {
            match serde_json::from_str::<Entry>(element) {
                Ok(entry) => entries.push(entry),
                Err(_) => eprintln!("failed to parse line"),
            }
        }

        return entries;
    }

    fn save_all() {}

    fn append<T : Display>(&mut self, additional_data : T) {
        self.data = format!("{}\n{}", self.data, additional_data);
    }

}


fn main() {
    let current_datetime : OffsetDateTime = OffsetDateTime::now_local().unwrap();
    Entry::load_all();
}
