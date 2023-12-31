use rand::{self}; 
use num_bigint::{BigUint,RandBigInt};
use sha256::digest; 

pub fn generate_random_num_in_range(lowerbound: &BigUint, upperbound: &BigUint) -> BigUint{
        let mut rng = rand::thread_rng(); 
        rng.gen_biguint_range(lowerbound, upperbound)
}

pub fn hash_biguint(val: &BigUint) -> String{
        digest(BigUint::to_bytes_le(val)) 
}

pub fn hex_to_biguint(hex: &str) -> BigUint{
        BigUint::parse_bytes(hex.replace(" ", "").as_bytes(), 16).expect("Invalid hex")
}
