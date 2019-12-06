fn head_tail(s: &str) -> (&str, i64) {
  let (head, tail) = match s.chars().next() {
    Some(c) => s.split_at(c.len_utf8()),
    None => s.split_at(0),
  };

  match tail.parse::<i64>() {
    Ok(num) => (head, num),
    Err(_) => (head, i64::max_value()),
  }
}

fn main() {
  let wires = std::fs::read_to_string("input.txt").expect("Error reading file");
  let mut visited_locations: std::collections::HashMap<(i64, i64), (usize, u64)> = std::collections::HashMap::new();
  let mut intersections: Vec<(i64, i64, u64)> = Vec::new();

  for (wire_num, wire) in wires.lines().enumerate() {
    let positions = wire.split(',').map(|s| head_tail(s));
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut num_steps = 0;
    for pos in positions {
      let (direction, distance) = pos;
      let movement: Box<dyn Fn(i64, i64) -> (i64, i64)> = match direction {
        "R" => Box::new(|x, y| (x + 1, y)),
        "D" => Box::new(|x, y| (x, y - 1)),
        "L" => Box::new(|x, y| (x - 1, y)),
        "U" => Box::new(|x, y| (x, y + 1)),
        _ => return,
      };
      for _ in 0..distance {
        let (new_x, new_y) = movement(x, y);
        x = new_x;
        y = new_y;
        num_steps += 1;
        match visited_locations.get(&(new_x, new_y)) {
          Some(visited_location) => {
            let (_wire_num, _num_steps) = visited_location;
            if _wire_num != &wire_num {
              // Get the total number of steps
              intersections.push((new_x, new_y, _num_steps + num_steps));
            }
          },
          None => {},
        }
        visited_locations.insert((new_x, new_y), (wire_num, num_steps));
      }
    }
  }

  let mut min_manhattan_distance = i64::max_value();
  let mut min_steps = u64::max_value();
  for intersection in &intersections {
    let (x, y, num_steps) = intersection;
    let manhattan_distance = x.abs() + y.abs();
    if manhattan_distance < min_manhattan_distance {
      min_manhattan_distance = manhattan_distance;
    }
    if num_steps < &min_steps {
      min_steps = *num_steps;
    }
  }

  println!("min_manhattan_distance: {}, min_steps: {}", min_manhattan_distance, min_steps);
}
