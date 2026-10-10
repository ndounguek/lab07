use rand::Rng;

const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
struct Reading {
    day: String,
    high: i32,
}

struct Summary {
    average: f64,
    hottest_day: String,
    days_above: usize,
}
fn main() {
    let mut highs: Vec<i32> = vec![72, 68, 75, 81, 79];

    for i in 0..highs.len() {
        println!("{}: {}", DAYS[i], highs[i]);
    }

    let mut rng = rand::rng();
    highs.push(rng.random_range(60..=100));
    highs.push(rng.random_range(60..=100));
 let mut log: Vec<Reading> = Vec::new();
    for i in 0..highs.len() {
        log.push(Reading {
            day: DAYS[i].to_string(),
            high: highs[i],
        });
    }
    for i in 0..log.len() {
        println!("{}: {}", log[i].day, log[i].high);
    }
let summary = summarize(&log, 75);
println!("Average: {}", summary.average);
println!("Hottest day: {}", summary.hottest_day);
println!("Days above 75: {}", summary.days_above);
}

fn average_temp(log: &Vec<Reading>) -> f64 {
    if log.is_empty() {
        return 0.0;
    }
    let mut total = 0;
    for i in 0..log.len() {
        total = total + log[i].high;
    }
    total as f64 / log.len() as f64
}

fn hottest_day(log: &Vec<Reading>) -> usize {
    let mut best_index = 0;
    for i in 0..log.len() {
        if log[i].high > log[best_index].high {
            best_index = i;
        }
    }
    best_index
}
fn count_above(log: &Vec<Reading>, threshold: i32) -> usize {
    let mut count = 0;
    for i in 0..log.len() {
    if log[i].high> threshold {
         count=count+1;
	}
    }
 count
} 
fn summarize(log: &Vec<Reading>, threshold: i32) -> Summary {
    Summary {
        average: average_temp(log),
        hottest_day: log[hottest_day(log)].day.clone(),
        days_above: count_above(log, threshold),
    }
}
