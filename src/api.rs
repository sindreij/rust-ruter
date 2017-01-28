use reqwest;

use serde_json;
use chrono::{DateTime, UTC};

use errors::*;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Place {
    pub ID: i32,
    pub Name: String,
    pub District: String,
    pub PlaceType: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct MonitoredStopVisit {
    pub RecordedAtTime: String,
    pub MonitoringRef: String,
    pub MonitoredVehicleJourney: MonitoredVehicleJourney,
    pub Extensions: Extensions,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct MonitoredVehicleJourney {
    pub LineRef: String,
    pub DirectionRef: Option<String>,
    pub DirectionName: Option<String>,
    pub PublishedLineName: String,
    pub OperatorRef: Option<String>,
    pub OriginName: String,
    pub OriginRef: String,
    pub DestinationRef: String,
    pub DestinationName: String,
    pub Monitored: bool,
    pub InCongestion: bool,
    pub Delay: String,
    pub MonitoredCall: MonitoredCall,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Extensions {
    pub IsHub: bool,
    pub LineColour: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct MonitoredCall {
    pub VisitNumber: i32,
    pub VehicleAtStop: bool,
    pub DestinationDisplay: String,
    pub AimedArrivalTime: DateTime<UTC>,
    pub ExpectedArrivalTime: DateTime<UTC>,
    pub AimedDepartureTime: DateTime<UTC>,
    pub ExpectedDepartureTime: DateTime<UTC>,
    pub DeparturePlatformName: String,
}

pub fn get_places(id: &str) -> Result<Vec<Place>> {
    let resp = reqwest::get(&format!("http://reisapi.ruter.no/Place/GetPlaces/{}", id))?;
    if !resp.status().is_success() {
        bail!("Error from ruter api");
    }

    Ok(serde_json::from_reader(resp)?)
}

pub fn get_departures(id: i32) -> Result<Vec<MonitoredStopVisit>> {
    let resp = reqwest::get(&format!("http://reisapi.ruter.no/StopVisit/GetDepartures/{}", id))?;
    if !resp.status().is_success() {
        bail!("Error from ruter api");
    }

    Ok(serde_json::from_reader(resp)?)
}
