import { Program, BN, web3 } from "@coral-xyz/anchor";
import { Guardian } from "../target/types/guardian";
import { Connection, Keypair, PublicKey, SystemProgram, Transaction, sendAndConfirmTransaction, LAMPORTS_PER_SOL } from "@solana/web3.js";
import {
    createAssociatedTokenAccountInstruction,
    createSyncNativeInstruction,
    getAssociatedTokenAddress,
    NATIVE_MINT,
    TOKEN_2022_PROGRAM_ID,
    createCloseAccountInstruction,
    NATIVE_MINT_2022,
    createInitializeMintInstruction,
    MINT_SIZE,
    ASSOCIATED_TOKEN_PROGRAM_ID
} from "@solana/spl-token";
import { accessRoleSeed, roleToBytes, getRole, vaultBlacklistSeed } from "./constants";
import { Vault } from "../target/types/vault";
import { Usdu } from "../target/types/usdu";
import { Susdu } from "../target/types/susdu";

export async function AirdropSol(connection: Connection, pubkey: PublicKey, amount: number) {
    const tx = await connection.requestAirdrop(pubkey, amount);
    const latestBlockHash = await connection.getLatestBlockhash();
    const strategy = {
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: tx,
    };
    await connection.confirmTransaction(strategy, "confirmed");
    console.log("Airdrop Sol Transaction signature:", tx);
}

export async function WrapSol(
    connection: Connection,
    payer: Keypair,
    amount: number // amount in SOL
) {
    const wsolTokenAccount = await getAssociatedTokenAddress(
        NATIVE_MINT_2022,  // wSOL mint 2022
        payer.publicKey
    );

    const tx = new Transaction();
    try {
        await connection.getTokenAccountBalance(wsolTokenAccount);
    } catch {
        tx.add(
            createAssociatedTokenAccountInstruction(
                payer.publicKey,     // payer
                wsolTokenAccount,    // token account
                payer.publicKey,     // owner
                NATIVE_MINT          // mint
            )
        );
    }
    tx.add(
        SystemProgram.transfer({
            fromPubkey: payer.publicKey,
            toPubkey: wsolTokenAccount,
            lamports: amount * LAMPORTS_PER_SOL
        })
    );
    tx.add(createSyncNativeInstruction(wsolTokenAccount));
    await sendAndConfirmTransaction(connection, tx, [payer]);

    return wsolTokenAccount;
}

export async function UnwrapSol(
    connection: Connection,
    payer: Keypair
) {
    const wsolTokenAccount = await getAssociatedTokenAddress(
        NATIVE_MINT,
        payer.publicKey
    );
    const tx = new Transaction().add(
        createCloseAccountInstruction(
            wsolTokenAccount,
            payer.publicKey,
            payer.publicKey
        )
    );

    await sendAndConfirmTransaction(connection, tx, [payer]);
}

export async function InitGuardianAccessRegistry(
    program: Program<Guardian>,
    accessRegistry: PublicKey,
    admin: Keypair,
) {
    try {
        const tx = await program.methods
            .initAccessRegistry()
            .accountsStrict({
                admin: admin.publicKey,
                accessRegistry: accessRegistry,
                systemProgram: SystemProgram.programId,
            })
            .signers([admin])
            .rpc();

        console.log("Init Access Registry Transaction signature:", tx);
    } catch (error) {
        console.error("Error:", error);
        const accessRegistryAccount = await program.account.accessRegistry.fetch(accessRegistry);
        console.log("Access Registry:", accessRegistryAccount);
        console.log("Access Registry bump:", accessRegistryAccount.bump);
        console.log("Access Registry admin:", accessRegistryAccount.admin);
        console.log("Access Registry is initialized:", accessRegistryAccount.isInitialized);
    }
    await new Promise(resolve => setTimeout(resolve, 1000));
}

export async function AssignRole(
    guardianProgram: Program<Guardian>,
    accessRegistry: PublicKey,
    admin: Keypair,
    owner: PublicKey,
    role: string,
): Promise<PublicKey> {
    const [assignRole] = PublicKey.findProgramAddressSync(
        [
            Buffer.from(accessRoleSeed),
            accessRegistry.toBuffer(),
            owner.toBuffer(),
            roleToBytes(role),
        ],
        guardianProgram.programId
    );
    console.log(`Assigning role ${role} to ${owner.toBase58()}`);
    console.log(`AssignRole address: ${assignRole.toBase58()}`);

    const [guardianAdmin] = PublicKey.findProgramAddressSync(
        [
            Buffer.from(accessRoleSeed),
            accessRegistry.toBuffer(),
            admin.publicKey.toBuffer(),
            roleToBytes("guardian_admin"),
        ],
        guardianProgram.programId
    );

    const roleType = getRole(role);
    try {
        const tx = await guardianProgram.methods
            .assignRole(roleType)
            .accountsStrict({
                authority: admin.publicKey,
                guardianAdmin: guardianAdmin, // admin no need to initialize this account
                user: owner,
                accessRegistry: accessRegistry,
                assignRole: assignRole,
                systemProgram: SystemProgram.programId,
            })
            .signers([admin])
            .rpc();

        console.log("Assign Role Transaction signature:", tx);
    } catch (error) {
        console.error("Error:", error);
    }
    await new Promise(resolve => setTimeout(resolve, 1000));
    return assignRole;
}

export async function InitAndCreateUSDU(
    usduProgram: Program<Usdu>,
    usdu: PublicKey,
    accessRegistry: PublicKey,
    usduConfig: PublicKey,
    admin: Keypair,
) {
    try {
        console.log(`Init Usdu Config: ${usduConfig.toBase58()}`);
        const tx = await usduProgram.methods
            .initConfig()
            .accountsStrict({
                admin: admin.publicKey,
                usduConfig: usduConfig,
                accessRegistry: accessRegistry,
                systemProgram: SystemProgram.programId,
            })
            .signers([admin])
            .rpc();

        console.log("Init Usdu Config Transaction signature:", tx);
    } catch (error) {
        console.error("Error:", error);
    }
    try {
        console.log(`Create Usdu: ${usdu.toBase58()}`);
        const tx = await usduProgram.methods
            .createUsdu(6)
            .accountsStrict({
                admin: admin.publicKey,
                usduConfig: usduConfig,
                usduToken: usdu,
                tokenProgram: TOKEN_2022_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
            })
            .signers([admin])
            .rpc();

        console.log("Create Usdu Transaction signature:", tx);
    } catch (error) {
        console.error("Error:", error);
    }
    await new Promise(resolve => setTimeout(resolve, 1000));
}

export async function InitAndCreateSusdu(
    susduProgram: Program<Susdu>,
    susdu: PublicKey,
    accessRegistry: PublicKey,
    susduConfig: PublicKey,
    admin: Keypair,
) {
    try {
        console.log(`Init Susdu Config: ${susduConfig.toBase58()}`);
        const tx = await susduProgram.methods
            .initConfig()
            .accountsStrict({
                admin: admin.publicKey,
                susduConfig: susduConfig,
                accessRegistry: accessRegistry,
                systemProgram: SystemProgram.programId,
            })
            .signers([admin])
            .rpc();

        console.log("Init Susdu Config Transaction signature:", tx);
    } catch (error) {
        console.error("Error:", error);
    }

    try {
        console.log(`Create Susdu: ${susdu.toBase58()}`);
        const tx = await susduProgram.methods
            .createSusdu(6)
            .accountsStrict({
                admin: admin.publicKey,
                susduConfig: susduConfig,
                susduToken: susdu,
                tokenProgram: TOKEN_2022_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
            })
            .signers([admin])
            .rpc();

        console.log("Create Susdu Transaction signature:", tx);
    } catch (error) {
        console.error("Error:", error);
    }
    await new Promise(resolve => setTimeout(resolve, 1000));
}

export async function InitVaultConfig(
    vaultProgram: Program<Vault>,
    vaultConfig: PublicKey,
    accessRegistry: PublicKey,
    usdu: PublicKey,
    susdu: PublicKey,
    admin: Keypair,
    cooldownDuration: number,
) {
    console.log(`Init Vault Config: ${vaultConfig.toBase58()}`);
    try {
        const tx = await vaultProgram.methods
            .initVaultConfig(new BN(cooldownDuration))
            .accountsStrict({
                admin: admin.publicKey,
                vaultConfig: vaultConfig,
                systemProgram: SystemProgram.programId,
                tokenProgram: TOKEN_2022_PROGRAM_ID,
                accessRegistry: accessRegistry,
                usduToken: usdu,
                susduToken: susdu,
            })
            .signers([admin])
            .rpc();

        console.log("Init Vault Config Transaction signature:", tx);
    } catch (error) {
        console.error("Error:", error);
    }
    await new Promise(resolve => setTimeout(resolve, 1000));
}

export async function InitVaultState(
    vaultProgram: Program<Vault>,
    vaultConfig: PublicKey,
    vaultState: PublicKey,
    vaultUsduTokenAccount: PublicKey,
    vaultSusduTokenAccount: PublicKey,
    vaultStakePoolUsduTokenAccount: PublicKey,
    vaultSlioUsduTokenAccount: PublicKey,
    usdu: PublicKey,
    susdu: PublicKey,
    admin: Keypair,
) {

    console.log(`Init Vault State: ${vaultState.toBase58()}`);
    try {
        const tx = await vaultProgram.methods
            .initVaultState()
            .accountsStrict({
                admin: admin.publicKey,
                vaultState: vaultState,
                systemProgram: SystemProgram.programId,
                rent: web3.SYSVAR_RENT_PUBKEY,
            })
            .signers([admin])
            .rpc();

        console.log("Init Vault State Transaction signature:", tx);
    } catch (error) {
        console.error("Error:", error);
    }

    console.log(`Init Vault State Usdu Token Account: ${vaultUsduTokenAccount.toBase58()}`);
    try {
        const tx = await vaultProgram.methods
            .initVaultStateUsduTokenAccount()
            .accountsStrict({
                admin: admin.publicKey,
                vaultState: vaultState,
                vaultConfig: vaultConfig,
                vaultUsduTokenAccount: vaultUsduTokenAccount,
                usduToken: usdu,
                systemProgram: SystemProgram.programId,
                tokenProgram: TOKEN_2022_PROGRAM_ID,
                rent: web3.SYSVAR_RENT_PUBKEY,
            })
            .signers([admin])
            .rpc();

        console.log("Init Vault State Usdu Token Account Transaction signature:", tx);
    } catch (error) {
        console.error("Error:", error);
    }

    console.log(`Init Vault State Susdu Token Account: ${vaultSusduTokenAccount.toBase58()}`);
    try {
        const tx = await vaultProgram.methods
            .initVaultStateSusduTokenAccount()
            .accountsStrict({
                admin: admin.publicKey,
                vaultState: vaultState,
                vaultConfig: vaultConfig,
                vaultSusduTokenAccount: vaultSusduTokenAccount,
                susduToken: susdu,
                systemProgram: SystemProgram.programId,
                tokenProgram: TOKEN_2022_PROGRAM_ID,
                rent: web3.SYSVAR_RENT_PUBKEY,
            })
            .signers([admin])
            .rpc();

        console.log("Init Vault State Susdu Token Account Transaction signature:", tx);
    } catch (error) {
        console.error("Error:", error);
    }

    console.log(`Init Vault State Stake Pool Usdu Token Account: ${vaultStakePoolUsduTokenAccount.toBase58()}`);
    try {
        const tx = await vaultProgram.methods
            .initVaultStateStakePoolUsduTokenAccount()
            .accountsStrict({
                admin: admin.publicKey,
                vaultState: vaultState,
                vaultConfig: vaultConfig,
                vaultStakePoolUsduTokenAccount: vaultStakePoolUsduTokenAccount,
                usduToken: usdu,
                systemProgram: SystemProgram.programId,
                tokenProgram: TOKEN_2022_PROGRAM_ID,
                rent: web3.SYSVAR_RENT_PUBKEY,
            })
            .signers([admin])
            .rpc();

        console.log("Init Vault State Stake Pool Usdu Token Account Transaction signature:", tx);
    } catch (error) {
        console.error("Error:", error);
    }

    console.log(`Init Vault State Slio Usdu Token Account: ${vaultSlioUsduTokenAccount.toBase58()}`);
    try {
        const tx = await vaultProgram.methods
            .initVaultStateSlioUsduTokenAccount()
            .accountsStrict({
                admin: admin.publicKey,
                vaultState: vaultState,
                vaultConfig: vaultConfig,
                vaultSlioUsduTokenAccount: vaultSlioUsduTokenAccount,
                usduToken: usdu,
                systemProgram: SystemProgram.programId,
                tokenProgram: TOKEN_2022_PROGRAM_ID,
                rent: web3.SYSVAR_RENT_PUBKEY,
            })
            .signers([admin])
            .rpc();

        console.log("Init Vault State Slio Usdu Token Account Transaction signature:", tx);
    } catch (error) {
        console.error("Error:", error);
    }
    await new Promise(resolve => setTimeout(resolve, 1000));
}

export async function CreateMintToken(
    connection: Connection,
    payer: Keypair,
    mint: Keypair,
    mintAuthority: PublicKey = payer.publicKey,
    freezeAuthority: PublicKey | null = null,
    decimals: number = 6,
) {
    const mintLen = MINT_SIZE;
    const mintLamports = await connection.getMinimumBalanceForRentExemption(mintLen);

    const createMintAccountIx = SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: mint.publicKey,
        space: mintLen,
        lamports: mintLamports,
        programId: TOKEN_2022_PROGRAM_ID
    });


    const initializeMintIx = createInitializeMintInstruction(
        mint.publicKey,
        decimals,
        mintAuthority,
        freezeAuthority,
        TOKEN_2022_PROGRAM_ID
    );

    const tx = new Transaction()
        .add(createMintAccountIx)
        .add(initializeMintIx);

    const sig = await sendAndConfirmTransaction(
        connection,
        tx,
        [payer, mint],
        {
            skipPreflight: true,
            commitment: 'confirmed'
        }
    );
    console.log("Create mint transaction:", sig);
}

export async function DepositCollateralAndMintUsdu(
    vaultProgram: Program<Vault>,
    usduProgram: Program<Usdu>,
    authority: Keypair,
    vaultConfig: PublicKey,
    usduConfig: PublicKey,
    accessRegistry: PublicKey,
    usduMinter: PublicKey,
    collateralDepositor: PublicKey,
    collateralToken: PublicKey,
    usduToken: PublicKey,
    benefactor: Keypair,
    beneficiary: Keypair,
    fund: Keypair,
    collateralAmount: number,
    usduAmount: number,
    benefactorCollateralTokenAccount: PublicKey,
    beneficiaryUsduTokenAccount: PublicKey,
    fundCollateralTokenAccount: PublicKey,
    vaultCollateralTokenAccount: PublicKey,
) {
    const tx = await vaultProgram.methods
        .depositCollateralMintUsdu(new BN(collateralAmount), new BN(usduAmount))
        .accountsStrict({
            vaultConfig: vaultConfig,
            usduConfig: usduConfig,
            authority: authority.publicKey,
            accessRegistry: accessRegistry,
            usduMinter: usduMinter,
            collateralDepositor: collateralDepositor,
            collateralToken: collateralToken,
            usduToken: usduToken,
            beneficiary: beneficiary.publicKey,
            benefactor: benefactor.publicKey,
            fund: fund.publicKey,
            benefactorCollateralTokenAccount: benefactorCollateralTokenAccount,
            fundCollateralTokenAccount: fundCollateralTokenAccount,
            vaultCollateralTokenAccount: vaultCollateralTokenAccount,
            beneficiaryUsduTokenAccount: beneficiaryUsduTokenAccount,
            usduProgram: usduProgram.programId,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            tokenProgram: TOKEN_2022_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
        })
        .signers([authority])
        .rpc();

    console.log("Deposit and Mint Transaction signature:", tx);
}

export async function RedeemUsduAndWithdrawCollateral(
    vaultProgram: Program<Vault>,
    usduProgram: Program<Usdu>,
    authority: Keypair,
    vaultConfig: PublicKey,
    vaultState: PublicKey,
    usduConfig: PublicKey,
    accessRegistry: PublicKey,
    usduRedeemer: PublicKey,
    collateralWithdrawer: PublicKey,
    collateralToken: PublicKey,
    usduToken: PublicKey,
    benefactor: Keypair,
    beneficiary: Keypair,
    fund: Keypair,
    collateralAmount: number,
    usduAmount: number,
    beneficiaryUsduTokenAccount: PublicKey,
    fundCollateralTokenAccount: PublicKey,
    vaultUsduTokenAccount: PublicKey,
    benefactorCollateralTokenAccount: PublicKey,
) {
    const tx = await vaultProgram.methods
        .redeemUsduWithdrawCollateral(new BN(collateralAmount), new BN(usduAmount))
        .accountsStrict({
            vaultConfig: vaultConfig,
            vaultState: vaultState,
            usduConfig: usduConfig,
            authority: authority.publicKey,
            accessRegistry: accessRegistry,
            usduRedeemer: usduRedeemer,
            collateralWithdrawer: collateralWithdrawer,
            collateralToken: collateralToken,
            usduToken: usduToken,
            benefactor: benefactor.publicKey,
            beneficiary: beneficiary.publicKey,
            fund: fund.publicKey,
            beneficiaryUsduTokenAccount: beneficiaryUsduTokenAccount,
            benefactorCollateralTokenAccount: benefactorCollateralTokenAccount,
            fundCollateralTokenAccount: fundCollateralTokenAccount,
            vaultUsduTokenAccount: vaultUsduTokenAccount,
            usduProgram: usduProgram.programId,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            tokenProgram: TOKEN_2022_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
        })
        .signers([authority])
        .rpc();

    console.log("Redeem Usdu and Withdraw Collateral Transaction signature:", tx);
}

export async function StakeUsduMintSusdu(
    vaultProgram: Program<Vault>,
    susduProgram: Program<Susdu>,
    caller: Keypair,
    receiver: Keypair,
    receiverSusduTokenAccount: PublicKey,
    callerUsduTokenAccount: PublicKey,
    accessRegistry: PublicKey,
    vaultStakePoolUsduTokenAccount: PublicKey,
    susduMinter: PublicKey,
    usduToken: PublicKey,
    susduToken: PublicKey,
    vaultState: PublicKey,
    vaultConfig: PublicKey,
    susduConfig: PublicKey,
    usduAmount: number,
    blacklistState: PublicKey,
) {
    const tx = await vaultProgram.methods
        .stakeUsduMintSusdu(new BN(usduAmount))
        .accountsStrict({
            caller: caller.publicKey,
            receiver: receiver.publicKey,
            receiverSusduTokenAccount: receiverSusduTokenAccount,
            callerUsduTokenAccount: callerUsduTokenAccount,
            accessRegistry: accessRegistry,
            vaultStakePoolUsduTokenAccount: vaultStakePoolUsduTokenAccount,
            susduMinter: susduMinter,
            usduToken: usduToken,
            susduToken: susduToken,
            vaultState: vaultState,
            vaultConfig: vaultConfig,
            susduConfig: susduConfig,
            blacklistState: blacklistState,
            susduProgram: susduProgram.programId,
            tokenProgram: TOKEN_2022_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
        })
        .signers([caller])
        .rpc();

    console.log("Stake Usdu and Mint Susdu Transaction signature:", tx);
}

export async function UnstakeSusdu(
    vaultProgram: Program<Vault>,
    susduProgram: Program<Susdu>,
    caller: Keypair,
    callerSusduTokenAccount: PublicKey,
    receiver: PublicKey,
    receiverUsduTokenAccount: PublicKey,
    susduConfig: PublicKey,
    vaultConfig: PublicKey,
    vaultState: PublicKey,
    vaultSusduTokenAccount: PublicKey,
    cooldown: PublicKey,
    accessRegistry: PublicKey,
    susduRedeemer: PublicKey,
    susduToken: PublicKey,
    usduToken: PublicKey,
    vaultStakePoolUsduTokenAccount: PublicKey,
    vaultSlioUsduTokenAccount: PublicKey,
    susduAmount: number,
    blacklistState: PublicKey,
) {
    const tx = await vaultProgram.methods
        .unstakeSusdu(new BN(susduAmount))
        .accountsStrict({
            caller: caller.publicKey,
            callerSusduTokenAccount: callerSusduTokenAccount,
            receiver: receiver,
            receiverUsduTokenAccount: receiverUsduTokenAccount,
            susduToken: susduToken,
            usduToken: usduToken,
            vaultSusduTokenAccount: vaultSusduTokenAccount,
            vaultStakePoolUsduTokenAccount: vaultStakePoolUsduTokenAccount,
            vaultSlioUsduTokenAccount: vaultSlioUsduTokenAccount,
            vaultConfig: vaultConfig,
            vaultState: vaultState,
            blacklistState: blacklistState,
            accessRegistry: accessRegistry,
            cooldown: cooldown,
            susduConfig: susduConfig,
            susduRedeemer: susduRedeemer,
            susduProgram: susduProgram.programId,
            tokenProgram: TOKEN_2022_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
        })
        .signers([caller])
        .rpc();

    console.log("Unstake Susdu Transaction signature:", tx);
}

export async function WithdrawUsdu(
    vaultProgram: Program<Vault>,
    receiver: Keypair,
    vaultConfig: PublicKey,
    vaultState: PublicKey,
    receiverUsduTokenAccount: PublicKey,
    vaultSlioUsduTokenAccount: PublicKey,
    cooldown: PublicKey,
    usduToken: PublicKey,
    blacklistState: PublicKey,
) {
    const tx = await vaultProgram.methods
        .withdrawUsdu()
        .accountsStrict({
            receiver: receiver.publicKey,
            vaultConfig: vaultConfig,
            vaultState: vaultState,
            receiverUsduTokenAccount: receiverUsduTokenAccount,
            vaultSlioUsduTokenAccount: vaultSlioUsduTokenAccount,
            cooldown: cooldown,
            usduToken: usduToken,
            blacklistState: blacklistState,
            tokenProgram: TOKEN_2022_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
        })
        .signers([receiver])
        .rpc();

    console.log("Withdraw Usdu Transaction signature:", tx);
}

export async function DistributeUsduReward(
    vaultProgram: Program<Vault>,
    caller: Keypair,
    vaultConfig: PublicKey,
    vaultState: PublicKey,
    callerUsduTokenAccount: PublicKey,
    vaultStakePoolUsduTokenAccount: PublicKey,
    accessRegistry: PublicKey,
    distributeRewarder: PublicKey,
    usduToken: PublicKey,
    susduToken: PublicKey,
    susduConfig: PublicKey,
    usduAmount: number,
) {
    const tx = await vaultProgram.methods
        .distributeUsduReward(new BN(usduAmount))
        .accountsStrict({
            caller: caller.publicKey,
            vaultConfig: vaultConfig,
            vaultState: vaultState,
            callerUsduTokenAccount: callerUsduTokenAccount,
            vaultStakePoolUsduTokenAccount: vaultStakePoolUsduTokenAccount,
            accessRegistry: accessRegistry,
            distributeRewarder: distributeRewarder,
            usduToken: usduToken,
            susduToken: susduToken,
            susduConfig: susduConfig,
            tokenProgram: TOKEN_2022_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
        })
        .signers([caller])
        .rpc();

    console.log("Distribute Usdu Reward Transaction signature:", tx);
}

export async function AdjustBlacklist(
    vaultProgram: Program<Vault>,
    authority: Keypair,
    vaultConfig: PublicKey,
    accessRegistry: PublicKey,
    grandMaster: PublicKey,
    isFrozenSusdu: boolean,
    isFrozenUsdu: boolean,
    user: PublicKey,
) {
    const blacklistState = PublicKey.findProgramAddressSync(
        [Buffer.from(vaultBlacklistSeed), user.toBuffer()],
        vaultProgram.programId
    )[0];
    const tx = await vaultProgram.methods
        .adjustBlacklist(user, isFrozenSusdu, isFrozenUsdu)
        .accountsStrict({
            authority: authority.publicKey,
            vaultConfig: vaultConfig,
            accessRegistry: accessRegistry,
            grandMaster: grandMaster,
            blacklistState: blacklistState,
            systemProgram: SystemProgram.programId,
        })
        .signers([authority])
        .rpc();

    console.log("Adjust Blacklist Transaction signature:", tx);
}

export async function RedistributeLockedSusdu(
    vaultProgram: Program<Vault>,
    susduProgram: Program<Susdu>,
    vaultConfig: PublicKey,
    authority: Keypair,
    accessRegistry: PublicKey,
    grandMaster: PublicKey,
    susduRedistributor: PublicKey,
    susduConfig: PublicKey,
    susduToken: PublicKey,
    lockedSusduTokenAccount: PublicKey,
    receiverSusduTokenAccount: PublicKey,
    blacklistState: PublicKey,
    receiver: PublicKey,
) {
    const tx = await vaultProgram.methods
        .redistributeLocked(receiver)
        .accountsStrict({
            authority: authority.publicKey,
            vaultConfig: vaultConfig,
            accessRegistry: accessRegistry,
            grandMaster: grandMaster,
            susduRedistributor: susduRedistributor,
            susduConfig: susduConfig,
            susduToken: susduToken,
            lockedSusduTokenAccount: lockedSusduTokenAccount,
            receiverSusduTokenAccount: receiverSusduTokenAccount,
            blacklistState: blacklistState,
            susduProgram: susduProgram.programId,
            tokenProgram: TOKEN_2022_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
        })
        .signers([authority])
        .rpc();

    console.log("Redistribute Locked Susdu Transaction signature:", tx);
}