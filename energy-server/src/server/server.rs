use server::data::Queryable;
use iron::prelude::*;
use iron;
use std::sync::Mutex;
pub struct Server<T> {
    data: Mutex<T>
}

static START: &'static str = "starttime";
static END: &'static str = "endtime";
static UNIT: &'static str = "kWh";

impl <T: 'static + Queryable> Server<T>{
    pub fn new(data_source: T) -> Server<T> {
        Server { data: Mutex::new(data_source)}
    }
    pub fn start(self) {

        Iron::new(move |req : &mut Request| -> IronResult<Response> {
                use params::Params;
                use params::FromValue;
                let param_map = req.get_ref::<Params>().unwrap();
                let start_time = &param_map[START];
                let start_time_int = i32::from_value(start_time).unwrap();
                let end_time = &param_map[END];
                let end_time_int = i32::from_value(end_time).unwrap();
                let data = &self.data.lock().unwrap().get_data_between_times(start_time_int, end_time_int);
                match data {
                    Ok(val) => {
                        let response = format!(r#"{{"results":{{"energy":{:?},"units":{}}}}}"#, val, UNIT);
                        Ok(Response::with((iron::status::Ok, response)))}
                    Err(_) => Ok(Response::with(iron::status::NotFound))
                }
                })
            .http("localhost:8080").unwrap();
    }
}