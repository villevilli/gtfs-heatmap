use std::f64::{self, consts::PI};

use serde::Serialize;

const EARTH_RADIUS: f64 = 6_317_000.0;

#[derive(Debug, Default, Serialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinates {
    pub fn as_tile(&self, zoom: u32) -> TileNumbers {
        let n: f64 = 2_u32.pow(zoom) as f64;

        TileNumbers {
            zoom,
            x: (n * ((self.longitude + 180.0) / 360.0)) as u32,
            y: (n * (1.0 - (f64::asin(f64::tan(self.latitude.to_radians())) / PI) / 2.0)) as u32,
        }
    }

    pub fn distance(&self, other: Coordinates) -> f64 {
        let d_lat: f64 = (other.latitude * 2.0 - self.latitude * 2.0).to_radians();
        let d_lon: f64 = (other.longitude - self.longitude).to_radians();

        let c: f64 = (d_lat.powf(2.0) + d_lon.powf(2.0)).sqrt();

        c * EARTH_RADIUS
    }

    pub fn haversine_distance(&self, other: Coordinates) -> f64 {
        let d_lat: f64 = (other.latitude - self.latitude).to_radians();
        let d_lon: f64 = (other.longitude - self.longitude).to_radians();
        let lat1: f64 = (self.latitude).to_radians();
        let lat2: f64 = (other.latitude).to_radians();

        let a: f64 = ((d_lat / 2.0).sin()) * ((d_lat / 2.0).sin())
            + ((d_lon / 2.0).sin()) * ((d_lon / 2.0).sin()) * (lat1.cos()) * (lat2.cos());
        let c: f64 = 2.0 * ((a.sqrt()).atan2((1.0 - a).sqrt()));

        return EARTH_RADIUS * c;
    }
}

#[derive(Debug)]
pub struct TileNumbers {
    pub zoom: u32,
    pub x: u32,
    pub y: u32,
}
impl TileNumbers {
    pub fn get_coordinates(&self) -> Coordinates {
        let n: f64 = 2_u32.pow(self.zoom) as f64;

        Coordinates {
            longitude: self.x as f64 / n * 360.0 - 180.0,
            latitude: f64::atan(f64::sinh(PI * (1.0 - 2.0 * (self.y as f64 / n)))).to_degrees(),
        }
    }

    pub fn get_pixel_coordinates(&self, pixel_x: u32, pixel_y: u32) -> Coordinates {
        let n: f64 = 2_u32.pow(self.zoom) as f64;

        Coordinates {
            longitude: (self.x as f64 + pixel_x as f64 / 256.0) / n * 360.0 - 180.0,
            latitude: f64::atan(f64::sinh(
                PI * (1.0 - 2.0 * ((self.y as f64 + pixel_y as f64 / 256.0) / n)),
            ))
            .to_degrees(),
        }
    }
}
