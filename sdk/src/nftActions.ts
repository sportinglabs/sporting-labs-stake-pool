import { Connection, PublicKey, Transaction } from "@solana/web3.js"
import { BN } from "@project-serum/anchor"
import { createInitPoolInstruction, PROGRAM_ID, Treasury } from "../generated";

export const stake = async(connection: Connection, wallet: any) => {



  const tx = new Transaction()

  // Send the transaction
  const blockhash = await connection.getLatestBlockhash()
  tx.recentBlockhash = blockhash.blockhash
  tx.feePayer = wallet.publicKey
  await wallet.signTransaction(tx)

  const sig = await connection.sendRawTransaction(tx.serialize(), { skipPreflight: true })

  return sig
}