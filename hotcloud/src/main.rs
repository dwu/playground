#![feature(custom_derive, plugin)]

#[macro_use] extern crate log;
#[macro_use] extern crate hyper;
extern crate env_logger;
extern crate toml;
extern crate rustc_serialize;
extern crate rand;
extern crate reqwest;
extern crate threadpool;
extern crate time;
extern crate chrono;

mod config;
mod query;
mod util;
mod generator;

use config::Config;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering, ATOMIC_USIZE_INIT, ATOMIC_BOOL_INIT};
use std::thread;
use std::env;
use std::time::Duration;
use reqwest::{Client, Body};
use reqwest::header::ContentType;

pub static ACTIVE_THREADS: AtomicUsize = ATOMIC_USIZE_INIT;
pub static GENERATOR_RUNNING: AtomicBool = ATOMIC_BOOL_INIT;

pub enum Disruption {
    Node(usize),
    Query(Vec<usize>),
    Metric(Vec<usize>)
}

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let config = Config::parse("config.toml".to_owned());
    let client = Arc::new(Client::builder().timeout(Duration::from_secs(180)).build().unwrap());

    if args.len() == 1 {
        debug!("Resetting index...");
        client.delete("http://localhost:9200/data/").send();
        client.put("http://localhost:9200/data/").header(ContentType::json()).body(config.es.mapping.clone()).send();

        client.delete("http://localhost:9200/hotcloud/").send();
        client.put("http://localhost:9200/hotcloud/").header(ContentType::json()).body(config.es.hotcloudmapping.clone()).send();

        client.get("http://localhost:9200/_cluster/health?wait_for_status=yellow").send();
        client.delete("http://localhost:9200/_scripts/hotcloud").send();
        client.post("http://localhost:9200/_scripts/hotcloud").header(ContentType::json()).body(config.es.query.clone()).send();

        generator::generate_timeline(&client, &config, false);

        while ::ACTIVE_THREADS.load(Ordering::SeqCst) > 0 {
            thread::sleep_ms(500);
        }

        let _ = client.post("http://localhost:9200/data/_refresh").send();
        query::run_hotcloud(&client, config);
    } else if args.len() >= 1 && &args[1] == "json" {
        generator::generate_timeline(&client, &config, true);

        while ::ACTIVE_THREADS.load(Ordering::SeqCst) > 0 {
            thread::sleep_ms(500);
        }
    }

}
