import fs from "fs";
// @ts-ignore
import { zKey } from "snarkjs";
import { resolve } from 'path';

// Преобразует bigint-строку в массив байт как hex
function toHexBytes(x: string, pad = 96): string[] {
  const hex = BigInt(x).toString(16).padStart(pad, "0");
  const bytes: string[] = [];
  for (let i = 0; i < hex.length; i += 2) {
    bytes.push(`0x${hex.slice(i, i + 2)}`);
  }
  return bytes;
}

function serializeG1(point: string[]): string[] {
  return point.flatMap(coord => toHexBytes(coord));
}

function serializeG2(g2: string[][][]): string[] {
  return g2.flat(2).flatMap(coord => toHexBytes(coord));
}

function serializeIC(ic: string[][]): string[][] {
  return ic.map(pt => serializeG1(pt));
}

async function main() {
    const zkeyPath = resolve(__dirname, '../circuits/build/shuffle_encrypt/shuffle_encrypt.zkey');
    const outPath = "verifying_key_bytes.rs";
  
    const buffer = fs.readFileSync(zkeyPath);
    const vk = await zKey.exportVerificationKey(new Uint8Array(buffer));
  
    const alpha = serializeG1(vk.vk_alpha_1); // G1
    const beta = serializeG2([vk.vk_beta_2]); // G2
    const gamma = serializeG2([vk.vk_gamma_2]);
    const delta = serializeG2([vk.vk_delta_2]);
    const ic = serializeIC(vk.IC);
  
    const rust = `\
  // Auto-generated verifying key constants
  
  pub const VK_ALPHA_G1_BETA_G2: [u8; ${alpha.length + beta.length}] = [
      ${[...alpha, ...beta].join(",\n    ")}
  ];
  
  pub const VK_GAMMA_G2_NEG_PC: [u8; ${gamma.length}] = [
      ${gamma.join(",\n    ")}
  ];
  
  pub const VK_DELTA_G2_NEG_PC: [u8; ${delta.length}] = [
      ${delta.join(",\n    ")}
  ];
  
  pub const VK_IC: [[u8; ${ic[0].length}]; ${ic.length}] = [
  ${ic.map(row => `    [${row.join(", ")}]`).join(",\n")}
  ];
  `;
  
    fs.writeFileSync(outPath, rust);
    console.log(`✅ Generated Rust consts at ${outPath}`);
  }
  
  main().catch(console.error);