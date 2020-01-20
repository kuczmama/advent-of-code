fn fuel_needed(mass: i64) -> i64 {
  let mut fuel = mass / 3 - 2;
  let mut sum = fuel;
  loop {
    fuel = fuel / 3 - 2;
    if fuel <= 0 {
      break;
    }
    sum += fuel;
  }
  sum
}

fn main() {
  let sum: i64 = include_str!("input.txt")
  .lines()
  .filter_map(|x| x.parse().ok())
  .map(|mass| fuel_needed(mass))
  .sum();
  println!("{}", sum);
}