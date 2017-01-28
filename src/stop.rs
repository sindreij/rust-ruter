use chrono::{DateTime, UTC};

use api;
use errors::*;

#[derive(Debug)]
pub struct Stop {
    id: i32,
    pub name: String,
    district: String,
}

pub struct Departure {
    pub line_number: String,
    pub destination: String,
    pub arrival_time: DateTime<UTC>,
    pub platform: String,
}

impl Stop {
    pub fn find_by_name(name: &str) -> Result<Stop> {
        let places = api::get_places(name)?;
        Stop::from_places(&places)
    }

    fn from_places(places: &[api::Place]) -> Result<Stop> {
        let place = places.iter()
            .filter(|p| p.PlaceType == "Stop")
            .next();
        match place {
            Some(place) => Stop::from_place(place),
            None => bail!("No such stop"),
        }
    }

    fn from_place(place: &api::Place) -> Result<Stop> {
        if place.PlaceType != "Stop" {
            bail!("Tried to create stop from a non-stop place");
        }
        Ok(Stop {
            id: place.ID,
            name: place.Name.clone(),
            district: place.District.clone(),
        })
    }

    pub fn fetch_departures(&self) -> Result<Vec<Departure>> {
        let data = api::get_departures(self.id)?;

        Ok(data.into_iter().map(|dep| {
            Departure {
                line_number: dep.MonitoredVehicleJourney.PublishedLineName,
                destination: dep.MonitoredVehicleJourney.MonitoredCall.DestinationDisplay,
                arrival_time: dep.MonitoredVehicleJourney.MonitoredCall.ExpectedArrivalTime,
                platform: dep.MonitoredVehicleJourney.MonitoredCall.DeparturePlatformName,
            }
        }).collect())
    }
}
