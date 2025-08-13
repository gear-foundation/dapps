import  { expect } from "chai";
import * as chai from "chai";
import chaiAsPromised from "chai-as-promised";
import { ethers } from "ethers";
import { describe, it } from "mocha";
// @ts-ignore
import * as circom_tester from "circom_tester";
// @ts-ignore
//import * as ff from "ffjavascript";

import { buildBls12381, F1Field } from "ffjavascript";
import { randomBytes } from 'crypto';



function batchElGamalVerifyNoAlpha(
    original: bigint[][],     
    encrypted: bigint [][],    
    rScalars: bigint[],      
    base: {X: bigint, Y: bigint, Z: bigint},
    pk: {X: bigint, Y: bigint, Z: bigint},
): boolean {
    const n = rScalars.length;

    let sumC0 = { X: 0n, Y: 0n, Z: 1n };
    let sumC1 = { X: 0n, Y: 0n, Z: 1n };
    let sumIC0 = { X: 0n, Y: 0n, Z: 1n };
    let sumIC1 = { X: 0n, Y: 0n, Z: 1n };
    let rSum: bigint = 0n;

    for (let i = 0; i < n; i++) {
        const ic0 = {
            X: original[0][i],
            Y: original[1][i],
            Z: original[2][i],
          };
          const ic1 = {
            X: original[3][i],
            Y: original[4][i],
            Z: original[5][i],
          };
      
          const c0 = {
            X: encrypted[0][i],
            Y: encrypted[1][i],
            Z: encrypted[2][i],
          };
          const c1 = {
            X: encrypted[3][i],
            Y: encrypted[4][i],
            Z: encrypted[5][i],
          };
        const r = rScalars[i];

        sumC0 = projectiveAdd(F, a, d, sumC0, c0);
        sumC1 = projectiveAdd(F, a, d,sumC1, c1);
        sumIC0 = projectiveAdd(F, a, d,sumIC0, ic0);
        sumIC1 = projectiveAdd(F, a, d,sumIC1, ic1);
        rSum += r;
    }

     const leftC0 = sumC0;
     const rightC0 = projectiveAdd(F, a, d, scalarMul(F, a, d, base, rSum), sumIC0);
     const leftC1 = sumC1;
     const rightC1 = projectiveAdd(F, a, d, scalarMul(F, a, d, pk, rSum), sumIC1);

    return isEqualProjective(leftC0, rightC0) && isEqualProjective(leftC1, rightC1);
}

function isEqualProjective(p1: {X: bigint, Y: bigint, Z: bigint}, p2: {X: bigint, Y: bigint, Z: bigint}): boolean {
    return (
      p1.X * p2.Z === p2.X * p1.Z &&
      p1.Y * p2.Z === p2.Y * p1.Z
    );
  }

const q = BigInt("52435875175126190479447740508185965837690552500527637822603658699938581184513"); // BLS12-381 scalar field
const F = new F1Field(q);
const neutral = { X: 0n, Y: 1n, Z: 1n };
const base = {
    X: BigInt("0x29c132cc2c0b34c5743711777bbe42f32b79c022ad998465e1e71866a252ae18"),
    Y: BigInt("0x2a6c669eda123e0f157d8b50badcd586358cad81eee464605e3167b6cc974166"),
    Z: 1n,
  };

const a = BigInt(-5);
const d = 45022363124591815672509500913686876175488063829319466900776701791074614335719n;
chai.use(chaiAsPromised);

function generateRandomScalar(numBits: number): bigint {
    const byteLength = Math.ceil(numBits / 8);
    const max = 1n << BigInt(numBits);

    while (true) {
        const buf = randomBytes(byteLength);
        let sk = BigInt("0x" + buf.toString("hex"));
        if (sk < max) return sk;
    }
}


function initializeDeck(numCards: number, F: any, a: bigint, d: bigint): { X: bigint, Y: bigint, Z: bigint }[][] {
    const deck: { X: bigint, Y: bigint, Z: bigint }[][] = [];
  
    for (let i = 1; i <= numCards; i++) {
      const ic0 = { ...neutral };
      const ic1 = scalarMul(F, a, d, base, BigInt(i));
      deck.push([ic0, ic1]);
    }
  
    return deck;
  }

  function permuteMatrix(matrix: bigint[][], permutation: number[]): bigint[][] {
    const permuted = Array.from({ length: 6 }, () => Array(52));
    for (let row = 0; row < 6; row++) {
      for (let col = 0; col < 52; col++) {
        permuted[row][col] = matrix[row][permutation[col]];
      }
    }
    return permuted;
  }

function generatePermutation(n: number): number[] {
    const arr = Array.from({ length: n }, (_, i) => i);
    for (let i = n - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [arr[i], arr[j]] = [arr[j], arr[i]];
    }
    return arr;
  }

  function elgamalEncryptDeck(
    F: any,
    a: bigint,
    d: bigint,
    G: any,
    pk: any,
    deck: bigint[][],
  ): { encrypted: bigint[][]; rScalars: bigint[] } {
    const encrypted = Array.from({ length: 6 }, () => Array(52).fill(0n));
    const rScalars: bigint[] = [];
  
    for (let i = 0; i < 52; i++) {
      const ic0 = {
        X: deck[0][i],
        Y: deck[1][i],
        Z: deck[2][i],
      };
      const ic1 = {
        X: deck[3][i],
        Y: deck[4][i],
        Z: deck[5][i],
      };
  
      const { c0, c1, r } = elgamalEncrypt(F, a, d, G, pk, { ic0, ic1 });
      rScalars.push(r);
  
      encrypted[0][i] = c0.X;
      encrypted[1][i] = c0.Y;
      encrypted[2][i] = c0.Z;
      encrypted[3][i] = c1.X;
      encrypted[4][i] = c1.Y;
      encrypted[5][i] = c1.Z;
    }
  
    return { encrypted, rScalars };
  }
  
  
function elgamalEncrypt(F: any, a: bigint, d: bigint, G: any, pk: any, msg: any) {
    const r = generateRandomScalar(50);
    const rG = scalarMul(F, a, d, G, r);
    const rPK = scalarMul(F, a, d, pk, r);
    const c0 = projectiveAdd(F, a, d, rG, { ...msg.ic0 });
    const c1 = projectiveAdd(F, a, d, rPK, { ...msg.ic1 });
    return { c0, c1, r };
}

function toAffine(F: any, P: { X: bigint, Y: bigint, Z: bigint }) {
    const x = F.div(P.X, P.Z);
    const y = F.div(P.Y, P.Z);
    return { x, y };
  }
  function expectAffineEqual(F: any, p1: { x: bigint; y: bigint }, p2: { x: bigint; y: bigint }) {
    expect(F.eq(p1.x, p2.x)).to.equal(true);
    expect(F.eq(p1.y, p2.y)).to.equal(true);
  }

  function initDeck(numCards: number): bigint[][] {
    const deck: bigint[][] = Array.from({ length: 6 }, () => Array(numCards).fill(0n));

    for (let i = 0; i < numCards; i++) {
        const scalar = BigInt(i + 1);
        deck[0][i] = 0n;
        deck[1][i] = 1n;
        deck[2][i] = 1n;
        const P = scalarMul(F, a, d, base, scalar);
        deck[3][i] = P.X;
        deck[4][i] = P.Y;
        deck[5][i] = P.Z;
    }

    return deck;
}
  describe("ShuffleEncrypt", function () {
    let circuit: any;
    this.timeout(100_000); 
    before(async () => {
            circuit = await circom_tester.wasm("./shuffle_encrypt/shuffle_encrypt.circom", {prime: "bls12381"});
        })
    it("should accept correct permutation and encryption", async () => {
        const deck = initDeck(52);
        const sk = generateRandomScalar(64);
        const pk = scalarMul(F, a, d, base, sk);
        const { encrypted, rScalars } = elgamalEncryptDeck(F, a, d, base, pk, deck);
        const permutation = generatePermutation(52);
        const shuffled = permuteMatrix(encrypted, permutation);
        
        const input = {
            pk: [pk.X.toString(), pk.Y.toString(), pk.Z.toString()],
            R:  rScalars.map(r => r.toString()),
            original: deck.map((row) => row.map((v) => v.toString())),
            permuted: shuffled.map((row) => row.map((v) => v.toString())),
          };
      
          const witness = await circuit.calculateWitness(input, true);

          expect(witness[1]).to.equal(1n);
    });
});

  describe("Permutation Circuit", function () {
    let circuit: any;
    before(async () => {
        circuit = await circom_tester.wasm("./test/test_is_permutation.circom", {prime: "bls12381"});
    })
    it("valid permutation: identity", async () => {
        const input = [18, 31, 21, 50, 9, 36, 7, 16, 11, 0, 27, 25, 14, 1, 34, 5, 24, 10, 8, 51, 39, 23, 33, 2, 13, 6, 37, 28, 29, 43, 32, 48, 20, 17, 38, 49, 44, 41, 22, 19, 30, 4, 26, 45, 47, 3, 42, 35, 15, 40, 46, 12];
        const witness = await circuit.calculateWitness({ in: input }, true);
        expect(witness[1]).to.equal(1n);
      });
    it("invalid permutation: duplicate", async () => {
        const input =[18, 18, 21, 50, 9, 36, 7, 16, 11, 0, 27, 25, 14, 1, 34, 5, 24, 10, 8, 51, 39, 23, 33, 2, 13, 6, 37, 28, 29, 43, 32, 48, 20, 17, 38, 49, 44, 41, 22, 19, 30, 4, 26, 45, 47, 3, 42, 35, 15, 40, 46, 12];
        const witness = await circuit.calculateWitness({ in: input }, true);
        expect(witness[1]).to.equal(0n);
    });

    it("invalid permutation: out_of_bounds", async () => {
        const input = [18, 31, 21, 50, 9, 52, 7, 16, 11, 0, 27, 25, 14, 1, 34, 5, 24, 10, 8, 51, 39, 23, 33, 2, 13, 6, 37, 28, 29, 43, 32, 48, 20, 17, 38, 49, 44, 41, 22, 19, 30, 4, 26, 45, 47, 3, 42, 35, 15, 40, 46, 12];
        const witness = await circuit.calculateWitness({ in: input }, true);
        expect(witness[1]).to.equal(0n);
    });
  })
describe("ElGamal Encrypt and Decrypt Circuits", function () {
    let encCircuit: any;
    let decCircuit: any;
    let G: any;
    before(async () => {
        encCircuit = await circom_tester.wasm("./test/test_elgamal_encrypt.circom", {prime: "bls12381"});
        decCircuit = await circom_tester.wasm("./test/test_elgamal_decrypt.circom", {prime: "bls12381"});
        G = {
            X: BigInt("0x29c132cc2c0b34c5743711777bbe42f32b79c022ad998465e1e71866a252ae18"),
            Y: BigInt("0x2a6c669eda123e0f157d8b50badcd586358cad81eee464605e3167b6cc974166"),
            Z: 1n
          };
    });

    it("should encrypt a point correctly", async () => {
        // Generate test data
        const sk = generateRandomScalar(64);
        const pk = scalarMul(F, a, d, G, sk);
       
        // --- ElGamal encryption --- //
        const msg = {
            ic0: { X: 0n, Y: 1n, Z: 1n },
            ic1: scalarMul(F, a, d, G, generateRandomScalar(64))
          };
        const { c0, c1, r } = elgamalEncrypt(F, a, d, G, pk, msg);

        // Prepare circuit inputs
        const input = {
            r: r.toString(),
            pk: [pk.X.toString(), pk.Y.toString(), pk.Z.toString()],
            ic0: [msg.ic0.X.toString(), msg.ic0.Y.toString(), msg.ic0.Z.toString()],
            ic1: [msg.ic1.X.toString(), msg.ic1.Y.toString(), msg.ic1.Z.toString()]
          };

        const witness = await encCircuit.calculateWitness(input, true);

        // Fetch outputs
        const c0out = {
            X:  witness[1],
            Y:  witness[2],
            Z:  witness[3],
        }

        const c1out = {
            X:  witness[4],
            Y:  witness[5],
            Z:  witness[6],
        }

        const c0Affine = toAffine(F, c0);
        const c1Affine = toAffine(F, c1);
        const c0outAffine = toAffine(F, c0out);
        const c1outAffine = toAffine(F, c1out);

        expectAffineEqual(F, c0Affine, c0outAffine);
        expectAffineEqual(F, c1Affine, c1outAffine);
    });

    it("Elgamal decrypt: decrypt(c1 - sk*c0) == message", async () => {
    
        const sk = generateRandomScalar(64);
        const pk = scalarMul(F, a, d, G, sk);
    
        const msg = {
            ic0: { X: 0n, Y: 1n, Z: 1n },
            ic1: scalarMul(F, a, d, G, generateRandomScalar(64))
          };
        const { c0, c1, r } = elgamalEncrypt(F, a, d, G, pk, msg);
        const skC0 = scalarMul(F, a, d, c0, sk);
        const skC0Neg = {
            X: F.neg(skC0.X),
            Y: skC0.Y,
            Z: skC0.Z,
        }

        const witness = await decCircuit.calculateWitness({
          c0: [c0.X.toString(), c0.Y.toString(), c0.Z.toString()],
          sk: sk.toString(),
          expected: [skC0Neg.X.toString(), skC0Neg.Y.toString(), skC0Neg.Z.toString()]
        }, true);
        console.log(witness[1])
        // с1 - sk*c0
        const decrypted = projectiveAdd(F, a, d, c1, skC0Neg);
 
        expectAffineEqual(F, toAffine(F, decrypted), toAffine(F, msg.ic1));
      });
    //   it("should allow N signers to sequentially encrypt and independently decrypt", async () => {
    //     const N = 5;
    //     const secretKeys = Array.from({ length: N }, () => generateRandomScalar(128));
    //     const publicKeys = secretKeys.map(sk => scalarMul(F, a, d, G, sk));

    //    // Aggregate public key
    //     const pkAgg = publicKeys.reduce((acc, pk) => projectiveAdd(F, a, d, acc, pk), { X: 0n, Y: 1n, Z: 1n });

    //     const msg = scalarMul(F, a, d, G, generateRandomScalar(128));

    //    // Sequential encryption
    //     let state = { ic0: { X: 0n, Y: 1n, Z: 1n }, ic1: msg };
    //     for (let i = 0; i < N; i++) {
    //         let step = elgamalEncrypt(F, a, d, G, pkAgg, state);
    //         state = { ic0: step.c0, ic1: step.c1}
    //     }

    //     const c0 = state.ic0;
    //     const c1 = state.ic1;

    //     // Sequential decryption (aggregate negation of sk_i * c0)
    //     let negatedSum = { X: 0n, Y: 1n, Z: 1n };
    //     for (const sk of secretKeys) {
    //         const witness = await decCircuit.calculateWitness({
    //             c0: [c0.X.toString(), c0.Y.toString(), c0.Z.toString()],
    //             sk: sk.toString()
    //         }, true);

    //         const negated = {
    //             X: BigInt(witness[1]),
    //             Y: BigInt(witness[2]),
    //             Z: BigInt(witness[3])
    //         };

    //         negatedSum = projectiveAdd(F, a, d, negatedSum, negated);
    //     }

    //     const decrypted = projectiveAdd(F, a, d, c1, negatedSum);
    //     const mRecovered = toAffine(F, decrypted);
    //     const mExpected = toAffine(F, msg);

    //     expect(F.eq(mRecovered.x, mExpected.x)).to.equal(true);
    //     expect(F.eq(mRecovered.y, mExpected.y)).to.equal(true);


    //   });
//     it("should fail for an invalid point", async () => {
//         const circuit = await circom_tester.wasm(CIRCUIT_PATH);
//         const invalidPk = {
//             x: "1234567890123456789012345678901234567890",
//             y: "9876543210987654321098765432109876543210"
//         };
//         const r = generateRandomScalar(251);
//         const ic0 = [F.random(), F.random()];
//         const ic1 = [F.random(), F.random()];

//         const input = {
//             ic0: ic0.map(x => babyjub.F.toObject(x)),
//             ic1: ic1.map(x => babyjub.F.toObject(x)),
//             r: BigInt(r),
//             pk: invalidPk
//         };
//         await expect(circuit.calculateWitness(input, true)).to.be.rejectedWith(Error, /Not enough values/);
//     });
 });

 function scalarMul(
    F: F1Field,
    a: bigint,
    d: bigint,
    P: { X: bigint, Y: bigint, Z: bigint },
    n: bigint
  ): { X: bigint, Y: bigint, Z: bigint } {
    if (n === 0n) {
      return { X: 0n, Y: 1n, Z: 1n }; // Нейтральный элемент
    }
    let R = { X: 0n, Y: 1n, Z: 1n }; // Нейтральный аккумулятор
    let Q = { ...P }; // Копия P для итеративного удвоения
  
    while (n > 0n) {
      if (n & 1n) {
        R = projectiveAdd(F, a, d, R, Q);
      }
      Q = projectiveAdd(F, a, d, Q, Q);
      n >>= 1n;
    }
  
    return R;
  }

  function projectiveAdd(
    F: any,
    a: bigint,
    d: bigint,
    P1: { X: bigint, Y: bigint, Z: bigint },
    P2: { X: bigint, Y: bigint, Z: bigint }
  ): { X: bigint, Y: bigint, Z: bigint } {
    const { X: X1, Y: Y1, Z: Z1 } = P1;
    const { X: X2, Y: Y2, Z: Z2 } = P2;
  
    const A = F.mul(Z1, Z2);                // A = Z1 * Z2
    const B = F.square(A);                  // B = A^2
    const C = F.mul(X1, X2);                // C = X1 * X2
    const D = F.mul(Y1, Y2);                // D = Y1 * Y2
    const E = F.mul(F.mul(d, C), D);        // E = d * C * D
    const F_ = F.sub(B, E);                 // F = B - E
    const G = F.add(B, E);                  // G = B + E
  
    const X1plusY1 = F.add(X1, Y1);
    const X2plusY2 = F.add(X2, Y2);
    const X1Y1_X2Y2 = F.mul(X1plusY1, X2plusY2); // (X1 + Y1)(X2 + Y2)
    const CD = F.add(C, D);                        // C + D
    const E_ = F.sub(X1Y1_X2Y2, CD);               // E = (X1 + Y1)(X2 + Y2) - (C + D)
  
    const X3 = F.mul(F.mul(A, F_), E_);     // X3 = A * F * E
    const Y3 = F.mul(F.mul(A, G), F.sub(D, F.mul(a, C))); // Y3 = A * G * (D - a * C)
    const Z3 = F.mul(F_, G);                // Z3 = F * G
  
    return { X: X3, Y: Y3, Z: Z3 };
  }
  

  function isOnCurve(x: bigint, y: bigint): boolean {
    try {
        // Проверка нейтрального элемента (0, 1)
        if (x === 0n && y === 1n) return true;

        // Вычисление x² и y² в поле

        x = F.normalize(x);
        y = F.normalize(y);

        const x2 = F.square(x);
        const y2 = F.square(y);

        // Левая часть уравнения: -5x² + y²
        const lhs = F.add(
            F.mul(F.e(a), x2),
            y2
        );

        // Правая часть уравнения: 1 + d*x²*y²
        const x2y2 = F.mul(x2, y2);
        const rhs = F.add(
            F.e(1n),
            F.mul(F.e(d), x2y2)
        );

        let lhs_norm = F.normalize(lhs)
        let rhs_norm = F.normalize(rhs)
        return F.eq(lhs_norm, rhs_norm);
        
    } catch (e) {
        return false;
    }
}

function randomFieldElement(): bigint {
    const buffer = randomBytes(32);
    const value = BigInt("0x" + buffer.toString("hex")) % q;
    return value;
  }

function generatePoint(
    F: ReturnType<typeof F1Field>,
    a: bigint,
    d: bigint
  ): { x: bigint; y: bigint } {
  
    while (true) {
      const x = F.e(randomFieldElement()); 
      const x2 = F.mul(x, x);
  
      const denom = F.sub(F.e(1n), F.mul(F.e(d), x2));
      if (F.isZero(denom)) continue;
  
      const numer = F.add(F.mul(F.neg(F.e(a)), x2), F.e(1n));
  
      const y2 = F.div(numer, denom);
  
      const y = F.sqrt(y2);
      if (y !== null) {
        return { 
          x: F.toObject(x), 
          y: F.toObject(y) 
        };
      }
    }
  }