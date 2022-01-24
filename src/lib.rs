use rand::Rng;
use std::fmt;
use std::cmp;

pub struct Round {
	pub selection: Vec<u32>,
	pub target: u32
}

#[derive(Clone)]
pub enum Operation {
	Plus,
	Minus,
	Times,
	Divide
}

#[derive(Clone)]
pub struct Step {
	pub num_1: u32,
	pub num_2: u32,
	pub op: Operation
}

pub struct Solution {
	pub method: Vec<Step>,
	pub solved: bool
}

impl Round {
	pub fn new(num_larges: u32) -> Round {
		if num_larges > 4 {
			panic!("I said between 0 and 4!");
		}

		let mut selection = Vec::new();

		let mut large_pot = vec![25, 50, 75, 100];
		let mut small_pot = vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10];

		for _ in 0..num_larges {
			selection.push(large_pot.remove(rand::thread_rng().gen_range(0..large_pot.len())));
		}

		for _ in num_larges..6 {
			selection.push(small_pot.remove(rand::thread_rng().gen_range(0..small_pot.len())));
		}

		Round {
			selection,
			target: rand::thread_rng().gen_range(101..999)
		}
	}

	pub fn new_spec(selection: Vec<u32>, target: u32) -> Round {
		Round {selection, target}
	}

	pub fn solve(&self) -> Solution {
		// Loop through all combinations of numbers from the selection
		for (i, num_1) in self.selection.iter().enumerate() {
	        for (j, num_2) in self.selection[i + 1..].iter().enumerate() {
	            for op in [Operation::Plus, Operation::Minus, Operation::Times, Operation::Divide] {
	            	// Put the largest number first in the step, so that subtraction and division (may) work
	            	let step = Step {num_1: cmp::max(*num_1, *num_2), num_2: cmp::min(*num_1, *num_2), op: op};
	            	match step.output() {
	            		// If this step is invalid, move on
	            		Err(_) =>  { continue; }
	            		Ok(number) => {
	            			// If we've solved it, this is the final step of the solution
	            			if number == self.target {
	            				return Solution { method: vec![step], solved: true };
	            			}
	            			else if self.selection.len() > 2 {
	            				// We haven't solved it yet, but this may be a step on the way. Create a new
	            				// round replacing this step's inputs with its output, and iterate.
	            				let mut new_sel = Vec::new();
	            				new_sel.extend_from_slice(&self.selection[0..i]);
	            				new_sel.extend_from_slice(&self.selection[i+1..i+1+j]);
	            				new_sel.extend_from_slice(&self.selection[i+1+j+1..]);
	            				new_sel.push(number);

	            				let new_round = Round { selection: new_sel, target: self.target };

	            				let sol = new_round.solve();

	            				// If we new have a solution, add this step to the start of the list, and carry
	            				// on returning.
	            				if sol.solved {
	            					let mut new_method = Vec::new();

		            				new_method.push(step);
		            				new_method.append(&mut sol.method.clone());

	            					return Solution { method: new_method, solved: sol.solved };
		            			}
	            			}
	            		}
	            	}
	            }
	        }
	    }
	    Solution { method: vec![], solved: false }
	}
}

impl Step {
	fn output(&self) -> Result<u32, &'static str> {
		match self.op {
			Operation::Plus => Ok(self.num_1 + self.num_2),
			Operation::Minus => if self.num_1 > self.num_2 { Ok(self.num_1 - self.num_2) } else { Err("Cannot subtract") },
			Operation::Times => Ok(self.num_1 * self.num_2),
			Operation::Divide => if self.num_1 % self.num_2 == 0 { Ok(self.num_1 / self.num_2) } else { Err("Cannot divide") }
		}
	}

	fn output_string(&self) -> String {
		match self.output() {
			Ok(number) => number.to_string(),
			Err(e) => String::from(e)
		}
	}
}

// Display formats for classes
impl fmt::Display for Round {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	for number in &self.selection {
    		write!(f, "{} ", number)?
    	}
    	write!(f, "with a target of {}", self.target)
    }
}

impl fmt::Display for Operation {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Operation::Plus => write!(f, "+")?,
			Operation::Minus => write!(f, "-")?,
			Operation::Times => write!(f, "*")?,
			Operation::Divide => write!(f, "/")?
		}
		Ok(())
    }
}

impl fmt::Display for Step {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}{}{}={}", self.num_1, self.op, self.num_2, self.output_string())
	}
}

impl fmt::Display for Solution {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.solved {
			for step in &self.method {
				write!(f, "{}\n", step)?
			}
		}
		else {
			write!(f, "impossible")?
		}
		Ok(())
    }
}