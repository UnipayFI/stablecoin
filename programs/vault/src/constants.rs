use anchor_lang::prelude::*;

#[constant]
pub const VAULT_CONFIG_SEED: &[u8] = b"vault-config";

#[constant]
pub const VAULT_STATE_SEED: &[u8] = b"vault-state";

#[constant]
pub const VAULT_COOLDOWN_SEED: &[u8] = b"vault-cooldown";

#[constant]
pub const VAULT_SUSDU_TOKEN_ACCOUNT_SEED: &[u8] = b"vault-susdu-token-approval";

#[constant]
pub const VAULT_USDU_TOKEN_ACCOUNT_SEED: &[u8] = b"vault-usdu-approval";

#[constant]
pub const VAULT_STAKE_POOL_USDU_TOKEN_ACCOUNT_SEED: &[u8] = b"vault-stake-pool-usdu";

#[constant]
pub const VAULT_SILO_USDU_TOKEN_ACCOUNT_SEED: &[u8] = b"vault-silo-usdu";

#[cfg(not(feature = "testnet"))]
#[constant]
pub const MIN_COOLDOWN_DURATION: u64 = 60 * 60;

#[cfg(feature = "testnet")]
#[constant]
pub const MIN_COOLDOWN_DURATION: u64 = 0;

#[constant]
pub const DEFAULT_COOLDOWN_DURATION: u64 = 7 * 24 * 60 * 60;

#[constant]
pub const MAX_COOLDOWN_DURATION: u64 = 30 * 24 * 60 * 60;
