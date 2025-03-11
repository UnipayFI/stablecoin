/// vault seeds
export const vaultStateSeed = "vault-state";
export const vaultConfigSeed = "vault-config";
export const vaultCooldownSeed = "vault-cooldown";
export const vaultSusduTokenAccountSeed = "vault-susdu-token-approval";
export const vaultUsduTokenAccountSeed = "vault-usdu-approval";
export const vaultStakePoolUsduTokenAccountSeed = "vault-stake-pool-usdu";
export const vaultSiloUsduTokenAccountSeed = "vault-silo-usdu";
export const blacklistHookConfigSeed = "blacklist-hook-config";
export const blacklistHookExtraAccountMetaListSeed = "extra-account-metas";
export const blacklistEntrySeed = "blacklist-entry";

/// usdu seeds
export const usduConfigSeed = "usdu-config";
export const usduSeed = "usdu-spl-token";

/// susdu seeds
export const susduConfigSeed = "susdu-config";
export const susduSeed = "susdu-spl-token";

/// access registry seeds
export const accessRegistrySeed = "access-registry";
export const accessRoleSeed = "access-role";

export type RoleType =
  | { guardianAdmin: {} }
  | { usduMinter: {} }
  | { usduRedeemer: {} }
  | { susduMinter: {} }
  | { susduRedeemer: {} }
  | { susduDistributor: {} }
  | { collateralDepositor: {} }
  | { collateralWithdrawer: {} }
  | { usduStaker: {} }
  | { usduUnstaker: {} }
  | { vaultAdmin: {} }
  | { rewardDistributor: {} };

export function getRole(role: string): RoleType {
  switch (role) {
    case "guardian_admin":
      return { guardianAdmin: {} };
    case "usdu_minter":
      return { usduMinter: {} };
    case "usdu_redeemer":
      return { usduRedeemer: {} };
    case "susdu_minter":
      return { susduMinter: {} };
    case "susdu_redeemer":
      return { susduRedeemer: {} };
    case "susdu_distributor":
      return { susduDistributor: {} };
    case "collateral_depositor":
      return { collateralDepositor: {} };
    case "collateral_withdrawer":
      return { collateralWithdrawer: {} };
    case "usdu_staker":
      return { usduStaker: {} };
    case "usdu_unstaker":
      return { usduUnstaker: {} };
    case "vault_admin":
      return { vaultAdmin: {} };
    case "reward_distributor":
      return { rewardDistributor: {} };
    case "distribute_rewarder":
      return { rewardDistributor: {} };
    default:
      throw new Error(`Invalid role: ${role}`);
  }
}

export function roleToBytes(role: string): Uint8Array {
  const bytes = new Uint8Array(32).fill(0);
  const roleBytes = new Uint8Array(Buffer.from(role, "utf-8"));
  bytes.set(roleBytes.subarray(0, Math.min(roleBytes.length, 32)));
  return bytes;
}
