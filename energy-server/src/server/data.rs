use std::fs::File;
use std::io::prelude::*;
use serde_json::from_str;
use std::marker::Send;
// Struct matching the layout of the json for strong typing
#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub current: i32,
    pub voltage: i32,
    pub time: i32,
}

pub trait Queryable : Send {
    fn get_data_between_times(&self, time1: i32, time2: i32) -> Result<f64, String>;
}
pub struct JsonBackedQueryable {
    data: Vec<Record>
}

impl Queryable for JsonBackedQueryable {
    fn get_data_between_times(&self, time1: i32, time2: i32) -> Result<f64, String> {
        if time1 > time2 {
            return Err("Time2 must be greater than time1".to_owned());
        }
        let iter = self.data.iter();
        let mut energy_counter: f64 = 0.0;
        for record in iter {
            if record.time >= time2 {
                break;
            }
            if record.time < time1 {
                continue;
            }
            energy_counter += (record.current*record.voltage) as f64;
        }
        // Convert to Wh
        energy_counter /= 3600.0;
        // Wh to kWh
        energy_counter /= 1000.0;
        Ok(energy_counter)
    }
}

impl JsonBackedQueryable {
    pub fn new(json_file_path: &str) -> JsonBackedQueryable {
        JsonBackedQueryable {data: JsonBackedQueryable::read_data_file(json_file_path)}
    }

    fn read_data_file(file_path: &str) -> Vec<Record> {
        let mut json_file = File::open(file_path).expect("Error opening data file");
        let mut json_string = String::new();
        let result = json_file.read_to_string(&mut json_string);
        match result {
            Result::Err(err) => { 
                panic!("Failed to read json to string with error: {}", err);
            }
            Result::Ok(val) => {
                println!("Read {} bytes from data file", val);
                // Parse data as array and sort by time to make querying faster
                let mut data: Vec<Record> = from_str(json_string.as_ref()).unwrap();
                data.sort_by(|a, b| a.time.cmp(&b.time));
                data
            }
        }
    }
}