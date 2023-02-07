import { Connection, PublicKey, Transaction } from "@solana/web3.js"
import { BN } from "@project-serum/anchor"
import { createInitPoolInstruction, createUpdatePoolInstruction, PROGRAM_ID, Treasury } from "../generated";

export const createPool = async(connection: Connection, wallet: any) => {

  const [treasuryPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("treasury")],
    PROGRAM_ID
  )

  const treasury = await Treasury.fromAccountAddress(connection, treasuryPda)

  const [stakePoolPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("stake-pool"), new BN(treasury.poolCount).toArrayLike(Buffer, "le", 8)],
    PROGRAM_ID
  );

  const tx = new Transaction();

  tx.add(
    createInitPoolInstruction({
      stakePool: stakePoolPda,
      treasury: treasuryPda,
      payer: wallet.publicKey
    }, { ix: {
      requiresCreators: [],
      authority: wallet.publicKey,
    }})
  )

  // Send the transaction
  const blockhash = await connection.getLatestBlockhash()
  tx.recentBlockhash = blockhash.blockhash
  tx.feePayer = wallet.publicKey
  await wallet.signTransaction(tx)

  const sig = await connection.sendRawTransaction(tx.serialize())

  return sig
}  

export type UpdateInstructions = {
  requiresCreators: PublicKey[]
  authority: PublicKey
  poolState: number
}

export const updatePool = async(connection: Connection, wallet: any, identifier: number, updateInstructions: UpdateInstructions) => {
  
  const [stakePoolPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("stake-pool"), new BN(identifier).toArrayLike(Buffer, "le", 8)],
    PROGRAM_ID
  );

  const tx = new Transaction();

  tx.add(
    createUpdatePoolInstruction({
      stakePool: stakePoolPda,
      payer: wallet.publicKey
    }, { ix: updateInstructions
    })
  )

  // Send the transaction
  const blockhash = await connection.getLatestBlockhash()
  tx.recentBlockhash = blockhash.blockhash
  tx.feePayer = wallet.publicKey
  await wallet.signTransaction(tx)

  const sig = await connection.sendRawTransaction(tx.serialize())

  return sig
}