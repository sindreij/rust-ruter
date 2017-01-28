#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate clap;
extern crate chrono;

mod errors;
mod api;
mod stop;

use clap::{App, Arg};
use chrono::{UTC};

use errors::*;
use stop::Stop;

quick_main!(run);

fn run() -> Result<()> {
    let app = App::new("Ruter API Command Line Client")
        .version("1.0")
        .author("Sindre I. Johansen <sindre@sindrejohansen.no>")
        .arg(Arg::with_name("STOP")
            .help("Select the stop to search for")
            .required(true)
            .index(1));

    let matches = app.get_matches();
    let stop_name = matches.value_of("STOP").unwrap();


    let stop = Stop::find_by_name(stop_name)?;

    let departures = stop.fetch_departures()?;

    println!("Avganger fra {}", stop.name);

    for departure in departures.iter().take(10) {
        println!("{:>3} {:25} {:>7} {}",
            departure.line_number,
            departure.destination,
            pretty_time(departure.arrival_time),
            departure.platform);
    }

    Ok(())
}

fn pretty_time(time: chrono::DateTime<UTC>) -> String {
    let duration = time - UTC::now();
    let seconds = duration.num_seconds();
    let minutes = seconds / 60;
    let seconds = seconds % 60;

    if minutes > 10 {
        return format!("{}", time.format("%H:%M"))
    }

    format!("{:>2}m {:>2}s", minutes, seconds)
}
