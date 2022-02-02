//Copyright (c) Microsoft Corporation. All rights reserved.
//Highly Confidential Material

use rand::Rng;
use std::cmp;
use std::fmt;

pub struct Round {
    pub selection: Vec<u32>,
    pub target: u32,
}

#[derive(Clone)]
pub enum Operation {
    Plus,
    Minus,
    Times,
    Divide,
}

#[derive(Clone)]
pub struct Step {
    pub num_1: u32,
    pub num_2: u32,
    pub op: fn(u32, u32) -> Result<u32, &'static str>,
    pub op_enum: Operation,
}

pub struct Solution {
    pub method: Vec<Step>,
    pub solved: bool,
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
            target: rand::thread_rng().gen_range(101..999),
        }
    }

    pub fn new_spec(selection: Vec<u32>, target: u32) -> Round {
        Round { selection, target }
    }

    pub fn solve(&self) -> Solution {
        let mut best_solution = Solution {
            method: vec![],
            solved: false,
        };
        // Loop through all combinations of numbers from the selection
        for (i, num_1) in self.selection.iter().enumerate() {
            for (j, num_2) in self.selection[i + 1..].iter().enumerate() {
                for op in [
                    Operation::Plus,
                    Operation::Minus,
                    Operation::Times,
                    Operation::Divide,
                ] {
                    // Put the largest number first in the step, so that subtraction and division (may) work
                    let step = Step::new(cmp::max(*num_1, *num_2), cmp::min(*num_1, *num_2), op);
                    match step.output() {
                        // If this step is invalid, move on
                        Err(_) => {
                            continue;
                        }
                        Ok(number) => {
                            // If we've solved it, this is the final step of the solution
                            if number == self.target {
                                return Solution {
                                    method: vec![step],
                                    solved: true,
                                };
                            } else if self.selection.len() > 2 {
                                // We haven't solved it yet, but this may be a step on the way. Create a new
                                // round replacing this step's inputs with its output, and iterate.
                                let mut new_sel = Vec::new();
                                new_sel.extend_from_slice(&self.selection[0..i]);
                                new_sel.extend_from_slice(&self.selection[i + 1..i + 1 + j]);
                                new_sel.extend_from_slice(&self.selection[i + 1 + j + 1..]);
                                new_sel.push(number);

                                let new_round = Round {
                                    selection: new_sel,
                                    target: self.target,
                                };

                                let sol = new_round.solve();

                                // If we now have a solution and it's the best so far, add this step to the start of the list, and
                                // store it off.
                                if sol.solved
                                    && (!best_solution.solved
                                        || sol.method.len() < best_solution.method.len())
                                {
                                    let mut new_method = vec![step];
                                    new_method.append(&mut sol.method.clone());

                                    best_solution = Solution {
                                        method: new_method,
                                        solved: sol.solved,
                                    };
                                }
                            }
                        }
                    }
                }
            }
        }
        best_solution
    }
}

impl Step {
    fn new(num_1: u32, num_2: u32, op_enum: Operation) -> Step {
        let op: fn(u32, u32) -> Result<u32, &'static str> = match op_enum {
            Operation::Plus => |x, y| Ok(x + y),
            Operation::Minus => |x, y| Ok(x - y),
            Operation::Times => |x, y| Ok(x * y),
            Operation::Divide => |x, y| {
                if y != 0 && x % y == 0 {
                    Ok(x / y)
                } else {
                    Err("Cannot divide")
                }
            },
        };
        Step {
            num_1,
            num_2,
            op,
            op_enum,
        }
    }

    fn output(&self) -> Result<u32, &'static str> {
        (self.op)(self.num_1, self.num_2)
    }

    fn output_string(&self) -> String {
        let number = self.output().unwrap();
        number.to_string()
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
            Operation::Divide => write!(f, "/")?,
        }
        Ok(())
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}={}",
            self.num_1,
            self.op_enum,
            self.num_2,
            self.output_string()
        )
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.solved {
            for step in &self.method {
                writeln!(f, "{}", step)?
            }
        } else {
            write!(f, "impossible")?
        }
        Ok(())
    }
}

#[test]
fn easy_solve() {
    let round = Round::new_spec(vec![100, 50], 150);
    let soln = round.solve();
    assert!(soln.solved);
    assert!(soln.method[0].num_1 == 100);
    assert!(soln.method[0].num_2 == 50);
    assert!(matches!(soln.method[0].op_enum, Operation::Plus));
}

#[test]
fn no_solve() {
    let round = Round::new_spec(vec![1, 1, 2, 2, 3, 3], 100);
    let soln = round.solve();
    assert!(!soln.solved);
    assert_eq!(soln.method.len(), 0);
}

#[test]
fn no_selection() {
    let round = Round::new_spec(vec![], 100);
    let soln = round.solve();
    assert!(!soln.solved);
    assert_eq!(soln.method.len(), 0);
}

#[test]
fn one_num_selection() {
    let round = Round::new_spec(vec![1], 100);
    let soln = round.solve();
    assert!(!soln.solved);
    assert_eq!(soln.method.len(), 0);
}

#[test]
fn multi_step_solve() {
    let round = Round::new_spec(vec![1, 1, 2, 2, 3, 3], 81);
    let soln = round.solve();
    assert!(soln.solved);
    assert_eq!(soln.method.len(), 5);
    println!("{}", soln);
}

#[test]
fn generate_4l_round() {
    let round = Round::new(4);
    assert!([25, 50, 75, 100].contains(&round.selection[0]));
    assert!([25, 50, 75, 100].contains(&round.selection[1]));
    assert!([25, 50, 75, 100].contains(&round.selection[2]));
    assert!([25, 50, 75, 100].contains(&round.selection[3]));
    assert!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].contains(&round.selection[4]));
    assert!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].contains(&round.selection[5]));
    assert!(round.target >= 100 && round.target <= 999);
}

#[test]
fn generate_3l_round() {
    let round = Round::new(3);
    assert!([25, 50, 75, 100].contains(&round.selection[0]));
    assert!([25, 50, 75, 100].contains(&round.selection[1]));
    assert!([25, 50, 75, 100].contains(&round.selection[2]));
    assert!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].contains(&round.selection[3]));
    assert!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].contains(&round.selection[4]));
    assert!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].contains(&round.selection[5]));
    assert!(round.target >= 100 && round.target <= 999);
}

#[test]
fn generate_2l_round() {
    let round = Round::new(2);
    assert!([25, 50, 75, 100].contains(&round.selection[0]));
    assert!([25, 50, 75, 100].contains(&round.selection[1]));
    assert!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].contains(&round.selection[2]));
    assert!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].contains(&round.selection[3]));
    assert!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].contains(&round.selection[4]));
    assert!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].contains(&round.selection[5]));
    assert!(round.target >= 100 && round.target <= 999);
}

#[test]
fn generate_1l_round() {
    let round = Round::new(1);
    assert!([25, 50, 75, 100].contains(&round.selection[0]));
    assert!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].contains(&round.selection[1]));
    assert!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].contains(&round.selection[2]));
    assert!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].contains(&round.selection[3]));
    assert!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].contains(&round.selection[4]));
    assert!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].contains(&round.selection[5]));
    assert!(round.target >= 100 && round.target <= 999);
}

#[test]
fn generate_0l_round() {
    let round = Round::new(0);
    assert!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].contains(&round.selection[0]));
    assert!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].contains(&round.selection[1]));
    assert!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].contains(&round.selection[2]));
    assert!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].contains(&round.selection[3]));
    assert!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].contains(&round.selection[4]));
    assert!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].contains(&round.selection[5]));
    assert!(round.target >= 100 && round.target <= 999);
}

#[test]
#[should_panic(expected = "I said between 0 and 4!")]
fn generate_5l_round() {
    Round::new(5);
}

#[test]
fn check_shortest_method() {
    let round = Round::new_spec(vec![100, 75, 50, 25, 2, 2], 200);
    let soln = round.solve();
    assert!(soln.solved);
    assert_eq!(soln.method.len(), 1);
    assert!(soln.method[0].num_1 == 100);
    assert!(soln.method[0].num_2 == 2);
    assert!(matches!(soln.method[0].op_enum, Operation::Times));
    println!("{}", round);
}

#[test]
fn invalid_divide_coverage() {
    let round = Round::new_spec(vec![100, 75], 200);
    let soln = round.solve();
    assert!(!soln.solved);
    assert_eq!(soln.method.len(), 0);
    println!("{}", soln);
}

#[test]
fn minus_divide_coverage() {
    let round = Round::new_spec(vec![100, 25, 3], 1);
    let soln = round.solve();
    assert_eq!(soln.method.len(), 2);
    println!("{}", soln);
}
