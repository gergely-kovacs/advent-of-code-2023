#[derive(Debug, PartialEq)]
struct AlmanacMapRange {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

#[derive(Debug, PartialEq)]
enum AlmanacMapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl std::str::FromStr for AlmanacMapType {
    type Err = ();

    fn from_str(input: &str) -> Result<AlmanacMapType, Self::Err> {
        match input {
            "seed-to-soil" => Ok(AlmanacMapType::SeedToSoil),
            "soil-to-fertilizer" => Ok(AlmanacMapType::SoilToFertilizer),
            "fertilizer-to-water" => Ok(AlmanacMapType::FertilizerToWater),
            "water-to-light" => Ok(AlmanacMapType::WaterToLight),
            "light-to-temperature" => Ok(AlmanacMapType::LightToTemperature),
            "temperature-to-humidity" => Ok(AlmanacMapType::TemperatureToHumidity),
            "humidity-to-location" => Ok(AlmanacMapType::HumidityToLocation),
            _ => Err(()),
        }
    }
}

impl ToString for AlmanacMapType {
    fn to_string(&self) -> String {
        match self {
            AlmanacMapType::SeedToSoil => "seed-to-soil".to_string(),
            AlmanacMapType::SoilToFertilizer => "soil-to-fertilizer".to_string(),
            AlmanacMapType::FertilizerToWater => "fertilizer-to-water".to_string(),
            AlmanacMapType::WaterToLight => "water-to-light".to_string(),
            AlmanacMapType::LightToTemperature => "light-to-temperature".to_string(),
            AlmanacMapType::TemperatureToHumidity => "temperature-to-humidity".to_string(),
            AlmanacMapType::HumidityToLocation => "humidity-to-location".to_string(),
        }
    }
}

fn get_seeds_from_almanac(almanac: &str) -> Vec<u64> {
    almanac
        .lines()
        .find(|line| line.starts_with("seeds: "))
        .unwrap()[7..]
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn get_section_from_almanac(almanac: &str, section_name: AlmanacMapType) -> String {
    let section_start = format!("{} map:\n", section_name.to_string());
    let section_end = "\n\n";

    if let Some(start_index) = almanac.find(&section_start) {
        let section_trimmed_start = &almanac[start_index + section_start.len()..];
        if let Some(end_index) = section_trimmed_start.find(section_end) {
            return section_trimmed_start[..end_index].trim().to_string();
        }
    }
    "".to_string()
}

fn map_section_to_almanac_ranges(almanac_section: &str) -> Vec<AlmanacMapRange> {
    almanac_section
        .lines()
        .map(|line| {
            let [destination_range_start, source_range_start, range_length]: [u64; 3] = line
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
                .try_into()
                .unwrap();
            AlmanacMapRange {
                destination_range_start,
                source_range_start,
                range_length,
            }
        })
        .collect()
}

fn map_number_according_to_map(number: u64, map: &Vec<AlmanacMapRange>) -> u64 {
    let active_range = map.iter().find(|map_range| {
        number >= map_range.source_range_start
            && number < map_range.source_range_start + map_range.range_length
    });
    if let Some(active_range) = active_range {
        return active_range.destination_range_start + number - active_range.source_range_start;
    }
    number
}

fn find_lowest_location_number(almanac: &str) -> u64 {
    let seeds: Vec<u64> = get_seeds_from_almanac(almanac);
    let soil_map = map_section_to_almanac_ranges(&get_section_from_almanac(
        almanac,
        AlmanacMapType::SeedToSoil,
    ));
    let fertilizer_map = map_section_to_almanac_ranges(&get_section_from_almanac(
        almanac,
        AlmanacMapType::SoilToFertilizer,
    ));
    let water_map = map_section_to_almanac_ranges(&get_section_from_almanac(
        almanac,
        AlmanacMapType::FertilizerToWater,
    ));
    let light_map = map_section_to_almanac_ranges(&get_section_from_almanac(
        almanac,
        AlmanacMapType::WaterToLight,
    ));
    let temperature_map = map_section_to_almanac_ranges(&get_section_from_almanac(
        almanac,
        AlmanacMapType::LightToTemperature,
    ));
    let humidity_map = map_section_to_almanac_ranges(&get_section_from_almanac(
        almanac,
        AlmanacMapType::TemperatureToHumidity,
    ));
    let location_map = map_section_to_almanac_ranges(&get_section_from_almanac(
        almanac,
        AlmanacMapType::HumidityToLocation,
    ));
    seeds
        .iter()
        .map(|seed| {
            let soil = map_number_according_to_map(*seed, &soil_map);
            let fertilizer = map_number_according_to_map(soil, &fertilizer_map);
            let water = map_number_according_to_map(fertilizer, &water_map);
            let light = map_number_according_to_map(water, &light_map);
            let temperature = map_number_according_to_map(light, &temperature_map);
            let humidity = map_number_according_to_map(temperature, &humidity_map);
            map_number_according_to_map(humidity, &location_map)
        })
        .min()
        .unwrap_or(0)
}

fn main() {
    let almanac = include_str!("../almanac.txt");
    // 83692320 is too low
    println!("{:?}", find_lowest_location_number(almanac));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_lowest_location_number() {
        let almanac = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(find_lowest_location_number(almanac), 35);
    }
}
