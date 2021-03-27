import { NextApiRequest, NextApiResponse } from 'next';
import {
  cryptoWaitReady,
  mnemonicGenerate,
  scryptEncode,
  blake2AsHex,
  keccakAsHex,
  schnorrkelKeypairFromSeed,
  signatureVerify
} from '@polkadot/util-crypto';
import { u8aToU8a } from '@polkadot/util';
import { Keyring } from '@polkadot/keyring';

export default async function subkey(
  req: NextApiRequest,
  res: NextApiResponse,
) {
  await cryptoWaitReady();

  const mnemonic = mnemonicGenerate();

  const keyring = new Keyring({
     type: 'sr25519'
  });

  keyring.addFromMnemonic(mnemonic)

  const keyPair = keyring.getPairs()[0];
  const payload = 'message';
  const signature =  keyPair.sign(u8aToU8a(payload), { withType: true });

  res
    .status(200)
    .json({
      message: 'Ok',
      mnemonic,
      // scrypt: scryptEncode(mnemonic),
      // keccak: keccakAsHex(mnemonic),
      // blake2: blake2AsHex(mnemonic),
      // keyPair,
      signature,
      payload,
      isVerified: signatureVerify(`${payload}`, signature, keyPair.publicKey).isValid
    });
}