import { NextApiRequest, NextApiResponse } from 'next';
import {
  cryptoWaitReady,
  mnemonicGenerate,
  scryptEncode,
  blake2AsHex,
  keccakAsHex,
  schnorrkelKeypairFromSeed
} from '@polkadot/util-crypto';
import { u8aToU8a } from '@polkadot/util';

export default async function subkey(
  req: NextApiRequest,
  res: NextApiResponse,
) {
  await cryptoWaitReady();

  const mnemonic = mnemonicGenerate();

  res
    .status(200)
    .json({
      message: 'Ok',
      mnemonic,
      scrypt: scryptEncode(mnemonic),
      keccak: keccakAsHex(mnemonic),
      blake2: blake2AsHex(mnemonic),
      keyPair: schnorrkelKeypairFromSeed(
        u8aToU8a(mnemonic)
      )
    });
}
