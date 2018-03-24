
extern crate reqwest;

use ::Disruption;
use rustc_serialize::{Encodable};
use rustc_serialize::json::{self};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use time::PreciseTime;
use reqwest::header::ContentType;

header! { (ContentTypeString, "Content-Type") => [String] }

pub fn disruption_to_usize(d: &Option<&(Disruption, usize)>) -> usize {
    match *d {
        None => 0,
        Some(&(Disruption::Node(_), _)) => 1,
        Some(&(Disruption::Query(_), _)) => 2,
        Some(&(Disruption::Metric(_), _)) => 3
    }
}

pub fn send_bulk<T: Encodable>(url: &str, client: &Arc<reqwest::Client>, bulk: Vec<T>) {
    ::ACTIVE_THREADS.fetch_add(1, Ordering::SeqCst);

    debug!("......");
    let mut s = String::new();
    let size = bulk.len();
    for b in bulk {
        s.push_str("{\"index\":{}}\n");
        let o = &json::encode(&b).unwrap();
        s.push_str(o);
        s.push_str("\n");
    }

    debug!("                      >>>>> Bulk: {} mb ({} elements)", s.len() / 1024 / 1024, size);
    let start = PreciseTime::now();
    let _ = client.post(url)
        .header(ContentTypeString("application/x-ndjson".to_owned()))
        .body(s)
        .send()
        .unwrap();
    let end = PreciseTime::now();
    let _ = PreciseTime::to(&start, end);
    ::ACTIVE_THREADS.fetch_sub(1, Ordering::SeqCst);
}
