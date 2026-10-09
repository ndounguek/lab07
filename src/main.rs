use rand::Rng;

const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

fn main() {
   let mut  highs: Vec<i32> = vec![72, 68, 75, 81, 79];

for i in 0..highs.len() {
println! ("{}: {}", DAYS[i],highs[i]);
}
 let mut rng= rand::rng();
highs.push(rng.random_range (60..=100));
highs.push(rng.random_range(60..=100));

for i in 0..highs.len() {
    println!("{}: {}", DAYS[i], highs[i]);
}
 println!("Average: {}", average_temp(&highs));
}
// fn name...{}
fn average_temp(log: &Vec<i32>) -> f64 {
    if log.is_empty() {
        return 0.0;
    }
let mut total=0; 
for i in 0..log.len(){
    total=total+log[i];
}
total as f64 / log.len() as f64
}
