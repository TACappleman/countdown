#[test]
fn easy_solve() {
    let round = countdown::Round::new_spec(vec![100, 50], 150);
    let soln = round.solve();
    assert_eq!(soln.solved, true);
    assert!(soln.method[0].num_1 == 100);
    assert!(soln.method[0].num_2 == 50);
    assert!(matches!(soln.method[0].op, countdown::Operation::Plus));
}

#[test]
fn no_solve() {
    let round = countdown::Round::new_spec(vec![1, 1, 2, 2, 3, 3], 100);
    let soln = round.solve();
    assert_eq!(soln.solved, false);
    assert_eq!(soln.method.len(), 0);
}

#[test]
fn no_selection() {
    let round = countdown::Round::new_spec(vec![], 100);
    let soln = round.solve();
    assert_eq!(soln.solved, false);
    assert_eq!(soln.method.len(), 0);
}

#[test]
fn one_num_selection() {
    let round = countdown::Round::new_spec(vec![1], 100);
    let soln = round.solve();
    assert_eq!(soln.solved, false);
    assert_eq!(soln.method.len(), 0);
}

#[test]
fn multi_step_solve() {
    let round = countdown::Round::new_spec(vec![1, 1, 2, 2, 3, 3], 81);
    let soln = round.solve();
    assert_eq!(soln.solved, true);
    assert_eq!(soln.method.len(), 5);
}

#[test]
fn generate_4l_round() {
    let round = countdown::Round::new(4);
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
    let round = countdown::Round::new(3);
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
    let round = countdown::Round::new(2);
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
    let round = countdown::Round::new(1);
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
    let round = countdown::Round::new(0);
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
    countdown::Round::new(5);
}

#[test]
fn check_shortest_method() {
    let round = countdown::Round::new_spec(vec![100, 75, 50, 25, 2, 2], 200);
    let soln = round.solve();
    assert_eq!(soln.solved, true);
    assert_eq!(soln.method.len(), 1);
    assert!(soln.method[0].num_1 == 100);
    assert!(soln.method[0].num_2 == 2);
    assert!(matches!(soln.method[0].op, countdown::Operation::Times));
}
