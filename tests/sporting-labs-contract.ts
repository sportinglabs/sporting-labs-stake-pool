require("dotenv").config();
import { Wallet, BN } from "@project-serum/anchor";
import { Connection, Keypair, PublicKey } from "@solana/web3.js"
import { readFileSync } from "fs";
import { createTreasuryAuthority, getAllPools, createPool, updatePool, stake, unstake } from "../sdk";

describe("sporting-labs-contract", () => {
  // Configure the client to use the local cluster.
  const connection = new Connection("https://solana-devnet.g.alchemy.com/v2/T9bjWGVG22puNBe91XFY_hE4lhR7hkgk")

  const kp = Keypair.fromSecretKey(Uint8Array.from(
    JSON.parse(readFileSync(process.env.KEYPAIR, { encoding: "utf-8"}))
  ))

  const wallet = new Wallet(kp);

  it("It works", async () => {
    // // Create a treasury
    // const res = await createTreasuryAuthority(connection, wallet);
    // console.log(res);

    // // Create a pool
    // const res = await createPool(connection, wallet);
    // console.log(res);

    // // stake NFT
    const mint = new PublicKey("9zorzrRpz9LqS8JZCveMcu3EyF5EciZ4MPdWTdeeSuQP")

    // const res = await stake(connection, wallet, mint)
    // console.log(res);
    
    // // unstake NFT
    // const res = await unstake(connection, wallet, mint)
    // console.log(res);

    // const res = await updatePool(connection, wallet, 1, {
    //   requiresCreators: [],
    //   authority: wallet.publicKey,
    //   poolState: 1
    // })

    // console.log(res);

    const allPools = await getAllPools(connection);
    console.log(allPools);

  });

});
