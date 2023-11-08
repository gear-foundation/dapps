import { FC, PropsWithChildren, SVGProps } from 'react';

export type ArrayElement<ArrayType extends readonly unknown[]> = ArrayType extends readonly (infer ElementType)[]
  ? ElementType
  : never;

export type BaseComponentProps = PropsWithChildren & {
  className?: string;
};

export type SVGComponent = FC<
  SVGProps<SVGSVGElement> & {
    title?: string | undefined;
  }
>;
