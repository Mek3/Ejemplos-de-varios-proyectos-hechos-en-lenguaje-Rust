use rand::prelude::*;

fn main()
{
	if rand::random() { // generates a boolean
		// Try printing a random unicode code point (probably a bad idea)!
		println!("char: {}", rand::random::<char>());
	}

	let mut rng = rand::thread_rng();
	
	let mut nums: Vec<i32> = (1..100).collect();
	nums.shuffle(&mut rng);
}

