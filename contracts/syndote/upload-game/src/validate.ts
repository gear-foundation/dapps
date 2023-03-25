import { IScheme } from './types';

function getValue(v: any) {
  return JSON.stringify(v);
}

export function validateScheme(scheme: IScheme) {
  const errors = [];
  if (scheme.fund_accounts && !scheme.prefunded_account) {
    errors.push(`prefunded_account is not specified. It's required if you want to fund some account`);
  }
  const programIds = [];
  const codeIds = [];

  for (const program of scheme.programs) {
    if (program.id === undefined) {
      errors.push({ error: `Program id is not specified`, value: getValue(program) });
    }
    programIds.push(program.id);
    if (!program.name) {
      errors.push({ error: `Program name is not specified.`, value: getValue(program) });
    }
    if (!program.path_to_wasm) {
      errors.push({ error: `Path to program is not specified.`, value: getValue(program) });
    }
    if (program.payload && !program.path_to_meta) {
      errors.push({
        error: `Path to meta is not specified. It's required if you use payload`,
        value: getValue(program),
      });
    }
  }

  if (scheme.codes) {
    for (const code of scheme.codes) {
      if (code.id === undefined) {
        errors.push({ error: `Code id is not specified.`, value: getValue(code) });
      }
      codeIds.push(code.id);
      if (!code.name) {
        errors.push({ error: `Code name is not specified.`, value: getValue(code) });
      }
      if (!code.path_to_wasm) {
        errors.push({ error: `Path to code is not specified.`, value: getValue(code) });
      }
    }
  }

  for (const transaction of scheme.transactions) {
    if (transaction.type === 'upload_program' || transaction.type === 'send_message') {
      if (transaction.program === undefined) {
        errors.push({ error: `Program id is not specified.`, value: getValue(transaction) });
      }
      if (!programIds.includes(transaction.program)) {
        errors.push({ error: `Program with id ${transaction.program} not found`, value: getValue(transaction) });
      }
    }
    if (transaction.type === 'upload_code') {
      if (transaction.code === undefined) {
        errors.push({ error: `Code id is not specified.`, value: getValue(transaction) });
      }
      if (!codeIds.includes(transaction.code)) {
        errors.push({ error: `Code with id ${transaction.code} not found`, value: getValue(transaction) });
      }
    }

    if (!transaction.account === undefined) {
      errors.push({ error: `Account is not specified.`, value: getValue(transaction) });
    }
  }
  if (errors.length > 0) {
    return errors;
  }
  return null;
}
