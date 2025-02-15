use geo::{Area, BooleanOps, Contains};
use rand::Rng;
use rand_xorshift::XorShiftRng;

use abstutil::prettyprint_usize;
use map_model::{BuildingID, Map};

use crate::{CensusArea, CensusPerson, Config};

pub fn assign_people_to_houses(
    areas: Vec<CensusArea>,
    map: &Map,
    rng: &mut XorShiftRng,
    _config: &Config,
) -> Vec<CensusPerson> {
    let mut people = Vec::new();
    for area in areas {
        for (home, n) in distribute_population_to_homes(area.polygon, area.population, map, rng) {
            for _ in 0..n {
                people.push(CensusPerson {
                    home,
                    // TODO Making this up for now. We can either move this to Config or see if we
                    // can extract it from the census. Also, not even sure which of these
                    // attributes are useful later in the pipeline.
                    age: rng.gen_range(5..95),
                    employed: rng.gen_bool(0.7),
                    owns_car: rng.gen_bool(0.5),
                });
            }
        }
    }
    people
}

/// Starting from some number of total people living in a polygonal area, randomly distribute them
/// to residential buildings within that area. Returns a list of homes with the number of residents
/// in each.
pub fn distribute_population_to_homes(
    polygon: geo::Polygon<f64>,
    population: usize,
    map: &Map,
    rng: &mut XorShiftRng,
) -> Vec<(BuildingID, usize)> {
    let map_boundary = geo::Polygon::from(map.get_boundary_polygon().clone());
    let bldgs: Vec<map_model::BuildingID> = map
        .all_buildings()
        .iter()
        .filter(|b| {
            polygon.contains(&geo::Point::from(b.label_center)) && b.bldg_type.has_residents()
        })
        .map(|b| b.id)
        .collect();

    // If the area is partly out-of-bounds, then scale down the number of residents linearly
    // based on area of the overlapping part of the polygon.
    let pct_overlap = polygon.intersection(&map_boundary).unsigned_area() / polygon.unsigned_area();
    let num_residents = (pct_overlap * (population as f64)) as usize;
    debug!(
        "Distributing {} residents to {} buildings. {}% of this area overlapped with the map, \
         scaled residents accordingly.",
        prettyprint_usize(num_residents),
        prettyprint_usize(bldgs.len()),
        (pct_overlap * 100.0) as usize
    );

    // How do you randomly distribute num_residents into some buildings?
    // https://stackoverflow.com/questions/2640053/getting-n-random-numbers-whose-sum-is-m
    // TODO Problems:
    // - Because of how we round, the sum might not exactly be num_residents
    // - This is not a uniform distribution, per stackoverflow
    // - Larger buildings should get more people

    let mut count_per_home = Vec::new();
    let mut rand_nums: Vec<f64> = (0..bldgs.len()).map(|_| rng.gen_range(0.0..1.0)).collect();
    let sum: f64 = rand_nums.iter().sum();
    for b in bldgs {
        let n = (rand_nums.pop().unwrap() / sum * (num_residents as f64)) as usize;
        count_per_home.push((b, n));
    }
    count_per_home
}
