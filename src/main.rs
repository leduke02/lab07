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
// Builds a Summary from the log using the three helper functions
fn summarize(log: &Vec<Reading>, threshold: i32) -> Summary {
    Summary {
        average: average_temp(log),
        hottest_day: log[hottest_day(log)].day.clone(),
        days_above: count_above(log, threshold),
    }
}
// Builds the log from DAYS and readings, then prints the summary
fn main() {
    // Step 1
    let mut log: Vec<Reading> = Vec::new();
    for (i, &high) in [72, 68, 75, 81, 79].iter().enumerate() {
        log.push(Reading { day: DAYS[i].to_string(), high });
    }
    println!("Initial readings:");
    print_log(&log);

    // Step 2
    let mut rng = rand::rng();
    for _ in 0..2 {
        log.push(Reading {
            day: DAYS[log.len()].to_string(),
            high: rng.random_range(60..=100),
        });
    }
    println!("\nAfter adding two more days:");
    print_log(&log);

    // Step 6
    let s = summarize(&log, 75);
    println!("\nAverage: {:.1}", s.average);
    println!("Hottest day: {}", s.hottest_day);
    println!("Days above 75: {}", s.days_above);
}

