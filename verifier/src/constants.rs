//! Constants.

// Constants for the chain type flag
// Specifically utilizing the two highest bits for chain type identification
pub const FLAG_CHAIN_TYPE_MAINNET: u8 = 0b0000_0000; // for mainnet
pub const FLAG_CHAIN_TYPE_TESTNET: u8 = 0b1000_0000; // for testnet
pub const FLAG_CHAIN_TYPE_SIGNET: u8 = 0b0100_0000; // for signet
pub const FLAG_CHAIN_TYPE_TESTNET4: u8 = 0b1100_0000; // for testnet4

/// The minimum difficulty bits (difficulty 1) used by testnet's 20-minute exception rule.
pub const TESTNET_MIN_DIFFICULTY_BITS: u32 = 0x1d00ffff;
/// On testnet, a block whose time is more than this many seconds after the previous block
/// must use the minimum difficulty (twice the 10-minute target spacing).
pub const TESTNET_MIN_DIFFICULTY_TIME_GAP: u32 = 20 * 60;
/// BIP-94: the first block of a difficulty period can't be earlier than its previous block
/// by more than this many seconds.
pub const BIP94_MAX_TIMEWARP: u32 = 600;
