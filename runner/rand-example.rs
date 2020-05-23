//: -s
use rand::prelude::*;

let mut rng = rand::thread_rng();
  let y: f64 = rng.gen();
  println!("y = {}", y);

  let mut nums: Vec<i32> = (1..100).collect();
  nums.shuffle(&mut rng);
  println!("nums = {:?}", nums);