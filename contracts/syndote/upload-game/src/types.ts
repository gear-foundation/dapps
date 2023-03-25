import { ProgramMetadata } from '@gear-js/api';

export interface IProgram extends Omit<SchemeProgram, 'id'> {
  address?: `0x${string}`;
  meta?: ProgramMetadata;
}

export interface ICode extends Omit<SchemeCode, 'id'> {
  hash?: `0x${string}`;
}

export interface SchemeProgram {
  name: string;
  id: number;
  path_to_wasm: string;
  path_to_meta: string;
  payload: any;
  value?: number;
}

export interface SchemeCode {
  name: string;
  id: number;
  path_to_wasm: string;
}

export interface UploadProgramTransactionScheme {
  type: 'upload_program';
  program: number;
  account: string;
  payload: any;
  value?: number;
  increase_gas?: number;
}

export interface SendMessageTransactionScheme {
  type: 'send_message';
  program: number;
  account: string;
  payload: any;
  value?: number;
  increase_gas?: number;
}

export interface UploadCodeTransactionScheme {
  type: 'upload_code';
  code: number;
  account: string;
}

export type SchemeTransaction =
  | SendMessageTransactionScheme
  | UploadCodeTransactionScheme
  | UploadProgramTransactionScheme;

export interface IScheme {
  wsAddress: `ws://${string}` | `wss://${string}`;
  accounts: Record<string, string>;
  prefunded_account?: string;
  fund_accounts?: Array<string>;
  programs: Array<SchemeProgram>;
  codes?: Array<SchemeCode>;
  transactions: Array<SchemeTransaction>;
}
