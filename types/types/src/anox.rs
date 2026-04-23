//! Anox-specific transaction types (MintTx, BurnTx).
//! Added to cuprate-types for the Anox fork.
//! TxIn/TxOut are represented as raw bytes pending integration with Cuprate's spend types.

/// Bridge burn transaction — destroys native tokens and requests minting on Base.
pub struct BurnTransaction {
    /// Private inputs proving ownership of the funds being burned (raw bytes, Phase 2 will type these).
    pub inputs: Vec<u8>,
    /// Optional private change outputs back to the sender (raw bytes, Phase 2 will type these).
    pub change_outputs: Vec<u8>,
    /// Amount to destroy on the chain and mint on Base.
    pub burn_amount: u64,
    /// Base recipient encoded explicitly in the transaction.
    pub base_recipient: [u8; 20],
}

/// Mint transaction — creates tokens from nothing, authorised by validators.
pub struct MintTransaction {
    /// Standard RingCT outputs (raw bytes, Phase 2 will type these).
    pub outputs: Vec<u8>,
    /// Ties this mint to a specific Base burn event (replay protection).
    /// keccak256(base_tx_hash || log_index)
    pub base_burn_id: [u8; 32],
    /// Amount in atomic units — needed for threshold validation.
    pub amount: u64,
    /// Aggregate blinding factor for the synthetic source commitment.
    /// C_src = Commit(amount, commitment_mask).
    pub commitment_mask: [u8; 32],
    /// Validator signatures over:
    /// keccak256(base_burn_id || amount || hash(outputs) || commitment_mask)
    /// At least 2/3+1 of the active set are required.
    pub validator_sigs: Vec<ValidatorSignature>,
}

/// A validator's signature over a mint authorization.
pub struct ValidatorSignature {
    /// keccak256(chain_pubkey) — unique validator identifier
    pub validator_id: [u8; 32],
    /// secp256k1 signature bytes
    pub signature: Vec<u8>,
}
