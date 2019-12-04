// --- Day 3: Crossed Wires ---
// The gravity assist was successful, and you're well on your way to the Venus refuelling station. 
//During the rush back on Earth, the fuel management system wasn't completely installed, so that's next on the priority list.

// Opening the front panel reveals a jumble of wires. Specifically, two wires are connected to a central 
//port and extend outward on a grid. You trace the path each wire takes as it leaves the central port, 
// one wire per line of text (your puzzle input).

// The wires twist and turn, but the two wires occasionally cross paths. To fix the circuit, 
//you need to find the intersection point closest to the central port. Because the wires are on a grid,
// use the Manhattan distance for this measurement. While the wires do technically cross right at 
// the central port where they both start, this point does not count, nor does a wire count as crossing with itself.

// For example, if the first wire's path is R8,U5,L5,D3, then starting from the central port (o),
// it goes right 8, up 5, left 5, and finally down 3:

// ...........
// ...........
// ...........
// ....+----+.
// ....|....|.
// ....|....|.
// ....|....|.
// .........|.
// .o-------+.
// ...........
// Then, if the second wire's path is U7,R6,D4,L4, it goes up 7, right 6, down 4, and left 4:

// ...........
// .+-----+...
// .|.....|...
// .|..+--X-+.
// .|..|..|.|.
// .|.-X--+.|.
// .|..|....|.
// .|.......|.
// .o-------+.
// ...........
// These wires cross at two locations (marked X), but the lower-left one is closer to the central port: its distance is 3 + 3 = 6.

// Here are a few more examples:

// R75,D30,R83,U83,L12,D49,R71,U7,L72
// U62,R66,U55,R34,D71,R55,D58,R83 = distance 159
// R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
// U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = distance 135
// What is the Manhattan distance from the central port to the closest intersection?

// To begin, get yo

fn head_tail(s: &str) -> (&str, i64) {
  let (head, tail) = match s.chars().next() {
    Some(c) => s.split_at(c.len_utf8()),
    None => s.split_at(0),
  };

  match tail.parse::<i64>() {
    Ok(num) => (head, num),
    Err(_) => (head, i64::max_value())
  }
}

fn main() {
  let wires = std::fs::read_to_string("input.txt").expect("Error reading file");
  let mut visited_locations = std::collections::HashMap::new();
  let mut intersections: Vec<(i64, i64)> = Vec::new();
  let mut min = i64::max_value();

  for (i, wire) in wires.lines().enumerate() {
   let positions = wire
   .split(',')
   .map(|s| head_tail(s));
    let mut x = 0;
    let mut y = 0;
   for pos in positions {
     let (direction, distance) = pos;
     match direction {
       "R" => {
         for _ in 0..distance {
           x += 1;
           match visited_locations.get(&(x, y)) {
            Some(_) => intersections.push((x, y)),
            _ => {},
           }
           if i == 0 {
            visited_locations.insert((x, y), true);
           }
         }
       },
       "D" => {
         for _ in 0..distance {
           y -= 1;
           match visited_locations.get(&(x, y)) {
            Some(_) => intersections.push((x, y)),
            _ => {},
           }
           if i == 0 {
            visited_locations.insert((x, y), true);
           }
         }
       },
       "L" => {
         for _ in 0..distance {
           x -= 1;
           match visited_locations.get(&(x, y)) {
            Some(_) => intersections.push((x, y)),
            _ => {},
           }
           if i == 0 {
            visited_locations.insert((x, y), true);
           }
         }
       },
       "U" => {
         for _ in 0..distance {
           y += 1;
           match visited_locations.get(&(x, y)) {
            Some(_) => intersections.push((x, y)),
            _ => {},
           }
           if i == 0 {
            visited_locations.insert((x, y), true);
           }
         }
       },
       uhh => panic!("unable to parse: {}", uhh),
     }

   }
  }

  for intersection in &intersections {
    let (x, y)  = intersection;
    let manhattan_distance = x.abs() + y.abs();
    if manhattan_distance < min {
      min = manhattan_distance;
      println!("min: {}", min);
    }
  }

  println!("min manhattan distance: {}", min);
}