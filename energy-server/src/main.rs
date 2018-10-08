#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate iron;
extern crate params;

mod server;
use server::data::JsonBackedQueryable;
use server::server::Server;

fn main() {
    let data_source = JsonBackedQueryable::new("sensors.json");
    let server = Server::new(data_source);
    server.start();
}
