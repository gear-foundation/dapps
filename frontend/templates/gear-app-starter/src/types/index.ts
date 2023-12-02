import { HexString } from '@polkadot/util/types';
import { FC, PropsWithChildren, SVGProps } from 'react';

export type BaseComponentProps = PropsWithChildren & {
  className?: string;
};

export type PickPartial<T, K extends keyof T> = T | Pick<T, K>;

export type Entries<T> = {
  [K in keyof T]: [K, T[K]];
}[keyof T][];

export type SVGComponent = FC<
  SVGProps<SVGSVGElement> & {
    title?: string | undefined;
  }
>;

export type ArrayElement<ArrayType extends readonly unknown[]> = ArrayType extends readonly (infer ElementType)[]
  ? ElementType
  : never;

export type Token = {
  id: string;
  ownerId: HexString;
  name: string;
  description: string;
  media: string;
  reference: string;
  approvedAccountIds: HexString[];
};
