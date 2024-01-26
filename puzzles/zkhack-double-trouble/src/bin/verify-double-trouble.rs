#![allow(unused, unreachable_code)]
use ark_ed_on_bls12_381::Fr;
use ark_ff::Field;
use double_trouble::data::puzzle_data;
use double_trouble::inner_product_argument::utils::challenge;
use double_trouble::verify;
use double_trouble::PUZZLE_DESCRIPTION;
use prompt::{puzzle, welcome};

// additional items brought into scope for puzzle solving
use double_trouble::CommitKey;
use ark_ec::AffineCurve;

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (ck, [instance_and_proof_1, instance_and_proof_2]) = puzzle_data();
    let (instance1, proof1) = instance_and_proof_1;
    let (instance2, proof2) = instance_and_proof_2;
    assert!(verify(&ck, &instance1, &proof1));
    assert!(verify(&ck, &instance2, &proof2));

    // ANCHOR: print
    // --snip--

    println!("commitment key:");
    for (i, ck_i) in ck.generators.iter().enumerate() {
        println!("ck.generators[{}] = {}", i, ck_i);
    }
    println!("ck.hiding_generator = {}\n", ck.hiding_generator);
    println!("instance 1, C_a:\n {}\n", instance1.comm_a);
    println!("instance 2, C_a:\n {}\n", instance2.comm_a);
    println!("instance 1, b:");
    for (i, b_i) in instance1.b.iter().enumerate() {
        println!("instance1.b[{}] = {}", i, b_i);
    }
    println!("");
    println!("instance 2, b:");
    for (i, b_i) in instance2.b.iter().enumerate() {
        println!("instance2.b[{}] = {}", i, b_i);
    }
    println!("");
    // ANCHOR_END: print

    // ANCHOR: check
    // --snip--

    assert_eq!(instance1, instance2);
    assert_eq!(ck, CommitKey::sample(8));
    // ANCHOR_END: check

    // ANCHOR: commitments
    // --snip--

    println!("proof1, comm_r:\n {}", proof1.commitment.comm_r);
    println!("proof1, comm_1:\n {}", proof1.commitment.comm_1);
    println!("proof1, comm_2:\n {}\n", proof1.commitment.comm_2);

    println!("proof2, comm_r:\n {}", proof2.commitment.comm_r);
    println!("proof2, comm_1:\n {}", proof2.commitment.comm_1);
    println!("proof2, comm_2:\n {}\n", proof2.commitment.comm_2);
    // ANCHOR_END: commitments

    // ANCHOR: double
    // --snip--

    if proof1.commitment.comm_r.mul(2) == proof2.commitment.comm_r {
        println!("C_r in the second proof is twice C_r in the first proof\n");
    }
    // ANCHOR_END: double

    // ANCHOR: solve
    // --snip--

    let gamma1 = challenge(&ck, &instance1, &proof1.commitment);
    let gamma2 = challenge(&ck, &instance2, &proof2.commitment);
    let s1 = proof1.response.s;
    let s2 = proof2.response.s;
    let u1 = proof1.response.u;
    let u2 = proof2.response.u;
    let k = (gamma1 - Fr::from(2) * gamma2).inverse().unwrap();
    let my_a: Vec<Fr> = s1
        .iter()
        .zip(s2.iter())
        .map(|(c1, c2)| k * (gamma1 * c2 - Fr::from(2) * gamma2 * c1))
        .collect();
    let my_comm_a_rand = k * (gamma1 * u2 - Fr::from(2) * gamma2 * u1);

    let (a, comm_a_rand): (Vec<Fr>, Fr) = {
        // Your solution here!
        (my_a, my_comm_a_rand)
    };
    assert_eq!(
        ck.commit_with_explicit_randomness(&a, comm_a_rand),
        instance1.comm_a
    );
    assert_eq!(
        ck.commit_with_explicit_randomness(&a, comm_a_rand),
        instance2.comm_a
    );
    println!("Puzzle solved!");
    // ANCHOR_END: solve
}
