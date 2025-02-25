import * as anchor from "@coral-xyz/anchor";
import { Program} from "@coral-xyz/anchor";
import { 
    Keypair, PublicKey, Connection, LAMPORTS_PER_SOL, Transaction, sendAndConfirmTransaction,
} from "@solana/web3.js";
import { 
    ASSOCIATED_TOKEN_PROGRAM_ID,
    getAccount,
    TOKEN_2022_PROGRAM_ID,
    getOrCreateAssociatedTokenAccount,
    createApproveInstruction,
    createMintToInstruction,
    Account,
    getAssociatedTokenAddressSync
} from "@solana/spl-token";
import { Guardian } from "../target/types/guardian";
import { Usdu } from "../target/types/usdu";
import { Susdu } from "../target/types/susdu";
import { Vault } from "../target/types/vault";
import { 
    accessRegistrySeed,
    susduSeed,
    usduSeed, 
    vaultConfigSeed,
    vaultStakePoolUsduTokenAccountSeed, 
    vaultSlioUsduTokenAccountSeed, 
    vaultStateSeed, 
    vaultCooldownSeed,
    vaultSusduTokenAccountSeed, 
    vaultUsduTokenAccountSeed, 
    usduConfigSeed,
    susduConfigSeed,
    vaultBlacklistSeed,
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
    AdjustBlacklist,
    RedistributeLockedSusdu,
} from "./utils";
import { assert } from "chai";
import { adminBytes, fundBytes, benefactorBytes, beneficiaryBytes, susduRecevierBytes } from "./accounts";

const NEED_AIRDROP_SOL = true;

describe("Stablecoin Preparation", () => {
    let guardianProgram: Program<Guardian>;
    let usduProgram: Program<Usdu>;
    let susduProgram: Program<Susdu>;
    let vaultProgram: Program<Vault>;

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
    let vaultSlioUsduTokenAccount: PublicKey;
    let connection: Connection;
    let usduMinter: PublicKey;
    let collateralDepositor: PublicKey;
    let usduRedeemer: PublicKey;
    let collateralWithdrawer: PublicKey;
    let susduMinter: PublicKey;
    let susduRedeemer: PublicKey;
    let grandMaster: PublicKey;
    let susduRedistributor: PublicKey;
    before(async () => {
        const provider = anchor.AnchorProvider.env();
        connection = provider.connection;
        guardianProgram = anchor.workspace.Guardian as Program<Guardian>;
        usduProgram = anchor.workspace.Usdu as Program<Usdu>;
        susduProgram = anchor.workspace.Susdu as Program<Susdu>;
        vaultProgram = anchor.workspace.Vault as Program<Vault>;

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
        [vaultSlioUsduTokenAccount] = PublicKey.findProgramAddressSync(
            [Buffer.from(vaultSlioUsduTokenAccountSeed)],
            vaultProgram.programId
        );

        if (NEED_AIRDROP_SOL) {
            await Promise.all([
                AirdropSol(connection, admin.publicKey, LAMPORTS_PER_SOL * 100),
            ])
        }

        // init guardian access registry
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

        // assign usdu_minter role to vault_config
        usduMinter =await AssignRole(guardianProgram, accessRegistry, admin, vaultConfig, "usdu_minter");
        // assign susdu_minter role to vault_config
        susduMinter = await AssignRole(guardianProgram, accessRegistry, admin, vaultConfig, "susdu_minter");
        // assign usdu_redeemer role to vault_config
        usduRedeemer = await AssignRole(guardianProgram, accessRegistry, admin, vaultConfig, "usdu_redeemer");
        // assign susdu_redeemer role to vault_config
        susduRedeemer = await AssignRole(guardianProgram, accessRegistry, admin, vaultConfig, "susdu_redeemer");
        // assign vault_susdu_redistributor role to vault_config
        susduRedistributor = await AssignRole(guardianProgram, accessRegistry, admin, vaultConfig, "susdu_redistributor");
        // assign vault_usdu_minter role to admin
        collateralDepositor = await AssignRole(guardianProgram, accessRegistry, admin, admin.publicKey, "collateral_depositor");
        // assign vault_usdu_redeemer role to admin
        collateralWithdrawer = await AssignRole(guardianProgram, accessRegistry, admin, admin.publicKey, "collateral_withdrawer");
        // assign vault_manager role to admin
        grandMaster = await AssignRole(guardianProgram, accessRegistry, admin, admin.publicKey, "grand_master");
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
        const mintToken = Keypair.generate();
        let benefactor = Keypair.fromSecretKey(Uint8Array.from(benefactorBytes));
        let fund = Keypair.fromSecretKey(Uint8Array.from(fundBytes));
        let beneficiary = Keypair.fromSecretKey(Uint8Array.from(beneficiaryBytes));

        let beneficiaryUsduTokenAccount: Account;
        let beneficiaryCollateralTokenAccount: Account;
        let fundCollateralTokenAccount: Account;
        let vaultCollateralTokenAccount: PublicKey;
        let benefactorCollateralTokenAccount: Account;
        let benefactorUsduTokenAccount: Account;

        let susduReceiver = Keypair.fromSecretKey(Uint8Array.from(susduRecevierBytes));
        let susduReceiverSusduTokenAccount: Account;

        before(async () => {
            if (NEED_AIRDROP_SOL) {
                await Promise.all([
                    AirdropSol(connection, beneficiary.publicKey, LAMPORTS_PER_SOL * 100),
                    AirdropSol(connection, fund.publicKey, LAMPORTS_PER_SOL * 100),
                    AirdropSol(connection, benefactor.publicKey, LAMPORTS_PER_SOL * 100),
                    AirdropSol(connection, susduReceiver.publicKey, LAMPORTS_PER_SOL * 100),
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
            beneficiaryCollateralTokenAccount = await getOrCreateAssociatedTokenAccount(
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

            vaultCollateralTokenAccount = getAssociatedTokenAddressSync(
                mintToken.publicKey,
                vaultConfig,
                true,
                TOKEN_2022_PROGRAM_ID,
                ASSOCIATED_TOKEN_PROGRAM_ID
            );

            benefactorCollateralTokenAccount = await getOrCreateAssociatedTokenAccount(
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
        });

        it ("mint token, approve, deposit", async () => {
            const mintIx = createMintToInstruction(
                mintToken.publicKey,
                benefactorCollateralTokenAccount.address,
                admin.publicKey,
                100_000_000_000,
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
            const approveIx = createApproveInstruction(
                benefactorCollateralTokenAccount.address,
                vaultConfig,
                benefactor.publicKey,
                100_000_000_000,
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
                1200_000_000,
                1000_000_000,
                benefactorCollateralTokenAccount.address,
                beneficiaryUsduTokenAccount.address,
                fundCollateralTokenAccount.address,
                vaultCollateralTokenAccount,
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
            await sendAndConfirmTransaction(
                connection,
                approveCollateralTx,
                [fund],
                {
                    skipPreflight: true,
                    commitment: "confirmed",
                }
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
            await sendAndConfirmTransaction(
                connection,
                approveUsduTx,
                [beneficiary],
                {
                    skipPreflight: true,
                    commitment: "confirmed",
                }
            );

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
        });

        it("stake usdu and mint susdu", async () => {
            const caller = beneficiary;
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
        });
        it("unstake susdu and ready to wait cooldown", async () => {
            const caller = susduReceiver;
            const callerSusduTokenAccount = susduReceiverSusduTokenAccount.address;
            const receiver = beneficiary;
            const receiverUsduTokenAccount = beneficiaryUsduTokenAccount.address;
            const blacklistState = PublicKey.findProgramAddressSync(
                [Buffer.from(vaultBlacklistSeed), beneficiary.publicKey.toBuffer()],
                vaultProgram.programId
            )[0];
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
        });
        it("withdraw usdu", async () => {
            const receiver = beneficiary;
            const receiverUsduTokenAccount = beneficiaryUsduTokenAccount.address;
            const blacklistState = PublicKey.findProgramAddressSync(
                [Buffer.from(vaultBlacklistSeed), beneficiary.publicKey.toBuffer()],
                vaultProgram.programId
            )[0];
            const [cooldown] = PublicKey.findProgramAddressSync(
                [
                    Buffer.from(vaultCooldownSeed),
                    Buffer.from(usduMintToken.toBuffer()),
                    Buffer.from(receiver.publicKey.toBuffer()),
                ],
                vaultProgram.programId
            );
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
        });
        it("unstake susdu and ready to wait cooldown", async () => {
            const caller = susduReceiver;
            const callerSusduTokenAccount = susduReceiverSusduTokenAccount.address;
            const receiver = beneficiary;
            const receiverUsduTokenAccount = beneficiaryUsduTokenAccount.address;
            const blacklistState = PublicKey.findProgramAddressSync(
                [Buffer.from(vaultBlacklistSeed), beneficiary.publicKey.toBuffer()],
                vaultProgram.programId
            )[0];
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
                3_000_000,
                blacklistState,
            );
        });
        it("stake usdu and mint susdu again", async () => {
            const caller = beneficiary;
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
                27_520_000,
                blacklistState,
            );
        });
        it("stake usdu and mint susdu again and again", async () => {
            const caller = beneficiary;
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
                75_010_000,
                blacklistState,
            );
        });
        it("distribute usdu reward", async () => {
            const caller = beneficiary;
            const callerUsduTokenAccount = beneficiaryUsduTokenAccount.address;
            const distributeRewarder = await AssignRole(guardianProgram, accessRegistry, admin, beneficiary.publicKey, "distribute_rewarder");
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
        });
        it("stake usdu and mint susdu again and again", async () => {
            const caller = beneficiary;
            const blacklistState = PublicKey.findProgramAddressSync(
                [Buffer.from(vaultBlacklistSeed), beneficiary.publicKey.toBuffer()],
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
                231_010_000,
                blacklistState,
            );
        });
        it("stake usdu and mint susdu again and again", async () => {
            const caller = beneficiary;
            const blacklistState = PublicKey.findProgramAddressSync(
                [Buffer.from(vaultBlacklistSeed), beneficiary.publicKey.toBuffer()],
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
                112_000_000,
                blacklistState,
            );
        });
        it("unstake susdu and ready to wait cooldown", async () => {
            const caller = susduReceiver;
            const callerSusduTokenAccount = susduReceiverSusduTokenAccount.address;
            const receiver = beneficiary;
            const receiverUsduTokenAccount = beneficiaryUsduTokenAccount.address;
            const blacklistState = PublicKey.findProgramAddressSync(
                [Buffer.from(vaultBlacklistSeed), beneficiary.publicKey.toBuffer()],
                vaultProgram.programId
            )[0];
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
                11_000_000,
                blacklistState,
            );
        });
        it("adjust blacklist", async () => {
            const user = susduReceiver.publicKey;
            await AdjustBlacklist(
                vaultProgram,
                admin,
                vaultConfig,
                accessRegistry,
                grandMaster,
                true,
                false,
                user,
            );
        });
        it("redistribute locked susdu", async () => {
            const lockedSusduTokenAccount = susduReceiverSusduTokenAccount.address;
            const blacklistState = PublicKey.findProgramAddressSync(
                [Buffer.from(vaultBlacklistSeed), susduReceiver.publicKey.toBuffer()],
                vaultProgram.programId
            )[0];
            const newSusduReceiver = Keypair.generate();
            const newSusduReceiverSusduTokenAccount = await getOrCreateAssociatedTokenAccount(
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
                newSusduReceiverSusduTokenAccount.address,
                blacklistState,
                newSusduReceiver.publicKey,
            );
        });
    });
});
