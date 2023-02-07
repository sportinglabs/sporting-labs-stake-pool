import { Connection, PublicKey } from "@solana/web3.js";
import { PROGRAM_ID, Treasury, StakePool } from "../generated";
import { BN } from "@project-serum/anchor";

export const tryFn = async (fn: () => Promise<any>) => {
  try {
    return await fn();
  } catch (e) {
    return e;
  }
}

export const getAllPools = async (connection: Connection) => {
  const [treasuryPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("treasury")],
    PROGRAM_ID
  )

  const treasury = await Treasury.fromAccountAddress(connection, treasuryPda)

  const pools = []

  for (let i = 1; i < treasury.poolCount; i++) {    
    const [stakePoolPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("stake-pool"), new BN(i).toArrayLike(Buffer, "le", 8)],
      PROGRAM_ID
    );

    const pool = await StakePool.fromAccountAddress(connection, stakePoolPda)

    pools.push(pool)
  }

  return pools
}