import { Connection, Transaction, Keypair, PublicKey, SystemProgram } from "@solana/web3.js"
import {
    createInitializeMintInstruction,
    TOKEN_PROGRAM_ID,
    getMinimumBalanceForRentExemptMint,
    MINT_SIZE,
} from '@solana/spl-token';
import { createInitTreasuryInstruction, PROGRAM_ID } from "../generated";

export const createTreasuryAuthority = async(connection: Connection, wallet: any) => {

  const mintKp = Keypair.generate()

  const tx = new Transaction

  tx.add(
    SystemProgram.createAccount({
      fromPubkey: wallet.publicKey,
      newAccountPubkey: mintKp.publicKey,
      space: MINT_SIZE,
      lamports: await getMinimumBalanceForRentExemptMint(connection),
      programId: TOKEN_PROGRAM_ID,
    }),
    createInitializeMintInstruction(mintKp.publicKey, 0, wallet.publicKey, null, TOKEN_PROGRAM_ID)
  )

  const [treasuryPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("treasury")],
    PROGRAM_ID
  )

  tx.add(
    createInitTreasuryInstruction({
      treasury: treasuryPda,
      rewardMint: mintKp.publicKey,
      payer: wallet.publicKey,
    })
  )

  // Send the transaction
  const blockhash = await connection.getLatestBlockhash()
  tx.recentBlockhash = blockhash.blockhash
  tx.feePayer = wallet.publicKey
  tx.partialSign(mintKp)
  await wallet.signTransaction(tx)

  const sig = await connection.sendRawTransaction(tx.serialize())

  return sig
}