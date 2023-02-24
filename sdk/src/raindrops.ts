import { State, Instructions, Utils, PlayerProgram, ItemProgram } from "@raindrops-protocol/raindrops";
import { PublicKey } from "@solana/web3.js";
import BN from "bn.js";

export const testRaindrops = async (wallet) => {

  // const itemProgram = await ItemProgram.getProgramWithWallet(
  //   ItemProgram,
  //   wallet,
  //   "devnet",
  //   "https://rpc-devnet.helius.xyz/?api-key=5f68e534-f666-4159-965b-e87485e72d55"
  // );
  // const item = await itemProgram.client.account.item.fetch(player.equippedItems[0].item);
  // console.log(item);

  const playerProgram = await PlayerProgram.getProgramWithWallet(
    PlayerProgram,
    wallet,
    "devnet",
    "https://rpc-devnet.helius.xyz/?api-key=5f68e534-f666-4159-965b-e87485e72d55"
  );

  const [playerPda] = await Utils.PDA.getPlayerPDA(new PublicKey("7qXAghPweYNBdZ6d6JNLRev8UbzwzDGz1nDxAJ9Sasd6"), new BN(22));

  const player = await playerProgram.client.account.player.fetch(playerPda);
  console.log(player);  
  
}