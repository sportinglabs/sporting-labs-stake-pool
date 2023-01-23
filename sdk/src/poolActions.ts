import { Connection, PublicKey, Transaction } from "@solana/web3.js"
import { BN } from "@project-serum/anchor"
import { createInitPoolInstruction, PROGRAM_ID, Treasury } from "../generated";
import * as sbv2 from "@switchboard-xyz/solana.js";

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

  const vrf = new PublicKey("")

  const tx = new Transaction();

  tx.add(
    createInitPoolInstruction({
      stakePool: stakePoolPda,
      treasury: treasuryPda,
      vrf: vrf,
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

  const sig = await connection.sendRawTransaction(tx.serialize(), { skipPreflight: true })

  return sig
}