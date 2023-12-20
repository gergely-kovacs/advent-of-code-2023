struct Race {
    time_ms: u32,
    distance_mm: u64,
}

#[derive(Debug)]
struct RaceStrategy {
    hold_time_ms: u32,
    distance_mm: u64,
}

const SPEED_INCREASE_PER_MS: u32 = 1;

fn get_strategies_for_race(race: &Race) -> Vec<RaceStrategy> {
    (0..=race.time_ms)
        .filter_map(|hold_time_ms| {
            let travel_time_ms = race.time_ms - hold_time_ms;
            let speed = hold_time_ms * SPEED_INCREASE_PER_MS;
            let distance_mm = speed as u64 * travel_time_ms as u64;
            if distance_mm > race.distance_mm {
                return Some(RaceStrategy {
                    hold_time_ms,
                    distance_mm,
                });
            }
            None
        })
        .collect()
}

fn calculate_margin_of_error(races: Vec<Race>) -> u32 {
    races.iter().fold(0, |acc, race| match acc {
        0 => get_strategies_for_race(race).len() as u32,
        _ => get_strategies_for_race(race).len() as u32 * acc,
    })
}

fn main() {
    let _input_part_1 = Vec::from([
        Race {
            time_ms: 55,
            distance_mm: 246,
        },
        Race {
            time_ms: 82,
            distance_mm: 1441,
        },
        Race {
            time_ms: 64,
            distance_mm: 1012,
        },
        Race {
            time_ms: 90,
            distance_mm: 1111,
        },
    ]);
    let input_part_2 = Vec::from([Race {
        time_ms: 55826490,
        distance_mm: 246144110121111,
    }]);
    println!("{}", calculate_margin_of_error(input_part_2));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        let input = Vec::from([
            super::Race {
                time_ms: 7,
                distance_mm: 9,
            },
            super::Race {
                time_ms: 15,
                distance_mm: 40,
            },
            super::Race {
                time_ms: 30,
                distance_mm: 200,
            },
        ]);
        assert_eq!(super::calculate_margin_of_error(input), 288);
    }

    #[test]
    fn test_part_2() {
        let input = Vec::from([super::Race {
            time_ms: 71530,
            distance_mm: 940200,
        }]);
        assert_eq!(super::calculate_margin_of_error(input), 71503);
    }
}
