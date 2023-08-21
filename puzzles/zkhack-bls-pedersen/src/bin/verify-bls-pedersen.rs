use bls_pedersen::bls::verify;
use bls_pedersen::data::puzzle_data;
use bls_pedersen::PUZZLE_DESCRIPTION;
use prompt::{puzzle, welcome};

// additional items brought into scope for puzzle solving
use bls_pedersen::hash::hash_to_curve;
use ark_crypto_primitives::crh::pedersen::bytes_to_bits;
use ark_ff::{PrimeField, Zero};
use ark_ec::{AffineCurve, ProjectiveCurve, msm::VariableBaseMSM};
use ark_bls12_381::{Fr, G1Affine, G1Projective};
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (pk, ms, sigs) = puzzle_data();
    for (m, sig) in ms.iter().zip(sigs.iter()) {
        verify(pk, m, *sig);
    }
    
    // ANCHOR: write
    // --snip--
    
    let m = b"your_username";

    let mut data_file = File::create("sage/data.sage").unwrap();

    // write matrix M to be passed to SAGE
    data_file.write_all("M = [".as_bytes()).unwrap();
    for m in ms {
        let (hash, _) = hash_to_curve(&m);
        let bits = bytes_to_bits(&hash);
        let bits: Vec<u8> = bits.into_iter().map(|x| x as u8).collect();
        let line = format!("{:?}, ", bits);
        data_file.write_all(line.as_bytes()).unwrap();
    }
    data_file.write_all("]\n".as_bytes()).unwrap();

    // write vector h to be passed to SAGE
    data_file.write_all("h = ".as_bytes()).unwrap();
    let (hash, _) = hash_to_curve(m);
    let bits = bytes_to_bits(&hash);
    let bits: Vec<u8> = bits.into_iter().map(|x| x as u8).collect();
    let line = format!("{:?}", bits);
    data_file.write_all(line.as_bytes()).unwrap();
    // ANCHOR_END: write

    // ANCHOR: read
    // --snip--
    
    // read solution in coeffs.txt and cast these strings (one per line) into scalar field Fr elements
    let mut coeffs = Vec::new();
    for line in fs::read_to_string("sage/coeffs.txt").unwrap().lines() {
        // let c = Fr::from_le_bytes_mod_order(line.as_bytes()); // doesn't work
        let c: Fr = line.parse().unwrap();
        coeffs.push(c);
    }
    // ANCHOR_END: read
    
    // ANCHOR: forge
    // --snip--
    
    // compute forgery using affine coordinates
    let mut aff_forge = G1Affine::zero();
    for (c, sig) in coeffs.iter().zip(sigs.iter()) {
        aff_forge = aff_forge + sig.mul(*c).into();
    }
    
    // compute forgery using projective coordinates
    let mut proj_forge = G1Projective::zero();
    for (c, sig) in coeffs.iter().zip(sigs.iter()) {
        proj_forge += sig.mul(*c);
    }
    
    // compute forgery using multi-scalar multiplication
    let coeffs: Vec<<Fr as PrimeField>::BigInt> = 
        coeffs.iter().map(|c| (*c).into_repr()).collect();
    let msm_forge = VariableBaseMSM::multi_scalar_mul(&sigs, &coeffs);
        
    /* Your solution here! */

    verify(pk, m, aff_forge);
    verify(pk, m, proj_forge.into_affine());
    verify(pk, m, msm_forge.into_affine());
    println!("Puzzle solved!");
    //ANCHOR_END: forge
}
