use car::Car;
use city::{City, Lights};

mod car;
mod city;

// The `race` method runs the provided `cars` on their paths
// inside the given `city`. You can assume that no car's path
// will include incorrect intersections (that is those not present)
// inside the `city`).
// It returns a vector of car names, in order of their arrivals
// at their respective finish lines.
//
// You can remove `mut` qualifiers if they are not necessary for
// your solution.
async fn race(mut city: City, mut cars: Vec<Car>) -> Vec<String> {
    todo!()
}

#[tokio::main]
async fn main() {
    let mut city = City::new(3, 3);
    city.add_lights(0, 0, Lights::green(100));
    city.add_lights(1, 0, Lights::green(100));
    city.add_lights(2, 0, Lights::green(100));
    city.add_lights(0, 1, Lights::red(100));
    city.add_lights(1, 1, Lights::red(200));
    city.add_lights(2, 1, Lights::red(300));

    let cars = vec![
        Car::new("Kubica", vec![(1, 0), (1, 1), (1, 2)]),
        Car::new("Millie", vec![(0, 0), (0, 1), (0, 2)]),
        Car::new("Mr. M", vec![(2, 0), (2, 1), (2, 2)]),
    ];

    let results = race(city, cars).await;
    for name in results {
        println!("{}", name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::future::Future;
    use tokio::time::{Duration, Instant};

    async fn test_timed(ms: (u64, u64), test: impl Future<Output = ()>) {
        let start = Instant::now();
        test.await;
        let t = start.elapsed();
        assert!(t >= Duration::from_millis(ms.0));
        assert!(t <= Duration::from_millis(ms.1));
    }

    #[tokio::test]
    async fn test_no_lights() {
        for _ in 0..10 {
            let city = City::new(2, 4);
            let cars = vec![
                Car::new("A", vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
                Car::new("B", vec![(1, 0), (1, 1), (1, 2), (1, 3)]),
            ];

            let results = race(city, cars).await;
            assert_eq!(results, vec!["A".to_string(), "B".to_string()]);
        }
    }

    #[tokio::test]
    async fn test_all_green() {
        test_timed((0, 100), async {
            let mut city = City::new(2, 2);
            city.add_lights(0, 0, Lights::green(1000));
            city.add_lights(0, 1, Lights::green(1000));
            city.add_lights(1, 0, Lights::green(1000));
            city.add_lights(1, 1, Lights::green(1000));

            let cars = vec![
                Car::new("A", vec![(0, 0), (0, 1)]),
                Car::new("B", vec![(1, 0), (1, 1)]),
            ];

            let results = race(city, cars).await;
            assert_eq!(results, vec!["A".to_string(), "B".to_string()])
        })
        .await;
    }

    #[tokio::test]
    async fn test_separate_lanes() {
        test_timed((300, 400), async {
            let mut city = City::new(3, 3);
            city.add_lights(0, 1, Lights::red(100));
            city.add_lights(1, 1, Lights::red(200));
            city.add_lights(2, 1, Lights::red(300));

            let cars = vec![
                Car::new("Kubica", vec![(1, 0), (1, 1), (1, 2)]),
                Car::new("Millie", vec![(0, 0), (0, 1), (0, 2)]),
                Car::new("Mr. M", vec![(2, 0), (2, 1), (2, 2)]),
            ];

            let results = race(city, cars).await;
            assert_eq!(
                results,
                vec![
                    "Millie".to_string(),
                    "Kubica".to_string(),
                    "Mr. M".to_string()
                ]
            );
        })
        .await;
    }

    #[tokio::test]
    async fn test_mid_race_red_lights() {
        test_timed((400, 450), async {
            let mut city = City::new(3, 3);
            city.add_lights(1, 0, Lights::red(100));
            city.add_lights(0, 0, Lights::red(300));
            city.add_lights(1, 1, Lights::green(200));

            let cars = vec![
                Car::new("A", vec![(0, 0), (0, 1), (1, 1), (1, 2)]),
                Car::new("B", vec![(1, 0), (1, 1), (1, 2)]),
            ];

            let results = race(city, cars).await;
            assert_eq!(results, vec!["B".to_string(), "A".to_string()]);
        })
        .await;
    }

    #[tokio::test]
    async fn test_same_lane_split_finish() {
        test_timed((800, 850), async {
            let mut city = City::new(3, 5);
            city.add_lights(1, 0, Lights::red(100));
            city.add_lights(1, 1, Lights::red(200));
            city.add_lights(1, 2, Lights::red(300));
            city.add_lights(1, 3, Lights::red(450));
            city.add_lights(1, 4, Lights::green(400));
            city.add_lights(0, 3, Lights::green(1000));
            city.add_lights(2, 3, Lights::red(700));

            let cars = vec![
                Car::new("A", vec![(1, 0), (1, 1), (1, 2), (1, 3), (1, 4)]),
                Car::new("B", vec![(1, 0), (1, 1), (1, 2), (1, 3), (0, 3), (0, 4)]),
                Car::new("C", vec![(1, 0), (1, 1), (1, 2), (1, 3), (2, 3), (2, 4)]),
            ];

            let results = race(city, cars).await;
            assert_eq!(
                results,
                vec!["B".to_string(), "C".to_string(), "A".to_string()]
            );
        })
        .await;
    }

    #[tokio::test]
    async fn test_long_change_times_no_waiting() {
        test_timed((0, 100), async {
            let mut city = City::new(3, 3);
            city.add_lights(0, 0, Lights::red(1000));
            city.add_lights(0, 1, Lights::red(1000));
            city.add_lights(0, 2, Lights::red(1000));
            city.add_lights(1, 0, Lights::green(1000));
            city.add_lights(1, 1, Lights::green(1000));
            city.add_lights(1, 2, Lights::green(1000));
            city.add_lights(2, 0, Lights::red(1000));
            city.add_lights(2, 1, Lights::red(1000));
            city.add_lights(2, 2, Lights::red(1000));

            let cars = vec![Car::new("Kubica", vec![(1, 0), (1, 1), (1, 2)])];

            let results = race(city, cars).await;
            assert_eq!(results, vec!["Kubica".to_string()]);
        })
        .await;
    }

    #[tokio::test]
    async fn test_singular_lights_multiple_changes() {
        test_timed((800, 900), async {
            let mut city = City::new(1, 3);
            city.add_lights(0, 0, Lights::red(700));
            city.add_lights(0, 1, Lights::green(200));

            let cars = vec![Car::new("Kubica", vec![(0, 0), (0, 1), (0, 2)])];

            let results = race(city, cars).await;
            assert_eq!(results, vec!["Kubica".to_string()]);
        }).await;
    }
}
