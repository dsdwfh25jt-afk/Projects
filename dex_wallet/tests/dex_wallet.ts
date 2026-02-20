import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DexWallet } from "../target/types/dex_wallet";
import {
  createMint,
  createAccount,
  mintTo,
  getAccount,
  getMint,
} from "@solana/spl-token";
import { expect } from "chai";

describe("dex_wallet", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.DexWallet as Program<DexWallet>;

  let tokenMintA: anchor.web3.PublicKey;
  let tokenMintB: anchor.web3.PublicKey;
  let userTokenAAccount: anchor.web3.PublicKey;
  let userTokenBAccount: anchor.web3.PublicKey;
  let userLpAccount: anchor.web3.PublicKey;

  const [poolPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("pool")],
    program.programId
  );

  before(async () => {
    tokenMintA = await createMint(
      provider.connection,
      provider.wallet.payer,
      provider.wallet.publicKey,
      null,
      6
    );

    tokenMintB = await createMint(
      provider.connection,
      provider.wallet.payer,
      provider.wallet.publicKey,
      null,
      6
    );

    userTokenAAccount = await createAccount(
      provider.connection,
      provider.wallet.payer,
      tokenMintA,
      provider.wallet.publicKey
    );

    userTokenBAccount = await createAccount(
      provider.connection,
      provider.wallet.payer,
      tokenMintB,
      provider.wallet.publicKey
    );

    await mintTo(
      provider.connection,
      provider.wallet.payer,
      tokenMintA,
      userTokenAAccount,
      provider.wallet.payer,
      1_000_000 * 10 ** 6
    );

    await mintTo(
      provider.connection,
      provider.wallet.payer,
      tokenMintB,
      userTokenBAccount,
      provider.wallet.payer,
      1_000_000 * 10 ** 6
    );
  });

  it("Initializes the pool", async () => {
    const [vaultAPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault_a")],
      program.programId
    );

    const [vaultBPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault_b")],
      program.programId
    );

    const [lpMintPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("lp_mint")],
      program.programId
    );

    await program.methods
      .initializePool(new anchor.BN(30)) // 0.3% fee
      .accounts({
        authority: provider.wallet.publicKey,
        pool: poolPda,
        tokenMintA,
        tokenMintB,
        vaultA: vaultAPda,
        vaultB: vaultBPda,
        lpMint: lpMintPda,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .rpc();

    const poolAccount = await program.account.pool.fetch(poolPda);
    expect(poolAccount.feeBps.toNumber()).to.equal(30);
    expect(poolAccount.tokenMintA.toString()).to.equal(tokenMintA.toString());
    expect(poolAccount.tokenMintB.toString()).to.equal(tokenMintB.toString());

    const [lpMintPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("lp_mint")],
      program.programId
    );
    userLpAccount = await createAccount(
      provider.connection,
      provider.wallet.payer,
      lpMintPda,
      provider.wallet.publicKey
    );
  });

  it("Adds liquidity", async () => {
    const [vaultAPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault_a")],
      program.programId
    );

    const [vaultBPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault_b")],
      program.programId
    );

    const [lpMintPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("lp_mint")],
      program.programId
    );

    const amountA = new anchor.BN(1000 * 10 ** 6);
    const amountB = new anchor.BN(2000 * 10 ** 6);

    await program.methods
      .addLiquidity(amountA, amountB)
      .accounts({
        user: provider.wallet.publicKey,
        pool: poolPda,
        userTokenA: userTokenAAccount,
        userTokenB: userTokenBAccount,
        vaultA: vaultAPda,
        vaultB: vaultBPda,
        lpMint: lpMintPda,
        userLpAccount,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .rpc();

    const lpAccount = await getAccount(provider.connection, userLpAccount);
    expect(Number(lpAccount.amount)).to.be.greaterThan(0);
  });

  it("Swaps token A for B", async () => {
    const [vaultAPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault_a")],
      program.programId
    );

    const [vaultBPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault_b")],
      program.programId
    );

    const amountIn = new anchor.BN(100 * 10 ** 6);
    const minAmountOut = new anchor.BN(1); // Accept any amount for test

    const balanceBefore = await getAccount(
      provider.connection,
      userTokenBAccount
    );

    await program.methods
      .swapAForB(amountIn, minAmountOut)
      .accounts({
        user: provider.wallet.publicKey,
        pool: poolPda,
        userTokenIn: userTokenAAccount,
        userTokenOut: userTokenBAccount,
        vaultA: vaultAPda,
        vaultB: vaultBPda,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .rpc();

    const balanceAfter = await getAccount(
      provider.connection,
      userTokenBAccount
    );
    expect(Number(balanceAfter.amount)).to.be.greaterThan(
      Number(balanceBefore.amount)
    );
  });
});
