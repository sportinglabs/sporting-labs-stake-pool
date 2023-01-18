import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SportingLabsContract } from "../target/types/sporting_labs_contract";

describe("sporting-labs-contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SportingLabsContract as Program<SportingLabsContract>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
