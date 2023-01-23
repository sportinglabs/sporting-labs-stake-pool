require("dotenv").config();
import { Wallet, BN } from "@project-serum/anchor";
import { Connection, Keypair } from "@solana/web3.js"
import { readFileSync } from "fs";
import { createTreasuryAuthority } from "../sdk";

describe("sporting-labs-contract", () => {
  // Configure the client to use the local cluster.
  const connection = new Connection("https://solana-devnet.g.alchemy.com/v2/T9bjWGVG22puNBe91XFY_hE4lhR7hkgk")

  const kp = Keypair.fromSecretKey(Uint8Array.from(
    JSON.parse(readFileSync(process.env.KEYPAIR, { encoding: "utf-8"}))
  ))

  const wallet = new Wallet(kp);

  it("It creates treasury!", async () => {
    // Add your test here.
    const res = await createTreasuryAuthority(connection, wallet);
    console.log(res);
    
  });

});
