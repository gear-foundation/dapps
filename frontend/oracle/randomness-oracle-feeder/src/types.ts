export interface Random {
    round: number;
    randomness: Uint8Array[];
    signature: string;
    prevSignature: string;
}
