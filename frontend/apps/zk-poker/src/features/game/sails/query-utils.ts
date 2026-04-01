import { UseQueryResult } from '@tanstack/react-query';
import { UseProgramQueryParameters, useProgramQuery } from '@gear-js/react-hooks';
import { QueryBuilder } from 'sails-js';

type NonServiceKeys = 'api' | 'registry' | 'programId' | 'newCtorFromCode' | 'newCtorFromCodeId';
type ServiceName<TProgram> = Exclude<keyof TProgram, NonServiceKeys>;
type QueryMethod = (...args: any[]) => QueryBuilder<any>;
type FunctionName<TService> = {
  [K in keyof TService]: TService[K] extends QueryMethod ? K : never;
}[keyof TService];
type QueryArgs<TMethod> = TMethod extends (...args: infer P) => QueryBuilder<any> ? P : never;
type QueryReturn<TMethod> = TMethod extends (...args: any[]) => QueryBuilder<infer R> ? R : never;

type TypedQueryResult<TData> = UseQueryResult<TData, Error> & {
  queryKey: (string | undefined)[];
};

type TypedUseProgramQueryParameters<
  TProgram,
  TServiceName extends ServiceName<TProgram>,
  TFunctionName extends FunctionName<TProgram[TServiceName]>,
  TMethod extends Extract<TProgram[TServiceName][TFunctionName], QueryMethod> = Extract<
    TProgram[TServiceName][TFunctionName],
    QueryMethod
  >,
  TData = QueryReturn<TMethod>,
> = UseProgramQueryParameters<TProgram, TServiceName, TFunctionName, QueryArgs<TMethod>, QueryReturn<TMethod>, TData>;

export const useTypedProgramQuery = <
  TProgram,
  TServiceName extends ServiceName<TProgram>,
  TFunctionName extends FunctionName<TProgram[TServiceName]>,
  TMethod extends Extract<TProgram[TServiceName][TFunctionName], QueryMethod> = Extract<
    TProgram[TServiceName][TFunctionName],
    QueryMethod
  >,
  TData = QueryReturn<TMethod>,
>(
  parameters: TypedUseProgramQueryParameters<TProgram, TServiceName, TFunctionName, TMethod, TData>,
) => useProgramQuery(parameters as never) as TypedQueryResult<TData>;
