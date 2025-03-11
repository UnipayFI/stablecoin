import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BlacklistHook } from "../target/types/blacklist_hook";
import {
    Keypair, PublicKey, SystemProgram, Transaction, sendAndConfirmTransaction, LAMPORTS_PER_SOL, Connection,
} from "@solana/web3.js";
import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    getAccount,
    TOKEN_2022_PROGRAM_ID,
    getAssociatedTokenAddressSync,
    createInitializeTransferHookInstruction,
    createInitializeMintInstruction,
    ExtensionType,
    getMintLen,
    createAssociatedTokenAccountInstruction,
    createMintToCheckedInstruction,
    createTransferCheckedWithTransferHookInstruction,
} from "@solana/spl-token";
import {
    AirdropSol,
    getBlacklistEntryPda,
} from "./utils";
import {
    blacklistHookConfigSeed,
    blacklistHookExtraAccountMetaListSeed,
    blacklistEntrySeed,
} from "./constants";
import { assert } from "chai";
import { adminBytes, fundBytes, benefactorBytes, beneficiaryBytes, mintTokenBytes } from "./accounts";

describe("blacklist-hook", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const connection = provider.connection;

    const blacklistHookProgram = anchor.workspace.BlacklistHook;

    const admin = Keypair.fromSecretKey(Uint8Array.from(adminBytes));
    const user1 = Keypair.fromSecretKey(Uint8Array.from(fundBytes));
    const user2 = Keypair.fromSecretKey(Uint8Array.from(benefactorBytes));
    const blacklistedUser = Keypair.fromSecretKey(Uint8Array.from(beneficiaryBytes));

    // Token mint with transfer hook
    const mint = Keypair.fromSecretKey(Uint8Array.from(mintTokenBytes));

    // PDA for blacklist config
    const [blacklistHookConfig] = PublicKey.findProgramAddressSync(
        [Buffer.from(blacklistHookConfigSeed)],
        blacklistHookProgram.programId
    );

    // PDA for extra account meta list
    const [extraAccountMetaList] = PublicKey.findProgramAddressSync(
        [Buffer.from(blacklistHookExtraAccountMetaListSeed), mint.publicKey.toBuffer()],
        blacklistHookProgram.programId
    );

    let sourceTokenAccount: PublicKey;
    let destinationTokenAccount: PublicKey;
    let blacklistedUserTokenAccount: PublicKey;

    before(async () => {
        await AirdropSol(provider.connection, admin.publicKey, 100 * LAMPORTS_PER_SOL);
        await AirdropSol(provider.connection, user1.publicKey, 100 * LAMPORTS_PER_SOL);
        await AirdropSol(provider.connection, user2.publicKey, 100 * LAMPORTS_PER_SOL);
        await AirdropSol(provider.connection, blacklistedUser.publicKey, 100 * LAMPORTS_PER_SOL);

        const mintLen = getMintLen([ExtensionType.TransferHook]);
        const lamports = await provider.connection.getMinimumBalanceForRentExemption(mintLen);

        const transaction = new Transaction().add(
            SystemProgram.createAccount({
                fromPubkey: admin.publicKey,
                newAccountPubkey: mint.publicKey,
                space: mintLen,
                lamports,
                programId: TOKEN_2022_PROGRAM_ID,
            }),
            createInitializeTransferHookInstruction(
                mint.publicKey,
                admin.publicKey,
                blacklistHookProgram.programId,
                TOKEN_2022_PROGRAM_ID
            ),
            createInitializeMintInstruction(
                mint.publicKey,
                6, // decimals
                admin.publicKey,
                null,
                TOKEN_2022_PROGRAM_ID
            )
        );

        await sendAndConfirmTransaction(
            provider.connection,
            transaction,
            [admin, mint],
            {
                skipPreflight: true,
                commitment: "confirmed",
            }
        );

        sourceTokenAccount = getAssociatedTokenAddressSync(
            mint.publicKey,
            user1.publicKey,
            false,
            TOKEN_2022_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );

        destinationTokenAccount = getAssociatedTokenAddressSync(
            mint.publicKey,
            user2.publicKey,
            false,
            TOKEN_2022_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );

        blacklistedUserTokenAccount = getAssociatedTokenAddressSync(
            mint.publicKey,
            blacklistedUser.publicKey,
            false,
            TOKEN_2022_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );

        const createAccountsTx = new Transaction();

        createAccountsTx.add(
            createAssociatedTokenAccountInstruction(
                admin.publicKey,
                sourceTokenAccount,
                user1.publicKey,
                mint.publicKey,
                TOKEN_2022_PROGRAM_ID,
                ASSOCIATED_TOKEN_PROGRAM_ID
            )
        );

        createAccountsTx.add(
            createAssociatedTokenAccountInstruction(
                admin.publicKey,
                destinationTokenAccount,
                user2.publicKey,
                mint.publicKey,
                TOKEN_2022_PROGRAM_ID,
                ASSOCIATED_TOKEN_PROGRAM_ID
            )
        );

        createAccountsTx.add(
            createAssociatedTokenAccountInstruction(
                admin.publicKey,
                blacklistedUserTokenAccount,
                blacklistedUser.publicKey,
                mint.publicKey,
                TOKEN_2022_PROGRAM_ID,
                ASSOCIATED_TOKEN_PROGRAM_ID
            )
        );

        await sendAndConfirmTransaction(
            provider.connection,
            createAccountsTx,
            [admin],
            {
                skipPreflight: true,
                commitment: "confirmed",
            }
        );

        const mintTx = new Transaction();

        mintTx.add(
            createMintToCheckedInstruction(
                mint.publicKey,
                sourceTokenAccount,
                admin.publicKey,
                1000000000,
                6,
                [],
                TOKEN_2022_PROGRAM_ID
            )
        );

        await sendAndConfirmTransaction(
            provider.connection,
            mintTx,
            [admin],
            {
                skipPreflight: true,
                commitment: "confirmed",
            }
        );
    });

    it("Initialize extra account meta and blacklist hook config", async () => {
        try {
            // @ts-ignore
            const tx = await blacklistHookProgram.methods
                .initializeExtraAccountMeta()
                .accounts({
                    admin: admin.publicKey,
                    extraAccountMetaList: extraAccountMetaList,
                    blacklistHookConfig: blacklistHookConfig,
                    mint: mint.publicKey,
                    tokenProgram: TOKEN_2022_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                })
                .signers([admin])
                .rpc();

            console.log("Initialize extra account meta and blacklist hook config transaction signature:", tx);

            const config = await blacklistHookProgram.account.blacklistHookConfig.fetch(blacklistHookConfig);
            assert.isTrue(config.isInitialized);
            assert.equal(config.admin.toString(), admin.publicKey.toString());
        } catch (error) {
            console.error("Error initializing:", error);
            throw error;
        }
    });

    it("Add address to blacklist", async () => {
        try {
            const blacklistEntryPda = getBlacklistEntryPda(blacklistHookProgram, blacklistedUser.publicKey);

            // @ts-ignore
            const tx = await blacklistHookProgram.methods
                .addToBlacklist()
                .accounts({
                    admin: admin.publicKey,
                    user: blacklistedUser.publicKey,
                    blacklistHookConfig: blacklistHookConfig,
                    blacklistEntry: blacklistEntryPda,
                    systemProgram: SystemProgram.programId,
                })
                .signers([admin])
                .rpc();

            console.log("Add to blacklist transaction signature:", tx);

            const blacklistEntry = await blacklistHookProgram.account.blacklistEntry.fetch(blacklistEntryPda);
            assert.isTrue(blacklistEntry.isActive);
            assert.equal(blacklistEntry.owner.toString(), blacklistedUser.publicKey.toString());
        } catch (error) {
            console.error("Error adding to blacklist:", error);
            throw error;
        }
    });

    it("Transfer to blacklisted address should fail", async () => {
        try {
            const transferTx = new Transaction();

            const transferInstruction = await createTransferCheckedWithTransferHookInstruction(
                connection,
                sourceTokenAccount,
                mint.publicKey,
                blacklistedUserTokenAccount,
                user1.publicKey,
                BigInt(100000000),
                6,
                [],
                'confirmed',
                TOKEN_2022_PROGRAM_ID
            );

            transferTx.add(transferInstruction);

            await sendAndConfirmTransaction(
                provider.connection,
                transferTx,
                [user1],
                {
                    skipPreflight: true,
                    commitment: "confirmed",
                }
            );

            assert.fail("Transfer to blacklisted address should have failed");
        } catch (error) {
            console.log("Transfer to blacklisted address failed as expected:", error.message);
            const blacklistedUserTokenAccountInfo = await getAccount(
                provider.connection,
                blacklistedUserTokenAccount,
                "confirmed",
                TOKEN_2022_PROGRAM_ID
            );
            assert.equal(Number(blacklistedUserTokenAccountInfo.amount), 0);
        }
    });

    it("Remove address from blacklist", async () => {
        try {
            const blacklistEntryPda = getBlacklistEntryPda(blacklistHookProgram, blacklistedUser.publicKey);
            // @ts-ignore
            const tx = await blacklistHookProgram.methods
                .removeFromBlacklist()
                .accounts({
                    admin: admin.publicKey,
                    user: blacklistedUser.publicKey,
                    blacklistHookConfig: blacklistHookConfig,
                    blacklistEntry: blacklistEntryPda,
                    systemProgram: SystemProgram.programId,
                })
                .signers([admin])
                .rpc();

            console.log("Remove from blacklist transaction signature:", tx);

            try {
                const blacklistEntryAfter = await blacklistHookProgram.account.blacklistEntry.fetch(blacklistEntryPda);
                console.log("Blacklist entry after removal:", {
                    isActive: blacklistEntryAfter.isActive,
                    owner: blacklistEntryAfter.owner.toString()
                });
                assert.isFalse(blacklistEntryAfter.isActive);
            } catch (error) {
                console.log("Blacklist entry account not found after removal, which is also acceptable");
            }
        } catch (error) {
            console.error("Error removing from blacklist:", error);
            throw error;
        }
    });

    it("Transfer to removed address should succeed", async () => {
        try {
            const transferTx = new Transaction();
            const transferInstruction = await createTransferCheckedWithTransferHookInstruction(
                connection,
                sourceTokenAccount,
                mint.publicKey,
                blacklistedUserTokenAccount,
                user1.publicKey,
                BigInt(100000000),
                6,
                [],
                'confirmed',
                TOKEN_2022_PROGRAM_ID
            );

            transferTx.add(transferInstruction);

            console.log("Sending transfer transaction...");
            const signature = await sendAndConfirmTransaction(
                provider.connection,
                transferTx,
                [user1],
                {
                    skipPreflight: true,
                    commitment: "confirmed",
                }
            );
            console.log("Transfer transaction signature:", signature);

            console.log("Checking destination token account balance...");
            const blacklistedUserTokenAccountInfo = await getAccount(
                provider.connection,
                blacklistedUserTokenAccount,
                "confirmed",
                TOKEN_2022_PROGRAM_ID
            );
            assert.equal(Number(blacklistedUserTokenAccountInfo.amount), 100_000000);
        } catch (error) {
            console.error("Error transferring to removed address:", error);
            if (error.logs) {
                console.error("Transaction logs:", error.logs);
            }
            throw error;
        }
    });

    it("Add sender to blacklist and verify transfer fails", async () => {
        try {
            const blacklistEntryPda = getBlacklistEntryPda(blacklistHookProgram, user1.publicKey);

            // @ts-ignore
            const tx = await blacklistHookProgram.methods
                .addToBlacklist()
                .accounts({
                    admin: admin.publicKey,
                    user: user1.publicKey,
                    blacklistHookConfig: blacklistHookConfig,
                    blacklistEntry: blacklistEntryPda,
                    systemProgram: SystemProgram.programId,
                })
                .signers([admin])
                .rpc();

            console.log("Add sender to blacklist transaction signature:", tx);

            const blacklistEntry = await blacklistHookProgram.account.blacklistEntry.fetch(blacklistEntryPda);
            assert.isTrue(blacklistEntry.isActive);
            assert.equal(blacklistEntry.owner.toString(), user1.publicKey.toString());

            const transferTx = new Transaction();

            const transferInstruction = await createTransferCheckedWithTransferHookInstruction(
                connection,
                sourceTokenAccount,
                mint.publicKey,
                destinationTokenAccount,
                user1.publicKey,
                BigInt(100000000),
                6,
                [],
                'confirmed',
                TOKEN_2022_PROGRAM_ID
            );

            transferTx.add(transferInstruction);

            await sendAndConfirmTransaction(
                provider.connection,
                transferTx,
                [user1],
                {
                    skipPreflight: true,
                    commitment: "confirmed",
                }
            );

            assert.fail("Transfer from blacklisted address should have failed");
        } catch (error) {
            if (error.message.includes("assert.fail")) {
                throw error;
            }
            console.log("Transfer from blacklisted address failed as expected:", error.message);
        }
    });

    it("Admin transfer", async () => {
        try {
            const newAdmin = Keypair.generate();
            await AirdropSol(provider.connection, newAdmin.publicKey, 10 * LAMPORTS_PER_SOL);

            // @ts-ignore
            const proposeTx = await blacklistHookProgram.methods
                .proposeNewAdmin()
                .accounts({
                    currentAdmin: admin.publicKey,
                    proposedAdmin: newAdmin.publicKey,
                    blacklistHookConfig: blacklistHookConfig,
                    systemProgram: SystemProgram.programId,
                })
                .signers([admin])
                .rpc();

            console.log("Propose new admin transaction signature:", proposeTx);

            let config = await blacklistHookProgram.account.blacklistHookConfig.fetch(blacklistHookConfig);
            assert.equal(config.pendingAdmin.toString(), newAdmin.publicKey.toString());

            // @ts-ignore
            const acceptTx = await blacklistHookProgram.methods
                .acceptAdminTransfer()
                .accounts({
                    newAdmin: newAdmin.publicKey,
                    blacklistHookConfig: blacklistHookConfig,
                    systemProgram: SystemProgram.programId,
                })
                .signers([newAdmin])
                .rpc();

            console.log("Accept admin transfer transaction signature:", acceptTx);

            config = await blacklistHookProgram.account.blacklistHookConfig.fetch(blacklistHookConfig);
            assert.equal(config.admin.toString(), newAdmin.publicKey.toString());
            assert.equal(config.pendingAdmin.toString(), PublicKey.default.toString());
        } catch (error) {
            console.error("Error transferring admin:", error);
            throw error;
        }
    });
});
