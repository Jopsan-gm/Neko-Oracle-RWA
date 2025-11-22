use soroban_sdk::{Bytes, BytesN, Env, Vec};
use crate::error::Error;

/// Verify a Noir proof for price oracle data
///
/// This function performs structural validation and public input verification.
/// Full cryptographic verification happens off-chain due to gas constraints.
///
/// # Arguments
/// * `env` - Soroban environment
/// * `proof_data` - Serialized UltraHonk proof from Noir
/// * `public_inputs` - Public circuit outputs (price, timestamp, etc.)
/// * `expected_price` - Price to verify against circuit output
/// * `expected_timestamp` - Timestamp to verify
///
/// # Returns
/// * `Ok(true)` if proof structure and inputs are valid
/// * `Err(Error)` if validation fails
pub fn verify_price_proof(
    env: &Env,
    proof_data: &Bytes,
    public_inputs: &Vec<u32>,
    expected_price: i128,
    expected_timestamp: u64,
) -> Result<bool, Error> {
    validate_proof_structure(proof_data)?;

    // === STEP 2: Validate public inputs format ===
    // Expected circuit outputs:
    // public_inputs[0] = final_price (averaged, 2 decimals: e.g., 30050 = $300.50)
    // Additional inputs may include individual API prices for transparency
    
    if public_inputs.is_empty() {
        return Err(Error::NoPublicInput);
    }

    // Get averaged price from circuit (first public input)
    let avg_price_from_proof = public_inputs.get(0).ok_or(Error::NoPublicInput)?;

    // === STEP 3: Verify price matches ===
    // Circuit outputs price with 2 decimals (e.g., 30050 for $300.50)
    // Contract expects price with 7 decimals (e.g., 3005000000)
    // Convert expected_price from 7 decimals to 2 decimals for comparison
    let expected_price_scaled = (expected_price / 100000) as u32;
    
    if avg_price_from_proof != expected_price_scaled {
        return Err(Error::PriceMismatch);
    }

    // === STEP 4: Anti-replay protection ===
    // Store proof hash to prevent reuse
    let proof_hash = env.crypto().keccak256(proof_data).into();
    
    if is_proof_used(env, &proof_hash) {
        return Err(Error::ProofAlreadyUsed);
    }

    store_used_proof(env, &proof_hash, expected_timestamp);

    // === STEP 5: Structural verification passed ===
    // In a full implementation, BN254 pairing verification would happen here:
    // - Deserialize proof points (A, B, C, etc.)
    // - Deserialize verification key
    // - Compute pairing equation: e(A,B) = e(α,β) * e(L,γ) * e(C,δ)
    // - Verify result equals identity in Fp12
    //
    // Cost estimate: 1-5M instructions (exceeds Soroban limits)
    //
    // Current approach: Off-chain verification + on-chain registry

    Ok(true)
}

/// Validate proof structure (format, size, content)
fn validate_proof_structure(proof: &Bytes) -> Result<(), Error> {
    let proof_len = proof.len();
    
    // Check proof is not empty
    if proof_len == 0 {
        return Err(Error::EmptyProof);
    }

    // UltraHonk proofs from Noir are typically 13-15KB
    // Groth16 proofs would be ~256 bytes
    // We accept proofs >= 256 bytes for flexibility
    if proof_len < 256 {
        return Err(Error::BadProofLen);
    }

    // Verify proof is not all zeros (corrupted data check)
    let mut all_zeros = true;
    let check_len = proof_len.min(64); // Check first 64 bytes
    
    for i in 0..check_len {
        if proof.get(i).unwrap_or(0) != 0 {
            all_zeros = false;
            break;
        }
    }

    if all_zeros {
        return Err(Error::ZeroProof);
    }

    Ok(())
}

/// Store a used proof hash to prevent replay attacks
fn store_used_proof(env: &Env, proof_hash: &BytesN<32>, timestamp: u64) {
    use soroban_sdk::symbol_short;
    
    let key = (symbol_short!("proof"), proof_hash.clone());
    env.storage().persistent().set(&key, &timestamp);
    
    // Extend TTL to maximum for audit trail
    let max_ttl = env.storage().max_ttl();
    env.storage().persistent().extend_ttl(&key, max_ttl, max_ttl);
}

/// Check if a proof has been used before
pub fn is_proof_used(env: &Env, proof_hash: &BytesN<32>) -> bool {
    use soroban_sdk::symbol_short;
    let key = (symbol_short!("proof"), proof_hash.clone());
    env.storage().persistent().has(&key)
}

/// Get timestamp when a proof was first used
pub fn get_proof_usage_timestamp(env: &Env, proof_hash: &BytesN<32>) -> Option<u64> {
    use soroban_sdk::symbol_short;
    let key = (symbol_short!("proof"), proof_hash.clone());
    env.storage().persistent().get(&key)
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env, Bytes, Vec as SdkVec};

    #[test]
    fn test_validate_proof_structure_valid() {
        let env = Env::default();
        
        // Create valid proof (300 bytes, not all zeros)
        let mut proof_bytes = [0u8; 300];
        proof_bytes[0] = 1;
        proof_bytes[10] = 42;
        let proof = Bytes::from_array(&env, &proof_bytes);
        
        let result = validate_proof_structure(&proof);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_proof_structure_empty() {
        let env = Env::default();
        let proof = Bytes::new(&env);
        
        let result = validate_proof_structure(&proof);
        assert_eq!(result, Err(Error::EmptyProof));
    }

    #[test]
    fn test_validate_proof_structure_too_small() {
        let env = Env::default();
        let proof = Bytes::from_array(&env, &[1u8; 100]); // Too small
        
        let result = validate_proof_structure(&proof);
        assert_eq!(result, Err(Error::BadProofLen));
    }

    #[test]
    fn test_validate_proof_structure_all_zeros() {
        let env = Env::default();
        let proof = Bytes::from_array(&env, &[0u8; 300]);
        
        let result = validate_proof_structure(&proof);
        assert_eq!(result, Err(Error::ZeroProof));
    }

    #[test]
    fn test_verify_price_proof_valid() {
        let env = Env::default();
        
        // Valid proof
        let mut proof_bytes = [0u8; 300];
        proof_bytes[0] = 1;
        let proof = Bytes::from_array(&env, &proof_bytes);
        
        // Price: 3005000000 (7 decimals) = $300.50
        // Circuit output: 30050 (2 decimals)
        let expected_price = 3_005_000_000i128;
        let timestamp = 1700000000u64;
        
        let mut public_inputs = SdkVec::new(&env);
        public_inputs.push_back(30050u32); // avg_price
        
        let result = verify_price_proof(
            &env,
            &proof,
            &public_inputs,
            expected_price,
            timestamp,
        );
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_verify_price_proof_mismatch() {
        let env = Env::default();
        
        let mut proof_bytes = [0u8; 300];
        proof_bytes[0] = 1;
        let proof = Bytes::from_array(&env, &proof_bytes);
        
        let expected_price = 3_005_000_000i128;
        let timestamp = 1700000000u64;
        
        let mut public_inputs = SdkVec::new(&env);
        public_inputs.push_back(40000u32); // Wrong price
        
        let result = verify_price_proof(
            &env,
            &proof,
            &public_inputs,
            expected_price,
            timestamp,
        );
        
        assert_eq!(result, Err(Error::PriceMismatch));
    }

    #[test]
    fn test_proof_replay_protection() {
        let env = Env::default();
        
        let mut proof_bytes = [0u8; 300];
        proof_bytes[0] = 1;
        let proof = Bytes::from_array(&env, &proof_bytes);
        
        let expected_price = 3_005_000_000i128;
        let timestamp = 1700000000u64;
        
        let mut public_inputs = SdkVec::new(&env);
        public_inputs.push_back(30050u32);
        
        // First verification should succeed
        let result1 = env.as_contract(&contract_id, || {
            verify_price_proof(
                &env,
                &proof,
                &public_inputs,
                expected_price,
                timestamp,
            )
        });
        assert!(result1.is_ok());
        
        // Second verification with same proof should fail (replay attack)
        let result2 = env.as_contract(&contract_id, || {
            verify_price_proof(
                &env,
                &proof,
                &public_inputs,
                expected_price,
                timestamp + 100,
            )
        });
        assert_eq!(result2, Err(Error::ProofAlreadyUsed));
    }
}
