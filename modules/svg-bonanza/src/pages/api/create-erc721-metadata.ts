import * as path from 'path';
import * as fs from 'fs/promises';
import { NextApiRequest, NextApiResponse } from 'next';
import { NFTStorage, Blob } from 'nft.storage';

export default async function createERC721Metadata(
  req: NextApiRequest,
  res: NextApiResponse
) {
  const { reward, score, scoreOutOf, sessionId } = (req.body ||
    {}) as Partial<GenerateSvgMarkupProps>;

  const isInputValid = [reward, score, scoreOutOf].every(
    x => typeof x === 'number' && x > 0
  ) && typeof sessionId === 'string' && sessionId.length > 0;

  if (isInputValid === false) {
    res.status(400).json({ message: 'Invalid input '});

    return;
  }

  const svgMarkup = await generateSvgMarkup({
    reward,
    score,
    scoreOutOf,
    sessionId,
  });

  const { NFT_STORAGE_TOKEN } = process.env;
  const client = new NFTStorage({ token: NFT_STORAGE_TOKEN });

  const nftCid = await client.storeBlob(
    new Blob([svgMarkup]),
  );
  const metadata = createMetadata(nftCid);

  res.status(200).json({ metadata });
}

interface GenerateSvgMarkupProps {
  reward: number;
  score: number;
  scoreOutOf: number;
  sessionId: string;
}

async function generateSvgMarkup({
  reward,
  score,
  scoreOutOf,
  sessionId,
}): Promise<string> {
  const rewardFormatted = Intl.NumberFormat('en-US', {
    currency: 'USD',
    style: 'currency',
  }).format(reward);

  const svgTemplateFileBuffer = await fs.readFile(
    path.resolve('src', 'assets', 'nft-template.svg')
  );

  const svgTemplate = svgTemplateFileBuffer.toString('utf8');
  const svgMarkup = svgTemplate
    .replace('{reward}', `${rewardFormatted}`)
    .replace('{score}', score)
    .replace('{scoreOutOf}', scoreOutOf)
    .replace('{signature}', sessionId);

  return svgMarkup;
}

function createMetadata(cid: string) {
  return {
    title: 'Assets Metadata',
    type: 'object',
    properties: {
      name: {
        type: 'string',
        description: 'Jack Block Winner!',
      },
      description: {
        type: 'string',
        description: 'A unique NTF minted in unique conditions for a winner of the Jack Block lottery',
      },
      image: {
        type: 'string',
        description: `https://ipfs.io/ipfs/${cid}`,
      },
    },
  };
}
