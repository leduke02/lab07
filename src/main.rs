use rand::Rng;

const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

	fn average_temp(log: &Vec<i32>) -> f64 {
	if log.is empty() {
	return 0.0;
}
	let sum:i32 = log.iter().sum();
	sum as f64 / log.len() as f64
}
	fn hottest_day(log: &Vec<i32>) -> usize {
	let mut hottest = 0;

	for i in 1..log.len() {
	if log [i] > log[hottest] {
	hottest = i;
	}
}
	hottest
}
	fn count_above(log: Vec<i32>, threshold: i32) -> usize {
	log.iter()
	.filter(|&&temp| temp > threshold)
	.count()
}
fn main() {
    let mut highs: Vec<i32> = vec![72, 68, 75, 81, 79];

	//Print the original temperatures
	for i in 0..highs.len() {
	println!("{}: {}", DAYS[i], highs[i]);
}
	//Add two random temperatures from 60 to 100
	let mut rng = rand::rng();

	highs.push(rng.random_range(60..=100));
	highs.push(rng.random_range(60..=100));

	println!("\nFull list:");

	//Print the complete list with weekday labels
	for i in 0..
