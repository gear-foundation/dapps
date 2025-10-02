import { F1Field } from 'ffjavascript';

const q = BigInt('52435875175126190479447740508185965837690552500527637822603658699938581184513');

// params for jubjub curve (BLS12-381 scalar field)
const curveParams = {
  q,
  a: BigInt(-5),
  d: 45022363124591815672509500913686876175488063829319466900776701791074614335719n,
  base: {
    X: BigInt('0x29c132cc2c0b34c5743711777bbe42f32b79c022ad998465e1e71866a252ae18'),
    Y: BigInt('0x2a6c669eda123e0f157d8b50badcd586358cad81eee464605e3167b6cc974166'),
    Z: 1n,
  },
  scalarField: BigInt('13108968793781547619861935127046491459309155893440570251786403306729687672801'), // BLS12-381 scalar field
  F: new F1Field(q),
  FQ_BYTES: 32,
  FR_BYTES: 32,
};

const SUITS = ['Hearts', 'Diamonds', 'Clubs', 'Spades'];
const RANKS = ['2', '3', '4', '5', '6', '7', '8', '9', '10', 'J', 'Q', 'K', 'A'];

export { curveParams, SUITS, RANKS };
