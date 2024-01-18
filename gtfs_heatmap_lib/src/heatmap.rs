use image::{GrayAlphaImage, LumaA};
use rusqlite::Connection;

use crate::{
    coords::{Coordinates, TileNumbers},
    dijkstras::TimeLookupTable,
    get_nearby_stops,
    gtfs_types::Stop,
};

const WALKING_SPEED: f64 = 1.4;
const TILE_SIZE: u32 = 256;
const CACHE_DIVIDER: u32 = 16;

type StopCacheArray = Vec<Vec<Vec<Stop>>>;

#[derive(Debug)]
struct StopCache {
    stops: StopCacheArray,
    tile: TileNumbers,
}

impl StopCache {
    fn get(&self, pixel_x: u32, pixel_y: u32) -> &Vec<Stop> {
        let mut cache_x = pixel_x / CACHE_DIVIDER;

        if cache_x & CACHE_DIVIDER > CACHE_DIVIDER / 2 {
            cache_x += 1;
        }

        let mut cache_y = pixel_y / CACHE_DIVIDER;

        if cache_y & CACHE_DIVIDER > CACHE_DIVIDER / 2 {
            cache_y += 1;
        }

        self.stops
            .get(cache_x as usize)
            .unwrap_or_else(|| panic!("{:#?}", self))
            .get(cache_y as usize)
            .unwrap_or_else(|| panic!("{:#?}", self.stops.len()))
    }
    fn new(tile: TileNumbers, connection: &Connection, search_box_distance: f64) -> StopCache {
        StopCache {
            stops: StopCache::gen_stops(&tile, connection, search_box_distance),
            tile,
        }
    }
    fn gen_stops(
        tile: &TileNumbers,
        connection: &Connection,
        search_box_distance: f64,
    ) -> StopCacheArray {
        let mut stops: StopCacheArray = StopCacheArray::new();

        for x in 0..(TILE_SIZE / CACHE_DIVIDER) {
            stops.push(Vec::new());

            let i = x as usize;

            let row = stops.get_mut(i).unwrap();

            for y in 0..(TILE_SIZE / CACHE_DIVIDER) {
                row.push(
                    get_nearby_stops(
                        &connection,
                        TileNumbers::get_pixel_coordinates(
                            &tile,
                            x * 16 + CACHE_DIVIDER / 2,
                            y * 16 + CACHE_DIVIDER / 2,
                        ),
                        search_box_distance,
                    )
                    .unwrap(),
                )
            }
        }

        stops
    }
}

pub fn generate_heatmap_tile(
    zoom: u32,
    tile_x: u32,
    tile_y: u32,
    connection: Connection,
    stop_time_lookuptable: &TimeLookupTable,
) -> GrayAlphaImage {
    let mut image = GrayAlphaImage::new(TILE_SIZE, TILE_SIZE);

    // let mut nearby_stops: Vec<Stop>;

    let mut search_radius = (360_f64 / (2_f64.powf(zoom as f64))) * 1.2;

    if search_radius > 0.001 {
        search_radius /= CACHE_DIVIDER as f64;
    }

    let stop_cache = StopCache::new(
        TileNumbers {
            zoom,
            x: tile_x,
            y: tile_y,
        },
        &connection,
        0.02,
    );

    for x in 0..image.width() {
        for y in 0..image.height() {
            let nearby_stops = stop_cache.get(x, y);
            image.put_pixel(
                x,
                y,
                LumaA([
                    0,
                    get_pixel_brightenss(
                        x,
                        y,
                        &nearby_stops,
                        TileNumbers {
                            zoom,
                            x: tile_x,
                            y: tile_y,
                        },
                        &stop_time_lookuptable,
                    ),
                ]),
            )
        }
    }

    image
}

fn get_pixel_brightenss(
    x: u32,
    y: u32,
    nearby_stops: &Vec<Stop>,
    current_tile: TileNumbers,
    time_lookup_table: &TimeLookupTable,
) -> u8 {
    let time: (f64) = nearby_stops.iter().fold(f64::INFINITY, |acc, stop| -> f64 {
        let mut distance = stop
            .coordinates
            .distance(current_tile.get_pixel_coordinates(x, y))
            / WALKING_SPEED;

        distance += *time_lookup_table
            .get(&stop.stop_id)
            .or(Some(&u32::MAX))
            .unwrap() as f64;

        if distance < acc {
            return distance;
        } else {
            acc
        }
    });

    (time / 1.0) as u8
}
