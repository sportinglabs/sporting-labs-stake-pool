/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'
import { InitPoolIx, initPoolIxBeet } from '../types/InitPoolIx'

/**
 * @category Instructions
 * @category InitPool
 * @category generated
 */
export type InitPoolInstructionArgs = {
  ix: InitPoolIx
}
/**
 * @category Instructions
 * @category InitPool
 * @category generated
 */
export const initPoolStruct = new beet.FixableBeetArgsStruct<
  InitPoolInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['ix', initPoolIxBeet],
  ],
  'InitPoolInstructionArgs'
)
/**
 * Accounts required by the _initPool_ instruction
 *
 * @property [_writable_] stakePool
 * @property [_writable_] treasury
 * @property [_writable_, **signer**] payer
 * @category Instructions
 * @category InitPool
 * @category generated
 */
export type InitPoolInstructionAccounts = {
  stakePool: web3.PublicKey
  treasury: web3.PublicKey
  payer: web3.PublicKey
  systemProgram?: web3.PublicKey
  anchorRemainingAccounts?: web3.AccountMeta[]
}

export const initPoolInstructionDiscriminator = [
  116, 233, 199, 204, 115, 159, 171, 36,
]

/**
 * Creates a _InitPool_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category InitPool
 * @category generated
 */
export function createInitPoolInstruction(
  accounts: InitPoolInstructionAccounts,
  args: InitPoolInstructionArgs,
  programId = new web3.PublicKey('41MZASop6YTB5UmYNSDFxFJ4QYEMeDY9f7WcABLUmfoB')
) {
  const [data] = initPoolStruct.serialize({
    instructionDiscriminator: initPoolInstructionDiscriminator,
    ...args,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.stakePool,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.treasury,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.payer,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
  ]

  if (accounts.anchorRemainingAccounts != null) {
    for (const acc of accounts.anchorRemainingAccounts) {
      keys.push(acc)
    }
  }

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  })
  return ix
}
