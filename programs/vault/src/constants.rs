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
pub const VAULT_SLIO_USDU_TOKEN_ACCOUNT_SEED: &[u8] = b"vault-slio-usdu";