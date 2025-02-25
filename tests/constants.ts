/// vault seeds
export const vaultStateSeed = "vault-state";
export const vaultConfigSeed = "vault-config";
export const vaultCooldownSeed = "vault-cooldown";
export const vaultSusduTokenAccountSeed = "vault-susdu-token-approval";
export const vaultUsduTokenAccountSeed = "vault-usdu-approval";
export const vaultStakePoolUsduTokenAccountSeed = "vault-stake-pool-usdu";
export const vaultSlioUsduTokenAccountSeed = "vault-slio-usdu";
export const vaultBlacklistSeed = "vault-blacklist";
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
    | { susduRedistributor: {} }
    | { collateralDepositor: {} }
    | { collateralWithdrawer: {} }
    | { usduStaker: {} }
    | { usduUnstaker: {} }
    | { grandMaster: {} }
    | { distributeRewarder: {} };

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
        case "susdu_redistributor":
            return { susduRedistributor: {} };
        case "collateral_depositor":
            return { collateralDepositor: {} };
        case "collateral_withdrawer":
            return { collateralWithdrawer: {} };
        case "usdu_staker":
            return { usduStaker: {} };
        case "usdu_unstaker":
            return { usduUnstaker: {} };
        case "grand_master":
            return { grandMaster: {} };
        case "distribute_rewarder":
            return { distributeRewarder: {} };
        default:
            throw new Error(`Invalid role: ${role}`);
    }
}

export function roleToBytes(role: string): Uint8Array {
    const bytes = new Uint8Array(32).fill(0);
    const roleBytes = new Uint8Array(Buffer.from(role, 'utf-8'));
    bytes.set(roleBytes.subarray(0, Math.min(roleBytes.length, 32)));
    return bytes;
}