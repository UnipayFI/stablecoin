# Stablecoin Project README
## Overview

This project implements a stablecoin system with the following key components:

- USDU: The main stablecoin token
- SUSDU: Staking token for USDU
- Vault: Manages collateral, minting, and redemption

## Core Flow

0. Role Assignment

```typescript
// Assign roles to vault_config
usduMinter = await AssignRole(guardianProgram, accessRegistry, admin, vaultConfig, "usdu_minter");
susduMinter = await AssignRole(guardianProgram, accessRegistry, admin, vaultConfig, "susdu_minter");
usduRedeemer = await AssignRole(guardianProgram, accessRegistry, admin, vaultConfig, "usdu_redeemer");
susduRedeemer = await AssignRole(guardianProgram, accessRegistry, admin, vaultConfig, "susdu_redeemer");
susduRedistributor = await AssignRole(guardianProgram, accessRegistry, admin, vaultConfig, "susdu_redistributor");

// Assign roles to admin
collateralDepositor = await AssignRole(guardianProgram, accessRegistry, admin, admin.publicKey, "collateral_depositor");
collateralWithdrawer = await AssignRole(guardianProgram, accessRegistry, admin, admin.publicKey, "collateral_withdrawer");
grandMaster = await AssignRole(guardianProgram, accessRegistry, admin, admin.publicKey, "grand_master");
```

1. Initial Setup

```typescript
// Initialize core components
await InitGuardianAccessRegistry(guardianProgram, accessRegistry, admin);
await InitAndCreateUSDU(usduProgram, usduMintToken, accessRegistry, usduConfig, admin);
await InitAndCreateSusdu(susduProgram, susduMintToken, accessRegistry, susduConfig, admin);
await InitVaultConfig(vaultProgram, vaultConfig, accessRegistry, usduMintToken, susduMintToken, admin, 0);
await InitVaultState(
    vaultProgram,
    vaultConfig,
    vaultState,
    vaultUsduTokenAccount,
    vaultSusduTokenAccount,
    vaultStakePoolUsduTokenAccount,
    vaultSlioUsduTokenAccount,
    usduMintToken,
    susduMintToken,
    admin
);
```

2. Main Operations

2.1 Deposit Collateral and Mint USDU

```typescript
// Mint collateral tokens
const mintIx = createMintToInstruction(
    mintToken.publicKey,
    benefactorCollateralTokenAccount.address,
    admin.publicKey,
    100_000_000_000,
    [admin],
    TOKEN_2022_PROGRAM_ID
);

// Approve collateral tokens
const approveIx = createApproveInstruction(
    benefactorCollateralTokenAccount.address,
    vaultConfig,
    benefactor.publicKey,
    100_000_000_000,
    [benefactor],
    TOKEN_2022_PROGRAM_ID
);

// Deposit and mint USDU
await DepositCollateralAndMintUsdu(
    vaultProgram,
    usduProgram,
    admin,
    vaultConfig,
    usduConfig,
    accessRegistry,
    usduMinter,
    collateralDepositor,
    mintToken.publicKey,
    usduMintToken,
    benefactor,
    beneficiary,
    fund,
    1200_000_000,
    1000_000_000,
    benefactorCollateralTokenAccount.address,
    beneficiaryUsduTokenAccount.address,
    fundCollateralTokenAccount.address,
    vaultCollateralTokenAccount,
);
```

2.2 Redeem USDU and Withdraw Collateral

```typescript
// Approve collateral to the vault
const approveCollateralIx = createApproveInstruction(
    fundCollateralTokenAccount.address,
    vaultConfig,
    fund.publicKey,
    100_000_000_000,
    [fund],
    TOKEN_2022_PROGRAM_ID
);

// Approve USDU to the vault
const approveUsduIx = createApproveInstruction(
    beneficiaryUsduTokenAccount.address,
    vaultConfig,
    beneficiary.publicKey,
    60_000_000_000,
    [beneficiary],
    TOKEN_2022_PROGRAM_ID
);

// Redeem and withdraw
await RedeemUsduAndWithdrawCollateral(
    vaultProgram,
    usduProgram,
    admin,
    vaultConfig,
    vaultState,
    usduConfig,
    accessRegistry,
    usduRedeemer,
    collateralWithdrawer,
    mintToken.publicKey,
    usduMintToken,
    benefactor,
    beneficiary,
    fund,
    20_000_000,
    10_000_000,
    beneficiaryUsduTokenAccount.address,
    fundCollateralTokenAccount.address,
    vaultUsduTokenAccount,
    benefactorCollateralTokenAccount.address,
);
```

2.3 Stake USDU and Mint SUSDU

```typescript
const blacklistState = PublicKey.findProgramAddressSync(
    [Buffer.from(vaultBlacklistSeed), caller.publicKey.toBuffer()],
    vaultProgram.programId
)[0];

await StakeUsduMintSusdu(
    vaultProgram,
    susduProgram,
    caller,
    susduReceiver,
    susduReceiverSusduTokenAccount.address,
    beneficiaryUsduTokenAccount.address,
    accessRegistry,
    vaultStakePoolUsduTokenAccount,
    susduMinter,
    usduMintToken,
    susduMintToken,
    vaultState,
    vaultConfig,
    susduConfig,
    100_000_000,
    blacklistState,
);
```

2.4 Unstake SUSDU

```typescript
const [cooldown] = PublicKey.findProgramAddressSync(
    [
        Buffer.from(vaultCooldownSeed),
        Buffer.from(usduMintToken.toBuffer()),
        Buffer.from(receiver.publicKey.toBuffer()),
    ],
    vaultProgram.programId
);

await UnstakeSusdu(
    vaultProgram,
    susduProgram,
    caller,
    callerSusduTokenAccount,
    receiver.publicKey,
    receiverUsduTokenAccount,
    susduConfig,
    vaultConfig,
    vaultState,
    vaultSusduTokenAccount,
    cooldown,
    accessRegistry,
    susduRedeemer,
    susduMintToken,
    usduMintToken,
    vaultStakePoolUsduTokenAccount,
    vaultSlioUsduTokenAccount,
    30_000_000,
    blacklistState,
);
```

2.5 Withdraw USDU (After Cooldown)

```typescript
await WithdrawUsdu(
    vaultProgram,
    receiver,
    vaultConfig,
    vaultState,
    receiverUsduTokenAccount,
    vaultSlioUsduTokenAccount,
    cooldown,
    usduMintToken,
    blacklistState,
);
```

2.6 Distribute USDU Reward

```typescript
const distributeRewarder = await AssignRole(
    guardianProgram, 
    accessRegistry, 
    admin, 
    beneficiary.publicKey, 
    "distribute_rewarder"
);

await DistributeUsduReward(
    vaultProgram,
    caller,
    vaultConfig,
    vaultState,
    callerUsduTokenAccount,
    vaultStakePoolUsduTokenAccount,
    accessRegistry,
    distributeRewarder,
    usduMintToken,
    susduMintToken,
    susduConfig,
    100_100_000,
);
```

2.7 Blacklist Management

```typescript
// Adjust blacklist
await AdjustBlacklist(
    vaultProgram,
    admin,
    vaultConfig,
    accessRegistry,
    grandMaster,
    true,  // add to blacklist
    false, // is permanent
    user,
);

// Redistribute locked SUSDU
await RedistributeLockedSusdu(
    vaultProgram,
    susduProgram,
    vaultConfig,
    admin,
    accessRegistry,
    grandMaster,
    susduRedistributor,
    susduConfig,
    susduMintToken,
    lockedSusduTokenAccount,
    newSusduReceiverTokenAccount.address,
    blacklistState,
    newSusduReceiver.publicKey,
);
```