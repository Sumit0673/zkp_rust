extern crate ark_bn254;
extern crate ark_ec;
extern crate ark_ff;
extern crate rand;

use ark_bn254::{Bn254, Fr, G1Projective, G2Projective, g1, g2};
use ark_ec::pairing::Pairing;  // Correct import for Pairing
use ark_ff::{Field, UniformRand};
use rand::thread_rng;

fn main() {
    println!("----------------------\nIntroduction\n----------------------");

    // Use the predefined generators for G1 and G2
    let g1 = g1::GENERATOR;  // G1 generator (predefined constant in ark_bn254)
    let g2 = g2::GENERATOR;  // G2 generator (predefined constant in ark_bn254)

    println!("G1: {:?}", g1);
    println!("G2: {:?}", g2);

    // G1 + G1 == G1 * 2
    let g1_add = g1 + g1;
    let g1_mult = g1.mul(2);  // Scalar multiplication
    println!("G1 + G1 == G1 * 2: {}", g1_add == g1_mult);

    // G2 + G2 == G2 * 2
    let g2_add = g2 + g2;
    let g2_mult = g2.mul(2);  // Scalar multiplication
    println!("G2 + G2 == G2 * 2: {}", g2_add == g2_mult);

    println!(
        "Addition with elements in different groups doesn't work: Compile-time type safety prevents this"
    );

    // G1 + G1 + G1 == G1 * 3
    let g1_add_triple = g1 + g1 + g1;
    let g1_mult_triple = g1.mul(3);  // Scalar multiplication
    println!("G1 + G1 + G1 == G1 * 3: {}", g1_add_triple == g1_mult_triple);

    println!("\n----------------------\nPairing\n----------------------");

    // Pairing example
    let scalar_a = Fr::from(5u64);
    let scalar_b = Fr::from(6u64);

    let a = g2.mul(scalar_a);  // Scalar multiplication
    let b = g1.mul(scalar_b);  // Scalar multiplication

    let pairing_ab = Bn254::pairing(a, b);

    println!("Pairing e(5*G2, 6*G1): {:?}", pairing_ab);

    let scalar_c = scalar_a * scalar_b;
    let c = g2.mul(scalar_c);  // Scalar multiplication

    let pairing_c_g1 = Bn254::pairing(c, g1);

    println!(
        "e(5*G2, 6*G1) == e(5*6*G2, G1): {}",
        pairing_ab == pairing_c_g1
    );
}
