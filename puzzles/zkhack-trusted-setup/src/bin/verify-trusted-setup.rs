use ark_bls12_381::Fr;
use ark_ec::AffineCurve;
use prompt::{puzzle, welcome};
use std::str::FromStr;
use trusted_setup::data::puzzle_data;
use trusted_setup::PUZZLE_DESCRIPTION;

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (_ts1, _ts2) = puzzle_data();

    // ANCHOR: check
    // --snip--

    // check that G1 and G2 are not in the correct group
    if _ts1[0].is_on_curve() {
        println!("G1 is on the curve.");
    } else {
        println!("G1 is not on the curve.");
    }

    if _ts1[0].is_in_correct_subgroup_assuming_on_curve() {
        println!("G1 is in the correct subgroup.");
    } else {
        println!("G1 is not in the correct subgroup.");
    }

    if _ts2[0].is_on_curve() {
        println!("G2 is on the curve.");
    } else {
        println!("G2 is not on the curve.");
    }

    if _ts2[0].is_in_correct_subgroup_assuming_on_curve() {
        println!("G2 is in the correct subgroup.");
    } else {
        println!("G2 is not in the correct subgroup.");
    }
    // ANCHOR_END: check

    // printing points to copy-paste them in sage script
    println!("G1 = {}", _ts1[0]);
    println!("s * G1 = {}", _ts1[1]);
    println!("G2 = {}", _ts2[0]);
    println!("s * g2 = {}", _ts2[1]);

    /* Your solution here! (s in decimal)*/
    let s = Fr::from_str("114939083266787167213538091034071020048").unwrap();
    println!("Checking the solution...");
    assert_eq!(_ts1[0].mul(s), _ts1[1]);
    assert_eq!(_ts2[0].mul(s), _ts2[1]);
    println!("It works!");
}
