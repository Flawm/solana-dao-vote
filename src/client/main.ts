import {
  Account,
  Keypair,
  Connection,
  PublicKey,
  LAMPORTS_PER_SOL,
  SystemProgram,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import {
  MintLayout,
  AccountLayout,
  Token,
  TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
import fs from 'mz/fs';
import path from 'path';
import * as borsh from 'borsh';

import { getPayer, getRpcUrl, createKeypairFromFile } from './utils';

let connection: Connection;

let payer: Keypair;

let meta_program = new PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s');

/**
 * Path to program files
 */
const PROGRAM_PATH = path.resolve(__dirname, '../../dist/program');

const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, 'dao_voter.so');

const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, 'dao_voter.json');

export async function establishConnection(): Promise<void> {
  const rpcUrl = await getRpcUrl();
  connection = new Connection(rpcUrl, 'confirmed');
  const version = await connection.getVersion();
  console.log('Connection to cluster established:', rpcUrl, version);
}

export async function establishPayer(): Promise<void> {
  payer = await getPayer();
}

const program_id = new PublicKey(
  'Daovoteq2Y28gJyme6TNUXT9TxXrePiouFuHezkiozci'
);

export async function castVote(): Promise<void> {
  const vote = +process.argv[3];

  let creator_address = new PublicKey(process.argv[4]),
    mint_key = new PublicKey(process.argv[2]),
    token_key = (await connection.getTokenLargestAccounts(mint_key)).value[0]
      .address,
    meta_key = (
      await PublicKey.findProgramAddress(
        [
          new Uint8Array([109, 101, 116, 97, 100, 97, 116, 97]),
          meta_program.toBuffer(),
          mint_key.toBuffer(),
        ],
        meta_program
      )
    )[0],
    auth_key = (
      await PublicKey.findProgramAddress(
        [mint_key.toBuffer(), toU64Le(vote)],
        program_id
      )
    )[0],
    vote_auth_key = (
      await PublicKey.findProgramAddress(
        [creator_address.toBuffer(), toU64Le(vote)],
        program_id
      )
    )[0],
    sys_key = new PublicKey('11111111111111111111111111111111');

  // accounts
  let account_0 = {
      pubkey: payer.publicKey,
      isSigner: false,
      isWritable: true,
    },
    account_1 = { pubkey: mint_key, isSigner: false, isWritable: false },
    account_2 = { pubkey: token_key, isSigner: false, isWritable: false },
    account_3 = { pubkey: meta_key, isSigner: false, isWritable: false },
    account_4 = { pubkey: auth_key, isSigner: false, isWritable: true },
    account_5 = { pubkey: sys_key, isSigner: false, isWritable: false },
    account_6 = { pubkey: vote_auth_key, isSigner: false, isWritable: false };

  const instruction = new TransactionInstruction({
    keys: [
      account_0,
      account_1,
      account_2,
      account_3,
      account_4,
      account_5,
      account_6,
    ],
    programId: program_id,
    data: Buffer.from(
      new Uint8Array(
        [1].concat(Array.from(toU64Le(vote))).concat(Array.from(toU64Le(1)))
      )
    ),
  });

  let transaction = new Transaction().add(instruction);

  transaction.recentBlockhash = (
    await connection.getRecentBlockhash()
  ).blockhash;
  transaction.feePayer = payer.publicKey;

  let a = await sendAndConfirmTransaction(connection, transaction, [payer], {
    skipPreflight: true,
  });

  console.log(a);
}

export async function testGPA() {
  let creator = process.argv[2],
    gpa = await connection.getProgramAccounts(program_id, {
      filters: [{ memcmp: { bytes: creator, offset: 32 } }, { dataSize: 116 }],
    });

  for (let i = 0, l = gpa.length; i < l; i++) {
    let data = gpa[i].account.data;
    let mint = new PublicKey(data.slice(0, 32)),
      creator = new PublicKey(data.slice(32, 64)),
      voter = new PublicKey(data.slice(64, 96)),
      vote = data.slice(96, 104),
      time =
        data[104] + (data[105] << 8) + (data[106] << 16) + (data[107] << 24),
      vote_option = data.slice(108);

    let vote_info_key = (
      await PublicKey.findProgramAddress([creator.toBuffer(), vote], program_id)
    )[0];

    let vote_info = await connection.getAccountInfo(vote_info_key);
    let url = vote_info!.data!.slice(0, 100);
    let test = String.fromCharCode(...Array.from(url).filter((e) => e > 0));

    console.log([
      mint.toBase58(),
      creator.toBase58(),
      voter.toBase58(),
      vote,
      vote_option,
      time,
      test,
    ]);
  }
}

const toU64Le = (n: number) => {
  const a = [];
  a.unshift(n & 255);

  while (n >= 256) {
    n = n >>> 8;
    a.unshift(n & 255);
  }

  //backfill 0
  for (let i = a.length; i < 8; i++) {
    a.push(0);
  }

  return new Uint8Array(a);
};

export async function createVote(): Promise<void> {
  const vote = +process.argv[3];

  let creator_key = new PublicKey(process.argv[4]),
    mint_key = new PublicKey(process.argv[2]),
    token_key = (await connection.getTokenLargestAccounts(mint_key)).value[0]
      .address,
    meta_key = (
      await PublicKey.findProgramAddress(
        [
          new Uint8Array([109, 101, 116, 97, 100, 97, 116, 97]),
          meta_program.toBuffer(),
          mint_key.toBuffer(),
        ],
        meta_program
      )
    )[0],
    auth_key = (
      await PublicKey.findProgramAddress(
        [creator_key.toBuffer(), toU64Le(vote)],
        program_id
      )
    )[0],
    sys_key = new PublicKey('11111111111111111111111111111111');

  // accounts
  let account_0 = {
      pubkey: payer.publicKey,
      isSigner: false,
      isWritable: true,
    },
    account_1 = { pubkey: mint_key, isSigner: false, isWritable: false },
    account_2 = { pubkey: token_key, isSigner: false, isWritable: false },
    account_3 = { pubkey: meta_key, isSigner: false, isWritable: false },
    account_4 = { pubkey: auth_key, isSigner: false, isWritable: true },
    account_5 = { pubkey: sys_key, isSigner: false, isWritable: false };

  const instruction = new TransactionInstruction({
    keys: [account_0, account_1, account_2, account_3, account_4, account_5],
    programId: program_id,
    data: Buffer.from(
      new Uint8Array(
        [2]
          .concat(Array.from(toU64Le(vote)))
          .concat(Array.from(Buffer.from(process.argv[5])))
      )
    ),
  });

  let transaction = new Transaction().add(instruction);

  transaction.recentBlockhash = (
    await connection.getRecentBlockhash()
  ).blockhash;
  transaction.feePayer = payer.publicKey;

  let a = await sendAndConfirmTransaction(connection, transaction, [payer], {
    skipPreflight: true,
  });

  console.log(a);
}
