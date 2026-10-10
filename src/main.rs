use rand::Rng;

const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

fn main() {
    let mut highs: Vec<i32> = vec![72, 68, 75, 81, 79];

    for i in 0..highs.len() {
        println!("{}: {}", DAYS[i], highs[i]);
    }

    let mut rng = rand::rng();
    highs.push(rng.random_range(60..=100));
    highs.push(rng.random_range(60..=100));

    for i in 0..highs.len() {
        println!("{}: {}", DAYS[i], highs[i]);
    }
    println!("Average: {}", average_temp(&highs));
    let day = hottest_day(&highs);
    println!("Hottest day: {}", DAYS[day]);
println!("Days above 75: {}", count_above(&highs, 75));
}

fn average_temp(log: &Vec<i32>) -> f64 {
    if log.is_empty() {
        return 0.0;
    }
    let mut total = 0;
    for i in 0..log.len() {
        total = total + log[i];
    }
    total as f64 / log.len() as f64
}

fn hottest_day(log: &Vec<Reading>) -> usize {
    let mut best_index = 0;
    for i in 0..log.len() {
        if log[i] > log[best_index].high {
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
