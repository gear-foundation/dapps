import { ReactElement } from 'react';

import { SignlessTransactionsMetadataProvider, SignlessTransactionsMetadataProviderProps } from './metadata-provider';
import { SignlessTransactionsSailsProvider, SignlessTransactionsSailsProviderProps } from './sails-provider';
import { BaseProgram } from './types';

function SignlessTransactionsProvider(props: SignlessTransactionsMetadataProviderProps): ReactElement;
function SignlessTransactionsProvider<TProgram extends BaseProgram>(
  props: SignlessTransactionsSailsProviderProps<TProgram>,
): ReactElement;

function SignlessTransactionsProvider<TProgram extends BaseProgram>(
  props: SignlessTransactionsMetadataProviderProps | SignlessTransactionsSailsProviderProps<TProgram>,
) {
  if ('metadataSource' in props) {
    return SignlessTransactionsMetadataProvider(props);
  } else if ('program' in props) {
    return <SignlessTransactionsSailsProvider {...props} />;
  } else {
    throw new Error('Invalid SignlessTransactionsProvider props');
  }
}

export { SignlessTransactionsProvider };
