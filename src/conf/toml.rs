extern crate toml;

use std::io::prelude::*;
use std::fs::File;
use std::collections::btree_map::BTreeMap;
use std::net::SocketAddr;
use std::str::FromStr;

//FIXME this is all a gross hack

fn get_conf() -> Option<BTreeMap<String,toml::Value>> {
	let mut conf_file = File::open("conf/war_machine.toml").unwrap();
	let mut conf_string = String::new();
	conf_file.read_to_string(&mut conf_string).unwrap();
	let mut parser = toml::Parser::new(&conf_string);
	
	let conf:BTreeMap<String,toml::Value> = match parser.parse() {
        Some(toml) => toml,
        None => {
            for err in &parser.errors {
                let (loline, locol) = parser.to_linecol(err.lo);
                let (hiline, hicol) = parser.to_linecol(err.hi);
                println!("{}:{}:{}-{}:{} error: {}",
                         "war_machine.toml", loline, locol, hiline, hicol, err.desc);
            }
            return None
        }
    };	
	println!("{:#?}",conf);
	Some(conf)
}

pub fn http_port() -> Option<u16> {
	let conf = get_conf().unwrap();
	let http_server = conf.get("http_server").unwrap().as_table().unwrap();
	let port = http_server.get("port").unwrap().clone().as_integer().unwrap() as u16;
	Some (port)
}

//FIXME extend the cassandadr client api to take a Vec<SocketAddr> and then use the commented code
pub fn contact_points() -> String {
	let conf = get_conf().unwrap();
	let cassandra_proxy = conf.get("cassandra_proxy").unwrap().as_table().unwrap();
	let contact_points = cassandra_proxy.get("contact_points").unwrap().clone();
//	let mut contact_points = Vec::new();
//	//FIXME use map
//	for contact_point in cassandra_proxy.get("contact_points").unwrap().clone().as_slice().unwrap() {
//		contact_points.push(SocketAddr::from_str(contact_point.as_str().unwrap()).unwrap())
//	}
	contact_points.as_str().unwrap().into()
}