use std::io::{stdout, stdin, Write};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::sync::mpsc::{self, TryRecvError};

fn main() {
    println!("How many larges would you like? (0-4)");

    let mut larges = String::new();
    stdin().read_line(&mut larges).expect("Failed to read line");

    let larges: u32 = larges.trim().parse().expect("Please input a number next time");

    let round = countdown::Round::new(larges);
    println!("Your round is: {}", round);

    let handle = setup_threads();

    let solution = round.solve();

    handle.join().unwrap();
    println!("\nSolution is:\n{}", solution);
}

fn setup_threads() -> JoinHandle<()> {
	let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
    	println!("Press enter to end round early");
    	let mut input = String::new();
	    stdin().read_line(&mut input).expect("Failed to read line");
    	let _ = tx.send(());
    });

    thread::spawn(move || {
        for i in 0..30001 {
        	if i % 1000 == 0 {
	            print!("\r{} ", 30-i/1000);
	            stdout().flush().unwrap();
        	}
            thread::sleep(Duration::from_millis(1));

	        match rx.try_recv() {
	            Ok(_) | Err(TryRecvError::Disconnected) => {
	                break;
	            }
	            Err(TryRecvError::Empty) => {}
	        }
        }
    })
}
