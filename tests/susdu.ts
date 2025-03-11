import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Guardian } from "../target/types/guardian";
import { Susdu } from "../target/types/susdu";
import { BlacklistHook } from "../target/types/blacklist_hook";
import {
    Keypair, PublicKey, SystemProgram, LAMPORTS_PER_SOL, Connection, Transaction, sendAndConfirmTransaction,
} from "@solana/web3.js";
import {
    TOKEN_2022_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
    getAssociatedTokenAddressSync,
    createAssociatedTokenAccountInstruction,
    createTransferCheckedWithTransferHookInstruction,
    getAccount,
} from "@solana/spl-token";
import {
    AirdropSol,
    InitGuardianAccessRegistry,
    InitAndCreateSusdu,
    InitializeBlacklistHook,
    AssignRole,
    getBlacklistEntryPda,
} from "./utils";
import {
    accessRegistrySeed,
    susduConfigSeed,
    susduSeed,
    blacklistHookConfigSeed,
    blacklistHookExtraAccountMetaListSeed,
    blacklistEntrySeed,
} from "./constants";
import { adminBytes } from "./accounts";
import { assert } from "chai";

describe("susdu with blacklist-hook", () => {
    const NEED_AIRDROP_SOL = true;
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const connection = provider.connection;

    const guardianProgram = anchor.workspace.Guardian as Program<Guardian>;
    const susduProgram = anchor.workspace.Susdu as Program<Susdu>;
    const blacklistHookProgram = anchor.workspace.BlacklistHook as Program<BlacklistHook>;

    const admin = Keypair.fromSecretKey(Uint8Array.from(adminBytes));
    const hacker = Keypair.generate();
    const normalUser = Keypair.generate();

    const [blacklistHookConfig] = PublicKey.findProgramAddressSync(
        [Buffer.from(blacklistHookConfigSeed)],
        blacklistHookProgram.programId
    );

    let accessRegistry: PublicKey;
    let susduConfig: PublicKey;
    let susduMinter: PublicKey;
    let susduToken: PublicKey;
    let extraAccountMetaList: PublicKey;

    let adminTokenAccount: PublicKey;
    let hackerTokenAccount: PublicKey;
    let normalUserTokenAccount: PublicKey;

    before(async () => {
        [accessRegistry] = PublicKey.findProgramAddressSync(
            [Buffer.from(accessRegistrySeed)],
            guardianProgram.programId
        );
        [susduConfig] = PublicKey.findProgramAddressSync(
            [Buffer.from(susduConfigSeed)],
            susduProgram.programId
        );
        [susduToken] = PublicKey.findProgramAddressSync(
            [Buffer.from(susduSeed)],
            susduProgram.programId
        );

        if (NEED_AIRDROP_SOL) {
            await Promise.all([
                AirdropSol(connection, admin.publicKey, LAMPORTS_PER_SOL * 100),
                AirdropSol(connection, hacker.publicKey, LAMPORTS_PER_SOL * 100),
                AirdropSol(connection, normalUser.publicKey, LAMPORTS_PER_SOL * 100),
            ])
        }

        await InitGuardianAccessRegistry(guardianProgram, accessRegistry, admin);
        await InitAndCreateSusdu(susduProgram, blacklistHookProgram, susduToken, accessRegistry, susduConfig, admin);

        extraAccountMetaList = PublicKey.findProgramAddressSync(
            [Buffer.from(blacklistHookExtraAccountMetaListSeed), susduToken.toBuffer()],
            blacklistHookProgram.programId
        )[0];

        await InitializeBlacklistHook(blacklistHookProgram, admin, extraAccountMetaList, blacklistHookConfig, susduToken);

        susduMinter = await AssignRole(guardianProgram, accessRegistry, admin, admin.publicKey, "susdu_minter");

        adminTokenAccount = getAssociatedTokenAddressSync(
            susduToken,
            admin.publicKey,
            false,
            TOKEN_2022_PROGRAM_ID
        );

        hackerTokenAccount = getAssociatedTokenAddressSync(
            susduToken,
            hacker.publicKey,
            false,
            TOKEN_2022_PROGRAM_ID
        );

        normalUserTokenAccount = getAssociatedTokenAddressSync(
            susduToken,
            normalUser.publicKey,
            false,
            TOKEN_2022_PROGRAM_ID
        );

        const createAccountsTx = new Transaction();

        createAccountsTx.add(
            createAssociatedTokenAccountInstruction(
                admin.publicKey,
                adminTokenAccount,
                admin.publicKey,
                susduToken,
                TOKEN_2022_PROGRAM_ID
            )
        );

        createAccountsTx.add(
            createAssociatedTokenAccountInstruction(
                admin.publicKey,
                hackerTokenAccount,
                hacker.publicKey,
                susduToken,
                TOKEN_2022_PROGRAM_ID
            )
        );

        createAccountsTx.add(
            createAssociatedTokenAccountInstruction(
                admin.publicKey,
                normalUserTokenAccount,
                normalUser.publicKey,
                susduToken,
                TOKEN_2022_PROGRAM_ID
            )
        );

        await sendAndConfirmTransaction(
            connection,
            createAccountsTx,
            [admin],
            {
                skipPreflight: true,
                commitment: "confirmed",
            }
        );
    });

    it("should add hacker to blacklist", async () => {
        const hackerBlacklistEntryPda = getBlacklistEntryPda(blacklistHookProgram, hacker.publicKey);

        try {
            const tx = await blacklistHookProgram.methods
                .addToBlacklist()
                .accountsStrict({
                    admin: admin.publicKey,
                    user: hacker.publicKey,
                    blacklistHookConfig: blacklistHookConfig,
                    blacklistEntry: hackerBlacklistEntryPda,
                    systemProgram: SystemProgram.programId,
                })
                .signers([admin])
                .rpc({skipPreflight: true, commitment: "confirmed"});

            console.log("Add hacker to blacklist transaction signature:", tx);

            const blacklistEntry = await blacklistHookProgram.account.blacklistEntry.fetch(hackerBlacklistEntryPda);
            console.log("Blacklist entry:", {
                isActive: blacklistEntry.isActive,
                owner: blacklistEntry.owner.toString()
            });

            assert.isTrue(blacklistEntry.isActive);
            assert.equal(blacklistEntry.owner.toString(), hacker.publicKey.toString());
        } catch (error) {
            console.error("Error adding hacker to blacklist:", error);
            throw error;
        }
    });

    it("should mint susdu tokens to admin", async () => {
        try {
            const mintAmount = 1000 * 10**6; // 1000 tokens with 6 decimals

            const tx = await susduProgram.methods
                .mintSusdu(new anchor.BN(mintAmount))
                .accountsStrict({
                    authority: admin.publicKey,
                    accessRegistry: accessRegistry,
                    accessRole: susduMinter,
                    susduConfig: susduConfig,
                    susduToken: susduToken,
                    receiver: admin.publicKey,
                    receiverTokenAccount: adminTokenAccount,
                    tokenProgram: TOKEN_2022_PROGRAM_ID,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                })
                .signers([admin])
                .rpc({skipPreflight: true, commitment: "confirmed"});

            console.log("Mint SUSDU transaction signature:", tx);

            const tokenAccountInfo = await getAccount(
                connection,
                adminTokenAccount,
                "confirmed",
                TOKEN_2022_PROGRAM_ID
            );

            console.log("Admin token balance:", tokenAccountInfo.amount.toString());
            assert.equal(tokenAccountInfo.amount.toString(), mintAmount.toString());
        } catch (error) {
            console.error("Error minting SUSDU:", error);
            throw error;
        }
    });

    it("should transfer tokens to normal user", async () => {
        try {
            const transferAmount = 100 * 10**6; // 100 tokens

            const transferTx = new Transaction();

            const transferInstruction = await createTransferCheckedWithTransferHookInstruction(
                connection,
                adminTokenAccount,
                susduToken,
                normalUserTokenAccount,
                admin.publicKey,
                BigInt(transferAmount),
                6,
                [],
                'confirmed',
                TOKEN_2022_PROGRAM_ID
            );

            transferTx.add(transferInstruction);

            const signature = await sendAndConfirmTransaction(
                provider.connection,
                transferTx,
                [admin],
                {
                    skipPreflight: true,
                    commitment: "confirmed",
                }
            );

            console.log("Transfer to normal user transaction signature:", signature);

            const normalUserTokenAccountInfo = await getAccount(
                connection,
                normalUserTokenAccount,
                "confirmed",
                TOKEN_2022_PROGRAM_ID
            );

            console.log("Normal user token balance:", normalUserTokenAccountInfo.amount.toString());
            assert.equal(normalUserTokenAccountInfo.amount.toString(), transferAmount.toString());
        } catch (error) {
            console.error("Error transferring to normal user:", error);
            throw error;
        }
    });

    it("should fail to transfer tokens to blacklisted user", async () => {
        try {
            const transferAmount = 100 * 10**6; // 100 tokens

            const transferTx = new Transaction();

            const transferInstruction = await createTransferCheckedWithTransferHookInstruction(
                connection,
                adminTokenAccount,
                susduToken,
                hackerTokenAccount,
                admin.publicKey,
                BigInt(transferAmount),
                6,
                [],
                'confirmed',
                TOKEN_2022_PROGRAM_ID
            );

            transferTx.add(transferInstruction);

            await sendAndConfirmTransaction(
                provider.connection,
                transferTx,
                [admin],
                {
                    skipPreflight: true,
                    commitment: "confirmed",
                }
            );

            assert.fail("Transfer to blacklisted user should have failed");
        } catch (error) {
            console.log("Transfer to blacklisted user failed as expected:", error.message);

            const hackerTokenAccountInfo = await getAccount(
                connection,
                hackerTokenAccount,
                "confirmed",
                TOKEN_2022_PROGRAM_ID
            );

            console.log("Hacker token balance:", hackerTokenAccountInfo.amount.toString());
            assert.equal(hackerTokenAccountInfo.amount.toString(), "0");
        }
    });

    it("should remove user from blacklist", async () => {
        const hackerBlacklistEntryPda = getBlacklistEntryPda(blacklistHookProgram, hacker.publicKey);

        try {
            const tx = await blacklistHookProgram.methods
                .removeFromBlacklist()
                .accountsStrict({
                    admin: admin.publicKey,
                    user: hacker.publicKey,
                    blacklistHookConfig: blacklistHookConfig,
                    blacklistEntry: hackerBlacklistEntryPda,
                    systemProgram: SystemProgram.programId,
                })
                .signers([admin])
                .rpc({skipPreflight: true, commitment: "confirmed"});

            console.log("Remove from blacklist transaction signature:", tx);

            try {
                const blacklistEntry = await blacklistHookProgram.account.blacklistEntry.fetch(hackerBlacklistEntryPda);
                console.log("Blacklist entry after removal:", {
                    isActive: blacklistEntry.isActive,
                    owner: blacklistEntry.owner.toString()
                });
            } catch (error) {
                console.log("Blacklist entry not found as expected:", error.message);
            }
        } catch (error) {
            console.error("Error removing from blacklist:", error);
            throw error;
        }
    });

    it("should transfer tokens to removed user", async () => {
        try {
            const transferAmount = 100 * 10**6; // 100 tokens

            const transferTx = new Transaction();

            const transferInstruction = await createTransferCheckedWithTransferHookInstruction(
                connection,
                adminTokenAccount,
                susduToken,
                hackerTokenAccount,
                admin.publicKey,
                BigInt(transferAmount),
                6,
                [],
                'confirmed',
                TOKEN_2022_PROGRAM_ID
            );

            transferTx.add(transferInstruction);

            const signature = await sendAndConfirmTransaction(
                provider.connection,
                transferTx,
                [admin],
                {
                    skipPreflight: true,
                    commitment: "confirmed",
                }
            );

            console.log("Transfer to removed user transaction signature:", signature);

            const hackerTokenAccountInfo = await getAccount(
                connection,
                hackerTokenAccount,
                "confirmed",
                TOKEN_2022_PROGRAM_ID
            );

            console.log("Removed user token balance:", hackerTokenAccountInfo.amount.toString());
            assert.equal(hackerTokenAccountInfo.amount.toString(), transferAmount.toString());
        } catch (error) {
            console.error("Error transferring to removed user:", error);
            throw error;
        }
    });
});
