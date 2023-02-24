import { Connection, PublicKey, Transaction } from "@solana/web3.js"
import { BN } from "@project-serum/anchor"
import { createClaimReceiptMintInstruction, createCloseStakeEntryInstruction, createInitEntryInstruction, createReturnReceiptMintInstruction, createStakeInstruction, createUnstakeInstruction, PROGRAM_ID, StakePool, Treasury } from "../generated";
import { getAssociatedTokenAddressSync, createAssociatedTokenAccountInstruction, getAccount } from "@solana/spl-token";
import { findTokenManagerAddress, findMintCounterId } from "@cardinal/token-manager/dist/cjs/programs/tokenManager/pda";
import { getRemainingAccountsForKind, getRemainingAccountsForInvalidate, TOKEN_MANAGER_ADDRESS } from "@cardinal/token-manager/dist/cjs/programs/tokenManager";
import { ASSOCIATED_PROGRAM_ID } from "@project-serum/anchor/dist/cjs/utils/token";
import { tryFn } from "./utils";
import { Utils } from "@raindrops-protocol/raindrops";

export const stake = async(connection: Connection, wallet: any, originalMint: PublicKey) => {

  // const [treasuryPda] = PublicKey.findProgramAddressSync(
  //   [Buffer.from("treasury")],
  //   PROGRAM_ID
  // )

  // const treasury = await Treasury.fromAccountAddress(connection, treasuryPda)

  const [stakePoolPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("stake-pool"), new BN(1).toArrayLike(Buffer, "le", 8)],
    PROGRAM_ID
  );

  const [stakeEntryPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("stake-entry"), stakePoolPda.toBuffer(), originalMint.toBuffer(), wallet.publicKey.toBuffer()],
    PROGRAM_ID
  );

  const [originalMintMetadata] = PublicKey.findProgramAddressSync(
    [Buffer.from("metadata"), new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").toBuffer(), originalMint.toBuffer()],
    new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
  );

  // raindrops
  const [playerPda] = await Utils.PDA.getPlayerPDA(originalMint, new BN(22));

  const tx = new Transaction()

  tx.add(
    createInitEntryInstruction({
      stakeEntry: stakeEntryPda,
      stakePool: stakePoolPda,
      originalMint: originalMint,
      player: playerPda,
      originalMintMetadata: originalMintMetadata,
      payer: wallet.publicKey,
      }, {
        user: wallet.publicKey,
      })
  )

  // Stake

  const stakeEntryOriginalMintAddress = getAssociatedTokenAddressSync(originalMint, stakeEntryPda, true);
  const stakeEntryOriginalMintTokenAccount = await tryFn(() => connection.getAccountInfo(stakeEntryOriginalMintAddress))
  if (!stakeEntryOriginalMintTokenAccount) { tx.add(createAssociatedTokenAccountInstruction(wallet.publicKey, stakeEntryOriginalMintAddress, stakeEntryPda, originalMint)) }

  const userOriginalMintTokenAddress = getAssociatedTokenAddressSync(originalMint, wallet.publicKey);
  const userOriginalMintTokenAccount = await tryFn(() => connection.getAccountInfo(userOriginalMintTokenAddress))
  if (!userOriginalMintTokenAccount) { tx.add(createAssociatedTokenAccountInstruction(wallet.publicKey, userOriginalMintTokenAddress, wallet.publicKey, originalMint)) }

  tx.add(
    createStakeInstruction({
      stakeEntry: stakeEntryPda,
      stakePool: stakePoolPda,
      stakeEntryOriginalMintTokenAccount: stakeEntryOriginalMintAddress,
      originalMint: originalMint,
      user: wallet.publicKey,
      userOriginalMintTokenAccount: userOriginalMintTokenAddress,
      }, {
        amount: new BN(1),
      })
  )

  // Claim receipt mint

  const tokenManagerAddress = findTokenManagerAddress(originalMint);
  const mintCounter = findMintCounterId(originalMint)

  const tokenManagerReceiptMintAddress = getAssociatedTokenAddressSync(originalMint, tokenManagerAddress, true);
  const tokenManagerReceiptMintTokenAccount = await tryFn(() => connection.getAccountInfo(tokenManagerReceiptMintAddress))
  if (!tokenManagerReceiptMintTokenAccount) { tx.add(createAssociatedTokenAccountInstruction(wallet.publicKey, tokenManagerReceiptMintAddress, tokenManagerAddress, originalMint)) }

  const remainingAccounts = getRemainingAccountsForKind(originalMint, 3);

  tx.add(createClaimReceiptMintInstruction({
    stakeEntry: stakeEntryPda,
    originalMint: originalMint,
    receiptMint: originalMint,
    stakeEntryReceiptMintTokenAccount: stakeEntryOriginalMintAddress,
    user: wallet.publicKey,
    userReceiptMintTokenAccount: userOriginalMintTokenAddress,
    tokenManagerReceiptMintTokenAccount: tokenManagerReceiptMintAddress,
    tokenManager: tokenManagerAddress,
    mintCounter: mintCounter,
    tokenManagerProgram: TOKEN_MANAGER_ADDRESS,
    associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
    anchorRemainingAccounts: remainingAccounts,
  }))
  
  // Send the transaction
  const blockhash = await connection.getLatestBlockhash()
  tx.recentBlockhash = blockhash.blockhash
  tx.feePayer = wallet.publicKey
  await wallet.signTransaction(tx)

  const sig = await connection.sendRawTransaction(tx.serialize())

  return sig
}

export const unstake = async(connection: Connection, wallet: any, originalMint: PublicKey) => {
  const [treasuryPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("treasury")],
    PROGRAM_ID
  )  

  const [stakePoolPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("stake-pool"), new BN(1).toArrayLike(Buffer, "le", 8)],
    PROGRAM_ID
  );

  const [stakeEntryPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("stake-entry"), stakePoolPda.toBuffer(), originalMint.toBuffer(), wallet.publicKey.toBuffer()],
    PROGRAM_ID
  );

  // Raindrops
  // const [playerPda] = await getPlayerPDA(originalMint, classIndex)

  const tx = new Transaction();

  // Return receipt mint
  const tokenManagerAddress = findTokenManagerAddress(originalMint);
  const tokenManagerReceiptMintAddress = getAssociatedTokenAddressSync(originalMint, tokenManagerAddress, true);

  const userOriginalMintTokenAddress = getAssociatedTokenAddressSync(originalMint, wallet.publicKey);
  const userOriginalMintTokenAccount = await tryFn(() => connection.getAccountInfo(userOriginalMintTokenAddress))
  if (!userOriginalMintTokenAccount) { tx.add(createAssociatedTokenAccountInstruction(wallet.publicKey, userOriginalMintTokenAddress, wallet.publicKey, originalMint)) }

  const remainingAccounts = await getRemainingAccountsForInvalidate(connection, originalMint)

  tx.add(
    createReturnReceiptMintInstruction({
      stakeEntry: stakeEntryPda,
      receiptMint: originalMint,
      tokenManager: tokenManagerAddress,
      tokenManagerTokenAccount: tokenManagerReceiptMintAddress,
      userReceiptMintTokenAccount: userOriginalMintTokenAddress,
      user: wallet.publicKey,
      collector: wallet.publicKey,
      tokenManagerProgram: TOKEN_MANAGER_ADDRESS,
      anchorRemainingAccounts: remainingAccounts,
    })
  )

  // Unstake
  const stakeEntryOriginalMintAddress = getAssociatedTokenAddressSync(originalMint, stakeEntryPda, true);

  const stakePoolAcc = await StakePool.fromAccountAddress(connection, stakePoolPda);  

  const userRewardMintAddress = getAssociatedTokenAddressSync(stakePoolAcc.rewardMint, wallet.publicKey);  
  const userRewardMintTokenAccount = await tryFn(() => connection.getAccountInfo(userRewardMintAddress));  
  if (!userRewardMintTokenAccount) { tx.add(createAssociatedTokenAccountInstruction(wallet.publicKey, userRewardMintAddress, wallet.publicKey, stakePoolAcc.rewardMint)) }

  // raindrops
  const [playerPda] = await Utils.PDA.getPlayerPDA(originalMint, new BN(22))

  tx.add(
    createUnstakeInstruction({
      stakePool: stakePoolPda,
      treasury: treasuryPda,
      stakeEntry: stakeEntryPda,
      originalMint: originalMint,
      player: playerPda,
      stakeEntryOriginalMintTokenAccount: stakeEntryOriginalMintAddress,
      userRewardMintTokenAccount: userRewardMintAddress,
      rewardMint: stakePoolAcc.rewardMint,
      user: wallet.publicKey,
      userOriginalMintTokenAccount: userOriginalMintTokenAddress
    })
  )

  tx.add(
    createCloseStakeEntryInstruction({
      stakeEntry: stakeEntryPda,
      stakePool: stakePoolPda,
      authority: stakePoolAcc.authority,
    })
  )

   // Send the transaction
  const blockhash = await connection.getLatestBlockhash()
  tx.recentBlockhash = blockhash.blockhash
  tx.feePayer = wallet.publicKey
  await wallet.signTransaction(tx)

  const sig = await connection.sendRawTransaction(tx.serialize(), { skipPreflight: true })

  return sig
}