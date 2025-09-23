import { bls12_381 } from '@noble/curves/bls12-381';
import  { expect } from "chai";
import * as chai from "chai";
import chaiAsPromised from "chai-as-promised";
import { ethers } from "ethers";
import { describe, it } from "mocha";
// @ts-ignore
import * as circom_tester from "circom_tester";
// @ts-ignore
import { F1Field } from "ffjavascript";
import { randomBytes } from 'crypto';


const q = BigInt("52435875175126190479447740508185965837690552500527637822603658699938581184513"); // BLS12-381 scalar field
const F = new F1Field(q);

// Параметры кривой Bandersnatch
const a = BigInt(-5);
const d = 45022363124591815672509500913686876175488063829319466900776701791074614335719n;

const CIRCUIT_PATH = "./common/bandersnatch.circom";

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
  
  function bandersnatchAdd(
    F: ReturnType<typeof F1Field>,
    a: bigint,
    d: bigint,
    X1: bigint, Y1: bigint, Z1: bigint,
    X2: bigint, Y2: bigint, Z2: bigint
  ): [bigint, bigint, bigint] {
    const A = F.mul(X1, X2);
    const B = F.mul(Y1, Y2);
    const C = F.mul(Z1, Z2);
    const D = F.mul(F.e(d), F.mul(A, B));
  
    const E = F.sub(F.mul(F.add(X1, Y1), F.add(X2, Y2)), F.add(A, B));
    const C2 = F.square(C);
    const F_ = F.sub(C2, D);
    const G = F.add(C2, D);
  
    const aA = F.mul(F.e(a), A);
    const BmaA = F.sub(B, aA);
  
    const X3 = F.mul(E, F_);
    const Y3 = F.mul(BmaA, G);
    const Z3 = F.mul(F_, F.mul(G, C));
  
    return [X3, Y3, Z3];
  }
  
describe("BLS12-381 Point Check", function () {
    this.timeout(10000);
  
    it("Should accept valid public key on BLS12-381", async () => {
        const circuit = await circom_tester.wasm(
            CIRCUIT_PATH,
            {prime: "bls12381"}
        );
  
        const point = generatePoint(F, a, d);
      
        const input = {
            x: F.toObject(point.x).toString(), // Используйте F.toObject()
            y: F.toObject(point.y).toString()
        };

        const witness = await circuit.calculateWitness(input, true);

         await circuit.checkConstraints(witness);
    });

})

describe("Bandersnatch Addition", () => {
    let circuit: any;
  
    before(async () => {
      circuit = await circom_tester.wasm("./test/test_bandersnatch_add.circom", {
        prime: "bls12381",
      });
    });
  
    it("Should correctly add two random points", async () => {
        const P1 = generatePoint(F, a, d);
        const P2 = generatePoint(F, a, d);

        const input = {
        X1: P1.x.toString(),
        Y1: P1.y.toString(),
        Z1: "1",
        X2: P2.x.toString(),
        Y2: P2.y.toString(),
        Z2: "1"
        };

        const expected = projectiveAdd(F, a, d, P1.x, P1.y, 1n, P2.x, P2.y, 1n);
        const witness = await circuit.calculateWitness(input, true);

        expect(witness[1].toString()).to.equal(expected.X.toString());
        expect(witness[2].toString()).to.equal(expected.Y.toString());
        expect(witness[3].toString()).to.equal(expected.Z.toString());

    });
  
    it("P + 0 = P (neutral addition)", async () => {
        const P = generatePoint(F, a, d);
    
        const input = {
          X1: P.x.toString(),
          Y1: P.y.toString(),
          Z1: "1",
          X2: "0",
          Y2: "1",
          Z2: "1"
        };
    
        const witness = await circuit.calculateWitness(input, true);
        const X3 = BigInt(witness[1]);
        const Y3 = BigInt(witness[2]);
        const Z3 = BigInt(witness[3]);
    
        const x = F.div(X3, Z3);
        const y = F.div(Y3, Z3);
    
        expect(x).to.equal(P.x);
        expect(y).to.equal(P.y);
      });

      it("P + P = 2P (doubling)", async () => {
        const P = generatePoint(F, a, d);
    
        const input = {
          X1: P.x.toString(),
          Y1: P.y.toString(),
          Z1: "1",
          X2: P.x.toString(),
          Y2: P.y.toString(),
          Z2: "1"
        };
    
        const expected = projectiveAdd(F, a, d, P.x, P.y, 1n, P.x, P.y, 1n);
        const witness = await circuit.calculateWitness(input, true);
    
        expect(witness[1].toString()).to.equal(expected.X.toString());
        expect(witness[2].toString()).to.equal(expected.Y.toString());
        expect(witness[3].toString()).to.equal(expected.Z.toString());
      });

      it("P + (-P) = 0 (cancellation to neutral point)", async () => {
        const P = generatePoint(F, a, d);
      
        const X1 = P.x;
        const Y1 = P.y;
        const Z1 = 1n;
      
        const X2 = F.neg(X1);
        const Y2 = Y1;
        const Z2 = 1n;
      
        const input = {
          X1: X1.toString(),
          Y1: Y1.toString(),
          Z1: Z1.toString(),
          X2: X2.toString(),
          Y2: Y2.toString(),
          Z2: Z2.toString(),
        };
      
        const witness = await circuit.calculateWitness(input, true);
        const X3 = BigInt(witness[1]);
        const Y3 = BigInt(witness[2]);
        const Z3 = BigInt(witness[3]);
      
        // Normalize to affine coordinates
        const Zinv = F.inv(Z3);
        const x = F.mul(X3, Zinv);
        const y = F.mul(Y3, Zinv);
      
        expect(x).to.equal(0n);     // x = 0
        expect(y).to.equal(1n);     // y = 1  → affine neutral point
      });
      
    })
  
    function projectiveAdd(F: any, a: bigint, d: bigint, X1: bigint, Y1: bigint, Z1: bigint, X2: bigint, Y2: bigint, Z2: bigint) {
        const A = F.mul(Z1, Z2);                 // A = Z1 * Z2
        const B = F.square(A);                   // B = A^2
        const C = F.mul(X1, X2);                 // C = X1 * X2
        const D = F.mul(Y1, Y2);                 // D = Y1 * Y2
        const E = F.mul(F.mul(d, C), D);         // E = d * C * D
        const F_ = F.sub(B, E);                  // F = B - E
        const G = F.add(B, E);                   // G = B + E
        const X = F.mul(A, F_);                  // A * F
        const X3 = F.mul(X, F.sub(
            F.mul(F.add(X1, Y1), F.add(X2, Y2)),
            F.add(C, D)
        ));
        const Y3 = F.mul(F.mul(A, G), F.sub(D, F.mul(a, C)));
        const Z3 = F.mul(F_, G);
        return { X: X3, Y: Y3, Z: Z3 };
    }
    
  
  describe("BandersnatchDoubleProjective", function () {
    this.timeout(10000);
  
    let doubleCircuit: any;
    let addCircuit: any;
  
    before(async () => {
        doubleCircuit = await circom_tester.wasm("./test/test_bandersnatch_double.circom", {
            prime: "bls12381",
          });
    });
  
    it("Double(P) == Add(P, P)", async () => {
        const P = generatePoint(F, a, d);
        const { x, y } = P;
    
        const expected = projectiveAdd(F, a, d, x, y, 1n, x, y, 1n);
    
        const input = {
          X: x.toString(),
          Y: y.toString(),
          Z: "1"
        };
    
        const witness = await doubleCircuit.calculateWitness(input, true);
        await doubleCircuit.checkConstraints(witness);

        const X3 = BigInt(witness[1]);
        const Y3 = BigInt(witness[2]);
        const Z3 = BigInt(witness[3]);

        expect(F.mul(X3, expected.Z)).to.equal(F.mul(expected.X, Z3));
        expect(F.mul(Y3, expected.Z)).to.equal(F.mul(expected.Y, Z3));

      });

      it("should compute add(double(P), double(P)) == P added 4 times", async () => {    
        const initialP = generatePoint(F, a, d);
        let acc = { X: 0n, Y: 1n, Z: 1n };

        // double(P)
        let witness = await doubleCircuit.calculateWitness({
            X: initialP.x.toString(),
            Y: initialP.y.toString(),
            Z: "1"
        }, true);
    
        const Xd = BigInt(witness[1]);
        const Yd = BigInt(witness[2]);
        const Zd = BigInt(witness[3]);

        for (let i = 0; i < 2; i++) {
            acc = projectiveAdd(F, a, d, acc.X, acc.Y, acc.Z, initialP.x, initialP.y, 1n);
        }


        let X3 = BigInt(witness[1]);
        let Y3 = BigInt(witness[2]);
        let Z3 = BigInt(witness[3]);

        expect(F.mul(X3, acc.Z)).to.equal(F.mul(acc.X, Z3));
        expect(F.mul(Y3, acc.Z)).to.equal(F.mul(acc.Y, Z3));

        // second double
        const doubleP = { x: X3, y: Y3}
        witness = await doubleCircuit.calculateWitness({
            X: doubleP.x.toString(),
            Y: doubleP.y.toString(),
            Z: Z3
        }, true);

        acc = { X: 0n, Y: 1n, Z: 1n };
        for (let i = 0; i < 2; i++) {
            acc = projectiveAdd(F, a, d, acc.X, acc.Y, acc.Z, doubleP.x, doubleP.y, Z3);
        }
        X3 = BigInt(witness[1]);
        Y3 = BigInt(witness[2]);
        Z3 = BigInt(witness[3]);

        expect(F.mul(X3, acc.Z)).to.equal(F.mul(acc.X, Z3));
        expect(F.mul(Y3, acc.Z)).to.equal(F.mul(acc.Y, Z3));

        // add 4 times initial P
        acc = { X: 0n, Y: 1n, Z: 1n };
        for (let i = 0; i < 4; i++) {
            acc = projectiveAdd(F, a, d, acc.X, acc.Y, acc.Z, initialP.x, initialP.y, 1n);
        }

        expect(F.mul(X3, acc.Z)).to.equal(F.mul(acc.X, Z3));
        expect(F.mul(Y3, acc.Z)).to.equal(F.mul(acc.Y, Z3));
    });
    
    it("Double(0) == 0 (neutral element)", async () => {
        const neutral = {
          X: "0",
          Y: "1",
          Z: "1"
        };
      
        const witness = await doubleCircuit.calculateWitness(neutral, true);
        await doubleCircuit.checkConstraints(witness);
      
        const X3 = BigInt(witness[1]);
        const Y3 = BigInt(witness[2]);
        const Z3 = BigInt(witness[3]);
      
        // Проверяем, что результат тоже (0, 1, 1)
        expect(X3).to.equal(0n);
        expect(Y3).to.equal(1n);
        expect(Z3).to.equal(1n);
      });
    
  });
  describe("BandersnatchScalarMulProjective", function () {
    this.timeout(15000);
  
    it("scalar mul by 2 equals P + P", async () => {
        const circuit = await circom_tester.wasm("test/test_bandersnatch_scalar_mul.circom", {
            prime: "bls12381",
          });
    
        const P = generatePoint(F, a, d);
    
        const expected = projectiveAdd(F, a, d, P.x, P.y, 1n, P.x, P.y, 1n);
    
        const input = {
          X: P.x.toString(),
          Y: P.y.toString(),
          Z: "1",
          scalar: "2"
        };
    
        const witness = await circuit.calculateWitness(input, true);

        const X3 = BigInt(witness[1]);
        const Y3 = BigInt(witness[2]);
        const Z3 = BigInt(witness[3]);
    
        expect(F.mul(X3, expected.Z)).to.equal(F.mul(expected.X, Z3));
        expect(F.mul(Y3, expected.Z)).to.equal(F.mul(expected.Y, Z3));
        expect(F.mul(Z3, expected.Z)).to.equal(F.mul(expected.Z, Z3));
        const Xaff = F.div(X3, Z3);
        const Yaff = F.div(Y3, Z3);

        const Xexpected = F.div(expected.X, expected.Z);
        const Yexpected = F.div(expected.Y, expected.Z);

        expect(Xaff).to.equal(Xexpected);
        expect(Yaff).to.equal(Yexpected);
      });

      it("scalar mul by 1 returns original point", async () => {
        const circuit = await circom_tester.wasm("test/test_bandersnatch_scalar_mul.circom", { prime: "bls12381" });
        const P = generatePoint(F, a, d);
    
        const input = {
            X: P.x.toString(),
            Y: P.y.toString(),
            Z: "1",
            scalar: "1"
        };
    
        const witness = await circuit.calculateWitness(input, true);
        const X3 = BigInt(witness[1]);
        const Y3 = BigInt(witness[2]);
        const Z3 = BigInt(witness[3]);
    
        const Xaff = F.div(X3, Z3);
        const Yaff = F.div(Y3, Z3);
    
        expect(Xaff).to.equal(P.x);
        expect(Yaff).to.equal(P.y);
    });

    it("scalar mul by 111111 equals P added 111111 times", async () => {
        const circuit = await circom_tester.wasm("test/test_bandersnatch_scalar_mul.circom", { prime: "bls12381" });
        const P = generatePoint(F, a, d);

        // Circom scalar mul
        const witness = await circuit.calculateWitness({
            X: P.x.toString(),
            Y: P.y.toString(),
            Z: "1",
            scalar: "111111"
        }, true);

        const Xc = BigInt(witness[1]);
        const Yc = BigInt(witness[2]);
        const Zc = BigInt(witness[3]);

        // JS scalar multiplication via addition: 3P = P + P + P
        let acc = { X: 0n, Y: 1n, Z: 1n };
        for (let i = 0; i < 111111; i++) {
            acc = projectiveAdd(F, a, d, acc.X, acc.Y, acc.Z, P.x, P.y, 1n);
        }

        let expRes = scalarMulJS(F, a, d, P.x, P.y, 1n, 111111n);
        // Compare in projective coordinates (X1/Z1 = X2/Z2)
        expect(F.mul(Xc, acc.Z)).to.equal(F.mul(acc.X, Zc));
        expect(F.mul(Yc, acc.Z)).to.equal(F.mul(acc.Y, Zc));

        expect(F.mul(expRes.X, acc.Z)).to.equal(F.mul(acc.X, expRes.Z));
        expect(F.mul(expRes.Y, acc.Z)).to.equal(F.mul(acc.Y, expRes.Z));

    });

    it("scalar mul big number", async () => {
        const circuit = await circom_tester.wasm("test/test_bandersnatch_scalar_mul.circom", { prime: "bls12381" });
        const P = generatePoint(F, a, d);

        // Circom scalar mul
        const witness = await circuit.calculateWitness({
            X: P.x.toString(),
            Y: P.y.toString(),
            Z: "1",
            scalar: "123456789123456789123456789"
        }, true);

        const Xc = BigInt(witness[1]);
        const Yc = BigInt(witness[2]);
        const Zc = BigInt(witness[3]);

        let expRes = scalarMulJS(F, a, d, P.x, P.y, 1n, 123456789123456789123456789n);
        // Compare in projective coordinates (X1/Z1 = X2/Z2)

        expect(F.mul(expRes.X, Zc)).to.equal(F.mul(Xc, expRes.Z));
        expect(F.mul(expRes.Y, Zc)).to.equal(F.mul(Yc, expRes.Z));

    });
    
  });


  function scalarMulJS(F: any, a: bigint, d: bigint, X: bigint, Y: bigint, Z: bigint, n: bigint) {
    if (n === 0n) {
      return { X: 0n, Y: 1n, Z: 1n }; // Нейтральный элемент
    }
    
    let R = { X: 0n, Y: 1n, Z: 1n }; // Начинаем с нейтрального элемента
    let P = { X, Y, Z };
    
    while (n > 0n) {
      if (n & 1n) {
        // Если текущий бит равен 1, добавляем P к результату
        R = projectiveAdd(F, a, d, R.X, R.Y, R.Z, P.X, P.Y, P.Z);
      }
      // Удваиваем P для следующей итерации
      P = projectiveAdd(F, a, d, P.X, P.Y, P.Z, P.X, P.Y, P.Z);
      n >>= 1n; // Сдвигаем к следующему биту
    }
    
    return R;
}
