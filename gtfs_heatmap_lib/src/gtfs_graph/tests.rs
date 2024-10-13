#[test]
fn to_datetime_midnight() {
    let date = date!(2003 - 5 - 16);
    let time = SECONDS_IN_DAY;

    let datetime = Edge::to_datetime(&time, date);

    assert_eq!(datetime, datetime!(2003 - 5 - 17 0:00 UTC));
}

#[test]
fn to_datetime_past_midnight() {
    let date = date!(2003 - 5 - 16);
    let time = SECONDS_IN_DAY + 120;

    let datetime = Edge::to_datetime(&time, date);

    assert_eq!(datetime, datetime!(2003 - 5 - 17 0:02 UTC))
}

#[test]
fn test_get_stops() {
    let gtfs = gtfs_structures::Gtfs::from_path("../data").unwrap();

    let gtfs_graph: GtfsGraph = gtfs.try_into().unwrap();

    gtfs_graph.get_stops();
}

#[test]
fn gtfs_to_graph() -> Result<(), Box<dyn error::Error>> {
    let mut gtfs = gtfs_structures::Gtfs::from_path("../hsl.zip")?;

    let mut stops: HashMap<String, Arc<RwLock<Stop>>> = HashMap::new();
    for (id, stop) in gtfs.stops.drain() {
        if let Ok(stop) = Arc::unwrap_or_clone(stop).try_into() {
            stops.insert(id, Arc::new(RwLock::new(stop)));
        }
    }
    Ok(())
}

#[test]
fn parse_gtfs_to_graph() -> Result<(), Box<dyn error::Error>> {
    let gtfs = gtfs_structures::Gtfs::from_path("../hsl.zip")?;

    let _gtfs_graph: GtfsGraph = gtfs.try_into()?;

    Ok(())
}
