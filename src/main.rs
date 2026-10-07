use rand::RngExt;
const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

fn print_log(log: &Vec<Reading>) {
    for reading in log {
        println!("{}: {}", reading.day, reading.high);
    }
}
fn average_temp(log: &Vec<Reading>) -> f64 {
    if log.is_empty() {
        return 0.0;
    }
    let mut sum = 0;
    for reading in log {
        sum += reading.high;
    }
    sum as f64 / log.len() as f64
}
fn hottest_day(log: &Vec<Reading>) -> usize {
    let mut best = 0;
    for i in 1..log.len() {
        if log[i].high > log[best].high {
            best = i;
        }
    }
    best
}
fn count_above(log: &Vec<Reading>, threshold: i32) -> usize {
    let mut count = 0;
    for reading in log {
        if reading.high > threshold {
            count += 1;
        }
    }
    count
}
struct Reading {
    day: String,
    high: i32,
}

struct Summary {
    average: f64,
    hottest_day: String,
    days_above: usize,
}
fn summarize(log: &Vec<Reading>, threshold: i32) -> Summary {
    Summary {
        average: average_temp(log),
        hottest_day: log[hottest_day(log)].day.clone(),
        days_above: count_above(log, threshold),
    }
}
fn main() {
    let mut highs: Vec<i32> = vec![72, 68, 75, 81, 79];

    // Step 1
    println!("Initial readings:");
    print_log(&highs);

    // Step 2
    let mut rng = rand::rng();
    for _ in 0..2 {
        highs.push(rng.random_range(60..=100));
    }
    println!("\nAfter adding two more days:");
    print_log(&highs);

    // Step 6
    println!("\nAverage: {:.1}", average_temp(&highs));
    println!("Hottest day: {}", DAYS[hottest_day(&highs)]);
    println!("Days above 75: {}", count_above(&highs, 75));
}

