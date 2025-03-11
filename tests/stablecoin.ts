import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import {
  Keypair,
  PublicKey,
  Connection,
  LAMPORTS_PER_SOL,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAccount,
  TOKEN_2022_PROGRAM_ID,
  getOrCreateAssociatedTokenAccount,
  createApproveInstruction,
  createMintToInstruction,
  Account,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import { Guardian } from "../target/types/guardian";
import { Usdu } from "../target/types/usdu";
import { Susdu } from "../target/types/susdu";
import { Vault } from "../target/types/vault";
import { BlacklistHook } from "../target/types/blacklist_hook";
import {
  accessRegistrySeed,
  susduSeed,
  usduSeed,
  vaultConfigSeed,
  vaultStakePoolUsduTokenAccountSeed,
  vaultSiloUsduTokenAccountSeed,
  vaultStateSeed,
  vaultCooldownSeed,
  vaultSusduTokenAccountSeed,
  vaultUsduTokenAccountSeed,
  usduConfigSeed,
  susduConfigSeed,
  blacklistHookExtraAccountMetaListSeed,
  blacklistHookConfigSeed,
} from "./constants";
import {
  InitGuardianAccessRegistry,
  AssignRole,
  InitAndCreateUSDU,
  InitVaultConfig,
  InitAndCreateSusdu,
  InitVaultState,
  AirdropSol,
  CreateMintToken,
  DepositCollateralAndMintUsdu,
  RedeemUsduAndWithdrawCollateral,
  StakeUsduMintSusdu,
  UnstakeSusdu,
  WithdrawUsdu,
  DistributeUsduReward,
  RedistributeLockedSusdu,
  InitializeBlacklistHook,
  AddToBlacklist,
  getBlacklistEntryPda,
} from "./utils";
import { assert } from "chai";
import {
  adminBytes,
  fundBytes,
  benefactorBytes,
  beneficiaryBytes,
  susduRecevierBytes,
  mintTokenBytes,
} from "./accounts";

const NEED_AIRDROP_SOL = true;

describe("Stablecoin Preparation", () => {
  let guardianProgram: Program<Guardian>;
  let usduProgram: Program<Usdu>;
  let susduProgram: Program<Susdu>;
  let vaultProgram: Program<Vault>;
  let blacklistHookProgram: Program<BlacklistHook>;

  let admin: Keypair;

  let accessRegistry: PublicKey;
  let usduConfig: PublicKey;
  let susduConfig: PublicKey;
  let usduMintToken: PublicKey;
  let susduMintToken: PublicKey;
  let vaultConfig: PublicKey;
  let vaultState: PublicKey;
  let vaultUsduTokenAccount: PublicKey;
  let vaultSusduTokenAccount: PublicKey;
  let vaultStakePoolUsduTokenAccount: PublicKey;
  let vaultSiloUsduTokenAccount: PublicKey;
  let connection: Connection;
  let usduMinter: PublicKey;
  let collateralDepositor: PublicKey;
  let usduRedeemer: PublicKey;
  let collateralWithdrawer: PublicKey;
  let susduMinter: PublicKey;
  let susduRedeemer: PublicKey;
  let vaultAdmin: PublicKey;
  let susduRedistributor: PublicKey;
  let extraAccountMetaList: PublicKey;
  let blacklistHookConfig: PublicKey;

  before(async () => {
    const provider = anchor.AnchorProvider.env();
    connection = provider.connection;
    guardianProgram = anchor.workspace.Guardian as Program<Guardian>;
    usduProgram = anchor.workspace.Usdu as Program<Usdu>;
    susduProgram = anchor.workspace.Susdu as Program<Susdu>;
    vaultProgram = anchor.workspace.Vault as Program<Vault>;
    blacklistHookProgram = anchor.workspace.BlacklistHook as Program<BlacklistHook>;;

    admin = Keypair.fromSecretKey(Uint8Array.from(adminBytes));

    [accessRegistry] = PublicKey.findProgramAddressSync(
      [Buffer.from(accessRegistrySeed)],
      guardianProgram.programId
    );

    [usduMintToken] = PublicKey.findProgramAddressSync(
      [Buffer.from(usduSeed)],
      usduProgram.programId
    );
    [susduMintToken] = PublicKey.findProgramAddressSync(
      [Buffer.from(susduSeed)],
      susduProgram.programId
    );

    [usduConfig] = PublicKey.findProgramAddressSync(
      [Buffer.from(usduConfigSeed)],
      usduProgram.programId
    );
    [susduConfig] = PublicKey.findProgramAddressSync(
      [Buffer.from(susduConfigSeed)],
      susduProgram.programId
    );
    [vaultConfig] = PublicKey.findProgramAddressSync(
      [Buffer.from(vaultConfigSeed)],
      vaultProgram.programId
    );
    [vaultState] = PublicKey.findProgramAddressSync(
      [Buffer.from(vaultStateSeed)],
      vaultProgram.programId
    );
    [vaultUsduTokenAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(vaultUsduTokenAccountSeed)],
      vaultProgram.programId
    );
    [vaultSusduTokenAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(vaultSusduTokenAccountSeed)],
      vaultProgram.programId
    );
    [vaultStakePoolUsduTokenAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(vaultStakePoolUsduTokenAccountSeed)],
      vaultProgram.programId
    );
    [vaultSiloUsduTokenAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(vaultSiloUsduTokenAccountSeed)],
      vaultProgram.programId
    );
    [extraAccountMetaList] = PublicKey.findProgramAddressSync(
      [Buffer.from(blacklistHookExtraAccountMetaListSeed), susduMintToken.toBuffer()],
      blacklistHookProgram.programId
    );
    [blacklistHookConfig] = PublicKey.findProgramAddressSync(
      [Buffer.from(blacklistHookConfigSeed)],
      blacklistHookProgram.programId
    );

    if (NEED_AIRDROP_SOL) {
      await Promise.all([
        AirdropSol(connection, admin.publicKey, LAMPORTS_PER_SOL * 100),
      ]);
    }
    
    await InitGuardianAccessRegistry(guardianProgram, accessRegistry, admin);
    await InitAndCreateUSDU(
      usduProgram,
      usduMintToken,
      accessRegistry,
      usduConfig,
      admin
    );
    await InitAndCreateSusdu(
      susduProgram,
      blacklistHookProgram,
      susduMintToken,
      accessRegistry,
      susduConfig,
      admin
    );
   
    await InitVaultConfig(
      vaultProgram,
      vaultConfig,
      accessRegistry,
      usduMintToken,
      susduMintToken,
      admin,
      0
    );
    await InitVaultState(
      vaultProgram,
      vaultConfig,
      vaultState,
      vaultUsduTokenAccount,
      vaultSusduTokenAccount,
      vaultStakePoolUsduTokenAccount,
      vaultSiloUsduTokenAccount,
      usduMintToken,
      susduMintToken,
      admin
    );
    await InitializeBlacklistHook(
        blacklistHookProgram,
        admin,
        extraAccountMetaList,
        blacklistHookConfig,
        susduMintToken
    );

    // assign usdu_minter role to vault_config
    usduMinter = await AssignRole(
      guardianProgram,
      accessRegistry,
      admin,
      vaultConfig,
      "usdu_minter"
    );
    // assign susdu_minter role to vault_config
    susduMinter = await AssignRole(
      guardianProgram,
      accessRegistry,
      admin,
      vaultConfig,
      "susdu_minter"
    );
    // assign usdu_redeemer role to vault_config
    usduRedeemer = await AssignRole(
      guardianProgram,
      accessRegistry,
      admin,
      vaultConfig,
      "usdu_redeemer"
    );
    // assign susdu_redeemer role to vault_config
    susduRedeemer = await AssignRole(
      guardianProgram,
      accessRegistry,
      admin,
      vaultConfig,
      "susdu_redeemer"
    );
    // assign vault_susdu_redistributor role to vault_config
    susduRedistributor = await AssignRole(
      guardianProgram,
      accessRegistry,
      admin,
      vaultConfig,
      "susdu_distributor"
    );
    // assign vault_usdu_minter role to admin
    collateralDepositor = await AssignRole(
      guardianProgram,
      accessRegistry,
      admin,
      admin.publicKey,
      "collateral_depositor"
    );
    // assign vault_usdu_redeemer role to admin
    collateralWithdrawer = await AssignRole(
      guardianProgram,
      accessRegistry,
      admin,
      admin.publicKey,
      "collateral_withdrawer"
    );
    // assign vault_manager role to admin
    vaultAdmin = await AssignRole(
      guardianProgram,
      accessRegistry,
      admin,
      admin.publicKey,
      "vault_admin"
    );
  });

  it("get usdu token and susdu token", async () => {
    let usduTokenAccount;
    let susduTokenAccount;
    try {
      usduTokenAccount = await getAccount(
        connection,
        vaultUsduTokenAccount,
        "confirmed",
        TOKEN_2022_PROGRAM_ID
      );
      susduTokenAccount = await getAccount(
        connection,
        vaultSusduTokenAccount,
        "confirmed",
        TOKEN_2022_PROGRAM_ID
      );
    } catch (error) {
      console.error("Error:", error);
    }
    assert(usduTokenAccount !== null, "Usdu Token Account not found");
    assert(susduTokenAccount !== null, "Susdu Token Account not found");
  });

  describe("stablecoin Testcase", () => {
    let mintToken = Keypair.fromSecretKey(Uint8Array.from(mintTokenBytes));
    let benefactor = Keypair.fromSecretKey(Uint8Array.from(benefactorBytes));
    let fund = Keypair.fromSecretKey(Uint8Array.from(fundBytes));
    let beneficiary = Keypair.fromSecretKey(Uint8Array.from(beneficiaryBytes));

    let beneficiaryUsduTokenAccount: Account;
    let beneficiaryCollateralTokenAccount: Account;
    let fundCollateralTokenAccount: Account;
    let benefactorCollateralTokenAccount: Account;
    let benefactorUsduTokenAccount: Account;

    let susduReceiver = Keypair.fromSecretKey(
      Uint8Array.from(susduRecevierBytes)
    );
    let susduReceiverSusduTokenAccount: Account;

    before(async () => {
      if (NEED_AIRDROP_SOL) {
        await Promise.all([
          AirdropSol(connection, beneficiary.publicKey, LAMPORTS_PER_SOL * 100),
          AirdropSol(connection, fund.publicKey, LAMPORTS_PER_SOL * 100),
          AirdropSol(connection, benefactor.publicKey, LAMPORTS_PER_SOL * 100),
          AirdropSol(
            connection,
            susduReceiver.publicKey,
            LAMPORTS_PER_SOL * 100
          ),
        ]);
      }
      await CreateMintToken(
        connection,
        admin,
        mintToken,
        admin.publicKey,
        admin.publicKey,
        6
      );
      console.log(`mintToken: ${mintToken.publicKey}`);
      beneficiaryUsduTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        beneficiary,
        usduMintToken,
        beneficiary.publicKey,
        true,
        "confirmed",
        {
          skipPreflight: true,
          commitment: "confirmed",
        },
        TOKEN_2022_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
      );
      beneficiaryCollateralTokenAccount =
        await getOrCreateAssociatedTokenAccount(
          connection,
          beneficiary,
          mintToken.publicKey,
          beneficiary.publicKey,
          true,
          "confirmed",
          {
            skipPreflight: true,
            commitment: "confirmed",
          },
          TOKEN_2022_PROGRAM_ID,
          ASSOCIATED_TOKEN_PROGRAM_ID
        );
      fundCollateralTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        fund,
        mintToken.publicKey,
        fund.publicKey,
        true,
        "confirmed",
        {
          skipPreflight: true,
          commitment: "confirmed",
        },
        TOKEN_2022_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
      );

      benefactorCollateralTokenAccount =
        await getOrCreateAssociatedTokenAccount(
          connection,
          benefactor,
          mintToken.publicKey,
          benefactor.publicKey,
          true,
          "confirmed",
          {
            skipPreflight: true,
            commitment: "confirmed",
          },
          TOKEN_2022_PROGRAM_ID,
          ASSOCIATED_TOKEN_PROGRAM_ID
        );
      benefactorUsduTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        benefactor,
        usduMintToken,
        benefactor.publicKey,
        true,
        "confirmed",
        {
          skipPreflight: true,
          commitment: "confirmed",
        },
        TOKEN_2022_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
      );
      susduReceiverSusduTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        susduReceiver,
        susduMintToken,
        susduReceiver.publicKey,
        true,
        "confirmed",
        {
          skipPreflight: true,
          commitment: "confirmed",
        },
        TOKEN_2022_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
      );
      console.log(
        `benefactorUsduTokenAccount: ${benefactorUsduTokenAccount.address}`
      );
      console.log(
        `benefactorCollateralTokenAccount: ${benefactorCollateralTokenAccount.address}`
      );
      console.log(
        `beneficiaryUsduTokenAccount: ${beneficiaryUsduTokenAccount.address}`
      );
      console.log(
        `beneficiaryCollateralTokenAccount: ${beneficiaryCollateralTokenAccount.address}`
      );
      console.log(
        `fundCollateralTokenAccount: ${fundCollateralTokenAccount.address}`
      );
      console.log(
        `susduReceiverSusduTokenAccount: ${susduReceiverSusduTokenAccount.address}`
      );
    });

    it("mint token, approve, deposit", async () => {
      const mintIx = createMintToInstruction(
        mintToken.publicKey,
        benefactorCollateralTokenAccount.address,
        admin.publicKey,
        10000_000_000_000,
        [admin],
        TOKEN_2022_PROGRAM_ID
      );
      const mintTx = new Transaction().add(mintIx);
      const mintTxSignature = await sendAndConfirmTransaction(
        connection,
        mintTx,
        [admin],
        {
          skipPreflight: true,
          commitment: "confirmed",
        }
      );
      console.log(`mintTxSignature: ${mintTxSignature}`);
      const approveIx = createApproveInstruction(
        benefactorCollateralTokenAccount.address,
        vaultConfig,
        benefactor.publicKey,
        10000_000_000_000,
        [benefactor],
        TOKEN_2022_PROGRAM_ID
      );
      const approveTx = new Transaction().add(approveIx);
      const approveTxSignature = await sendAndConfirmTransaction(
        connection,
        approveTx,
        [benefactor],
        {
          skipPreflight: true,
          commitment: "confirmed",
        }
      );
      console.log(`approveTxSignature: ${approveTxSignature}`);

      // deposit and mint usdu
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
        2200_000_000,
        2000_000_000,
        benefactorCollateralTokenAccount.address,
        beneficiaryUsduTokenAccount.address,
        fundCollateralTokenAccount.address
      );
      const usduConfigAccount = await usduProgram.account.usduConfig.fetch(
        usduConfig
      );
      console.log(
        `After deposit collateral, usduConfigAccount TotalSupply: ${usduConfigAccount.totalSupply}`
      );
      const susduConfigAccount = await susduProgram.account.susduConfig.fetch(
        susduConfig
      );
      console.log(
        `After deposit collateral, susduConfigAccount TotalSupply: ${susduConfigAccount.totalSupply}`
      );
    });

    it("redeem usdu and withdraw collateral", async () => {
      /// approve collateral to the vault
      const approveCollateralIx = createApproveInstruction(
        fundCollateralTokenAccount.address,
        vaultConfig,
        fund.publicKey,
        100_000_000_000,
        [fund],
        TOKEN_2022_PROGRAM_ID
      );
      const approveCollateralTx = new Transaction().add(approveCollateralIx);
      const approveCollateralTxSignature = await sendAndConfirmTransaction(
        connection,
        approveCollateralTx,
        [fund],
        {
          skipPreflight: true,
          commitment: "confirmed",
        }
      );
      console.log(
        `approveCollateralTxSignature: ${approveCollateralTxSignature}`
      );

      /// approve usdu to the vault
      const approveUsduIx = createApproveInstruction(
        beneficiaryUsduTokenAccount.address,
        vaultConfig,
        beneficiary.publicKey,
        60_000_000_000,
        [beneficiary],
        TOKEN_2022_PROGRAM_ID
      );
      const approveUsduTx = new Transaction().add(approveUsduIx);
      const approveUsduTxSignature = await sendAndConfirmTransaction(
        connection,
        approveUsduTx,
        [beneficiary],
        {
          skipPreflight: true,
          commitment: "confirmed",
        }
      );
      console.log(`approveUsduTxSignature: ${approveUsduTxSignature}`);

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
        benefactorCollateralTokenAccount.address
      );
      const usduConfigAccount = await usduProgram.account.usduConfig.fetch(
        usduConfig
      );
      console.log(
        `After redeem usdu, usduConfigAccount TotalSupply: ${usduConfigAccount.totalSupply}`
      );
      const susduConfigAccount = await susduProgram.account.susduConfig.fetch(
        susduConfig
      );
      console.log(
        `After redeem usdu, susduConfigAccount TotalSupply: ${susduConfigAccount.totalSupply}`
      );
    });

    it("stake usdu and mint susdu", async () => {
      const caller = beneficiary;
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
        1000_000_000
      );
      const usduConfigAccount = await usduProgram.account.usduConfig.fetch(
        usduConfig
      );
      console.log(
        `After stake usdu, usduConfigAccount TotalSupply: ${usduConfigAccount.totalSupply}`
      );
      const susduConfigAccount = await susduProgram.account.susduConfig.fetch(
        susduConfig
      );
      console.log(
        `After stake usdu, susduConfigAccount TotalSupply: ${susduConfigAccount.totalSupply}`
      );
    });
    it("unstake susdu and ready to wait cooldown", async () => {
      const caller = susduReceiver;
      const callerSusduTokenAccount = susduReceiverSusduTokenAccount.address;
      const receiver = beneficiary;
      const receiverUsduTokenAccount = beneficiaryUsduTokenAccount.address;
      const [cooldown] = PublicKey.findProgramAddressSync(
        [
          Buffer.from(vaultCooldownSeed),
          Buffer.from(usduMintToken.toBuffer()),
          Buffer.from(receiver.publicKey.toBuffer()),
          Buffer.from(caller.publicKey.toBuffer()),
        ],
        vaultProgram.programId
      );
      let sourceTokenBlacklistAccount = getBlacklistEntryPda(blacklistHookProgram, caller.publicKey);
      let destinationTokenBlacklistAccount = getBlacklistEntryPda(blacklistHookProgram, vaultConfig);
     
      await UnstakeSusdu(
        vaultProgram,
        susduProgram,
        blacklistHookProgram,
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
        vaultSiloUsduTokenAccount,
        300_000_000,
        extraAccountMetaList,
        sourceTokenBlacklistAccount,
        destinationTokenBlacklistAccount,
      );
      const usduConfigAccount = await usduProgram.account.usduConfig.fetch(
        usduConfig
      );
      console.log(
        `After unstake susdu, usduConfigAccount TotalSupply: ${usduConfigAccount.totalSupply}`
      );
      const susduConfigAccount = await susduProgram.account.susduConfig.fetch(
        susduConfig
      );
      console.log(
        `After unstake susdu, susduConfigAccount TotalSupply: ${susduConfigAccount.totalSupply}`
      );
      const cooldownAccount = await vaultProgram.account.cooldown.fetch(
        cooldown
      );
      console.log(
        `After unstake susdu, cooldownAccount.underlyingTokenAmount: ${cooldownAccount.underlyingTokenAmount}`
      );
    });
    it("withdraw usdu", async () => {
      const caller = susduReceiver;
      const receiver = beneficiary;
      const receiverUsduTokenAccount = beneficiaryUsduTokenAccount.address;
      const [cooldown] = PublicKey.findProgramAddressSync(
        [
          Buffer.from(vaultCooldownSeed),
          Buffer.from(usduMintToken.toBuffer()),
          Buffer.from(receiver.publicKey.toBuffer()),
          Buffer.from(caller.publicKey.toBuffer()),
        ],
        vaultProgram.programId
      );
      await WithdrawUsdu(
        vaultProgram,
        caller,
        receiver.publicKey,
        vaultConfig,
        vaultState,
        receiverUsduTokenAccount,
        vaultSiloUsduTokenAccount,
        cooldown,
        usduMintToken
      );
    });
    it("unstake susdu and ready to wait cooldown", async () => {
      const caller = susduReceiver;
      const callerSusduTokenAccount = susduReceiverSusduTokenAccount.address;
      const receiver = beneficiary;
      const receiverUsduTokenAccount = beneficiaryUsduTokenAccount.address;
      const [cooldown] = PublicKey.findProgramAddressSync(
        [
          Buffer.from(vaultCooldownSeed),
          Buffer.from(usduMintToken.toBuffer()),
          Buffer.from(receiver.publicKey.toBuffer()),
          Buffer.from(caller.publicKey.toBuffer()),
        ],
        vaultProgram.programId
      );
      let sourceTokenBlacklistAccount = getBlacklistEntryPda(blacklistHookProgram, caller.publicKey);
      let destinationTokenBlacklistAccount = getBlacklistEntryPda(blacklistHookProgram, vaultConfig);
      await UnstakeSusdu(
        vaultProgram,
        susduProgram,
        blacklistHookProgram,
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
        vaultSiloUsduTokenAccount,
        3_000_000,
        extraAccountMetaList,
        sourceTokenBlacklistAccount,
        destinationTokenBlacklistAccount,
      );
      const usduConfigAccount = await usduProgram.account.usduConfig.fetch(
        usduConfig
      );
      console.log(
        `After unstake susdu, usduConfigAccount TotalSupply: ${usduConfigAccount.totalSupply}`
      );
      const susduConfigAccount = await susduProgram.account.susduConfig.fetch(
        susduConfig
      );
      console.log(
        `After unstake susdu, susduConfigAccount TotalSupply: ${susduConfigAccount.totalSupply}`
      );
      const cooldownAccount = await vaultProgram.account.cooldown.fetch(
        cooldown
      );
      console.log(
        `After unstake susdu, cooldownAccount.underlyingTokenAmount: ${cooldownAccount.underlyingTokenAmount}`
      );
    });
    it("stake usdu and mint susdu again", async () => {
      const caller = beneficiary;
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
        27_520_000
      );
      const usduConfigAccount = await usduProgram.account.usduConfig.fetch(
        usduConfig
      );
      console.log(
        `After stake usdu, usduConfigAccount TotalSupply: ${usduConfigAccount.totalSupply}`
      );
      const susduConfigAccount = await susduProgram.account.susduConfig.fetch(
        susduConfig
      );
      console.log(
        `After stake usdu, susduConfigAccount TotalSupply: ${susduConfigAccount.totalSupply}`
      );
    });
    it("stake usdu and mint susdu again and again", async () => {
      const caller = beneficiary;
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
        75_010_000
      );
      const usduConfigAccount = await usduProgram.account.usduConfig.fetch(
        usduConfig
      );
      console.log(
        `After stake usdu, usduConfigAccount TotalSupply: ${usduConfigAccount.totalSupply}`
      );
      const susduConfigAccount = await susduProgram.account.susduConfig.fetch(
        susduConfig
      );
      console.log(
        `After stake usdu, susduConfigAccount TotalSupply: ${susduConfigAccount.totalSupply}`
      );
    });
    it("distribute usdu reward", async () => {
      const caller = beneficiary;
      const callerUsduTokenAccount = beneficiaryUsduTokenAccount.address;
      const distributeRewarder = await AssignRole(
        guardianProgram,
        accessRegistry,
        admin,
        beneficiary.publicKey,
        "reward_distributor"
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
        100_100_000
      );
    });
    it("stake usdu and mint susdu again and again", async () => {
      const caller = beneficiary;
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
        231_010_000
      );
      const usduConfigAccount = await usduProgram.account.usduConfig.fetch(
        usduConfig
      );
      console.log(
        `After stake usdu, usduConfigAccount TotalSupply: ${usduConfigAccount.totalSupply}`
      );
      const susduConfigAccount = await susduProgram.account.susduConfig.fetch(
        susduConfig
      );
      console.log(
        `After stake usdu, susduConfigAccount TotalSupply: ${susduConfigAccount.totalSupply}`
      );
    });
    it("stake usdu and mint susdu again and again", async () => {
      const caller = beneficiary;
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
        112_000_000
      );
      const usduConfigAccount = await usduProgram.account.usduConfig.fetch(
        usduConfig
      );
      console.log(
        `After stake usdu, usduConfigAccount TotalSupply: ${usduConfigAccount.totalSupply}`
      );
      const susduConfigAccount = await susduProgram.account.susduConfig.fetch(
        susduConfig
      );
      console.log(
        `After stake usdu, susduConfigAccount TotalSupply: ${susduConfigAccount.totalSupply}`
      );
    });
    it("unstake susdu and ready to wait cooldown", async () => {
      const caller = susduReceiver;
      const callerSusduTokenAccount = susduReceiverSusduTokenAccount.address;
      const receiver = beneficiary;
      const receiverUsduTokenAccount = beneficiaryUsduTokenAccount.address;
      const [cooldown] = PublicKey.findProgramAddressSync(
        [
          Buffer.from(vaultCooldownSeed),
          Buffer.from(usduMintToken.toBuffer()),
          Buffer.from(receiver.publicKey.toBuffer()),
          Buffer.from(caller.publicKey.toBuffer()),
        ],
        vaultProgram.programId
      );
      let sourceTokenBlacklistAccount = getBlacklistEntryPda(blacklistHookProgram, caller.publicKey);
      let destinationTokenBlacklistAccount = getBlacklistEntryPda(blacklistHookProgram, vaultConfig);
      await UnstakeSusdu(
        vaultProgram,
        susduProgram,
        blacklistHookProgram,
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
        vaultSiloUsduTokenAccount,
        11_000_000,
        extraAccountMetaList,
        sourceTokenBlacklistAccount,
        destinationTokenBlacklistAccount,
      );
      const usduConfigAccount = await usduProgram.account.usduConfig.fetch(
        usduConfig
      );
      console.log(
        `After unstake susdu, usduConfigAccount TotalSupply: ${usduConfigAccount.totalSupply}`
      );
      const susduConfigAccount = await susduProgram.account.susduConfig.fetch(
        susduConfig
      );
      console.log(
        `After unstake susdu, susduConfigAccount TotalSupply: ${susduConfigAccount.totalSupply}`
      );
      const cooldownAccount = await vaultProgram.account.cooldown.fetch(
        cooldown
      );
      console.log(
        `After unstake susdu, cooldownAccount.underlyingTokenAmount: ${cooldownAccount.underlyingTokenAmount}`
      );
    });
    it("adjust blacklist", async () => {
      const user = susduReceiver.publicKey;
      await AddToBlacklist(
        blacklistHookProgram,
        blacklistHookConfig,
        admin,
        user
      );
    });
    it("redistribute locked susdu", async () => {
      const lockedSusduTokenAccount = susduReceiverSusduTokenAccount.address;
      
      const newSusduReceiver = Keypair.generate();
      const newSusduReceiverSusduTokenAccount =
        await getOrCreateAssociatedTokenAccount(
          connection,
          susduReceiver,
          susduMintToken,
          newSusduReceiver.publicKey,
          true,
          "confirmed",
          {
            skipPreflight: true,
            commitment: "confirmed",
          },
          TOKEN_2022_PROGRAM_ID,
          ASSOCIATED_TOKEN_PROGRAM_ID
        );
    const fromBlacklistEntry = getBlacklistEntryPda(
        blacklistHookProgram,
        susduReceiver.publicKey
        );
      const toBlacklistEntry = getBlacklistEntryPda(
        blacklistHookProgram,
        newSusduReceiver.publicKey
      );
      await RedistributeLockedSusdu(
        vaultProgram,
        susduProgram,
        vaultConfig,
        susduRedistributor,
        fromBlacklistEntry,
        toBlacklistEntry,
        admin,
        accessRegistry,
        vaultAdmin,
        susduConfig,
        susduMintToken,
        lockedSusduTokenAccount,
        newSusduReceiverSusduTokenAccount.address,
        newSusduReceiver.publicKey
      );
    });
  });
});
