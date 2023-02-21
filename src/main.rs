// version 0.1
// todo-tool, a simple terminal application to quickly create daily todo notes

use time::OffsetDateTime;
use serde::{Serialize, Deserialize};
use std::{fmt::Display, fs::{OpenOptions}, io::{stdin, stdout, Read, Write}};
use serde_json;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Entry {
    data : String,
    #[serde(with = "time::serde::iso8601")]
    date : OffsetDateTime,
}

impl Entry {
    const DATA_FILEPATH : &'static str = "data.json";

    fn display(&self) -> String {
        if self.data.is_empty() {
            return format!("--- {:0>2}/{:0>2}/{} ---\n()\n", self.date.day(), self.date.month() as u8, self.date.year());
        } else {
            return format!("--- {:0>2}/{:0>2}/{} ---\n{}", self.date.day(), self.date.month() as u8, self.date.year(), self.data);
        }
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
                Err(error) => {
                    if !element.is_empty() {eprintln!("failed to parse the following line from data.json : {}\n--> {}", element, error)};
                },
            }
        }
        return entries;
    }
        
    fn save_all(entries : &Vec<Self>) {
        let mut file = OpenOptions::new().create(true).write(true).open(Entry::DATA_FILEPATH).unwrap();
        let mut data : String = String::new();
        for entry in entries {
            data = data + &serde_json::to_string(entry).unwrap() + "\n";
        }
        file.write_all(data.as_bytes()).unwrap();
    }

    fn append<T : Display>(&mut self, additional_data : T) {
        self.data = format!("{}{}\n", self.data, additional_data);
    }

}

fn main() {
    let current_datetime : OffsetDateTime = OffsetDateTime::now_local().unwrap();

    println!(" ----- todo-tool version 0.1 -----\ntype _help for more information\n");

    let print_help = || {
        println!("_help    : print help information\n_exit    : exit application\n_display : diplays every single entry stored\n_clear   : clears the contents of today's entry\n_today   : displays today's entry\n");
    };

    let newline = || {
        print!("\n");
    };

    let mut entries : Vec<Entry> = Entry::load_all();
    let mut user_input : String = String::new();
    let empty_entry : Entry = Entry::new(current_datetime.clone());

    if entries.len() == 0 {
        entries.push(empty_entry.clone());
        Entry::save_all(&entries);
    }

    // checks if a new todo entry has to be created
    let mut entry_index : usize = entries.len() - 1;
    let last_entry_date_tag = entries[entry_index].date.year() * 10000 + entries[entry_index].date.month() as i32 * 100 + entries[entry_index].date.day() as i32;
    let current_date_tag : i32 =  current_datetime.date().year() * 10000 + current_datetime.date().month() as i32 * 100 + current_datetime.date().day() as i32;
    if current_date_tag > last_entry_date_tag {
        entries.push(empty_entry);
        Entry::save_all(&entries);
        entry_index += 1;
    }

    if entries[entry_index].data.is_empty() {
        println!(" --- today's entry ---\n()\n");
    } else {
        println!(" --- today's entry ---\n{}", &entries[entry_index].data);
    }

    loop {
        stdout().write_all(">> ".as_bytes()).unwrap(); stdout().flush().unwrap();
        stdin().read_line(&mut user_input).unwrap();
        newline();
        match user_input.trim() {
            "_help" => print_help(),
            "_exit" => break,
            "_quit" => break,
            "_clear" => {
                entries[entry_index].data.clear();
                Entry::save_all(&entries);
            },
            "_today" => {
                if entries[entry_index].data.is_empty() {
                    println!(" --- today's entry ---\n()\n");
                } else {
                    println!(" --- today's entry ---\n{}", &entries[entry_index].data);
                }
            }
            "_display" => {
                for entry in entries.iter() {
                    println!("{}", entry.display());
                }
            }
            _ => {
                entries[entry_index].append(user_input.trim());
                Entry::save_all(&entries);
            },
        }
        user_input.clear();
    }

}
