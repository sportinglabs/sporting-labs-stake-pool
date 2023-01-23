/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as web3 from '@solana/web3.js'
import * as beet from '@metaplex-foundation/beet'
import * as beetSolana from '@metaplex-foundation/beet-solana'

/**
 * Arguments used to create {@link StakeEntry}
 * @category Accounts
 * @category generated
 */
export type StakeEntryArgs = {
  bump: number
  pool: web3.PublicKey
  amount: beet.bignum
  originalMint: web3.PublicKey
  originalMintClaimed: boolean
  lastStaker: web3.PublicKey
}

export const stakeEntryDiscriminator = [187, 127, 9, 35, 155, 68, 86, 40]
/**
 * Holds the data for the {@link StakeEntry} Account and provides de/serialization
 * functionality for that data
 *
 * @category Accounts
 * @category generated
 */
export class StakeEntry implements StakeEntryArgs {
  private constructor(
    readonly bump: number,
    readonly pool: web3.PublicKey,
    readonly amount: beet.bignum,
    readonly originalMint: web3.PublicKey,
    readonly originalMintClaimed: boolean,
    readonly lastStaker: web3.PublicKey
  ) {}

  /**
   * Creates a {@link StakeEntry} instance from the provided args.
   */
  static fromArgs(args: StakeEntryArgs) {
    return new StakeEntry(
      args.bump,
      args.pool,
      args.amount,
      args.originalMint,
      args.originalMintClaimed,
      args.lastStaker
    )
  }

  /**
   * Deserializes the {@link StakeEntry} from the data of the provided {@link web3.AccountInfo}.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static fromAccountInfo(
    accountInfo: web3.AccountInfo<Buffer>,
    offset = 0
  ): [StakeEntry, number] {
    return StakeEntry.deserialize(accountInfo.data, offset)
  }

  /**
   * Retrieves the account info from the provided address and deserializes
   * the {@link StakeEntry} from its data.
   *
   * @throws Error if no account info is found at the address or if deserialization fails
   */
  static async fromAccountAddress(
    connection: web3.Connection,
    address: web3.PublicKey,
    commitmentOrConfig?: web3.Commitment | web3.GetAccountInfoConfig
  ): Promise<StakeEntry> {
    const accountInfo = await connection.getAccountInfo(
      address,
      commitmentOrConfig
    )
    if (accountInfo == null) {
      throw new Error(`Unable to find StakeEntry account at ${address}`)
    }
    return StakeEntry.fromAccountInfo(accountInfo, 0)[0]
  }

  /**
   * Provides a {@link web3.Connection.getProgramAccounts} config builder,
   * to fetch accounts matching filters that can be specified via that builder.
   *
   * @param programId - the program that owns the accounts we are filtering
   */
  static gpaBuilder(
    programId: web3.PublicKey = new web3.PublicKey(
      '654kE3ccD76txX3nrP8Q2FTxjD82qk6XrcoJZYZ1cess'
    )
  ) {
    return beetSolana.GpaBuilder.fromStruct(programId, stakeEntryBeet)
  }

  /**
   * Deserializes the {@link StakeEntry} from the provided data Buffer.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static deserialize(buf: Buffer, offset = 0): [StakeEntry, number] {
    return stakeEntryBeet.deserialize(buf, offset)
  }

  /**
   * Serializes the {@link StakeEntry} into a Buffer.
   * @returns a tuple of the created Buffer and the offset up to which the buffer was written to store it.
   */
  serialize(): [Buffer, number] {
    return stakeEntryBeet.serialize({
      accountDiscriminator: stakeEntryDiscriminator,
      ...this,
    })
  }

  /**
   * Returns the byteSize of a {@link Buffer} holding the serialized data of
   * {@link StakeEntry}
   */
  static get byteSize() {
    return stakeEntryBeet.byteSize
  }

  /**
   * Fetches the minimum balance needed to exempt an account holding
   * {@link StakeEntry} data from rent
   *
   * @param connection used to retrieve the rent exemption information
   */
  static async getMinimumBalanceForRentExemption(
    connection: web3.Connection,
    commitment?: web3.Commitment
  ): Promise<number> {
    return connection.getMinimumBalanceForRentExemption(
      StakeEntry.byteSize,
      commitment
    )
  }

  /**
   * Determines if the provided {@link Buffer} has the correct byte size to
   * hold {@link StakeEntry} data.
   */
  static hasCorrectByteSize(buf: Buffer, offset = 0) {
    return buf.byteLength - offset === StakeEntry.byteSize
  }

  /**
   * Returns a readable version of {@link StakeEntry} properties
   * and can be used to convert to JSON and/or logging
   */
  pretty() {
    return {
      bump: this.bump,
      pool: this.pool.toBase58(),
      amount: (() => {
        const x = <{ toNumber: () => number }>this.amount
        if (typeof x.toNumber === 'function') {
          try {
            return x.toNumber()
          } catch (_) {
            return x
          }
        }
        return x
      })(),
      originalMint: this.originalMint.toBase58(),
      originalMintClaimed: this.originalMintClaimed,
      lastStaker: this.lastStaker.toBase58(),
    }
  }
}

/**
 * @category Accounts
 * @category generated
 */
export const stakeEntryBeet = new beet.BeetStruct<
  StakeEntry,
  StakeEntryArgs & {
    accountDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['accountDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['bump', beet.u8],
    ['pool', beetSolana.publicKey],
    ['amount', beet.u64],
    ['originalMint', beetSolana.publicKey],
    ['originalMintClaimed', beet.bool],
    ['lastStaker', beetSolana.publicKey],
  ],
  StakeEntry.fromArgs,
  'StakeEntry'
)