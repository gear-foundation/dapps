import { VariantProps } from 'class-variance-authority';
import { BaseHTMLAttributes } from 'react';

import { headingVariants } from './heading-variants';

export interface HeadingProps extends BaseHTMLAttributes<HTMLHeadingElement>, VariantProps<typeof headingVariants> {}

export function Heading({ children, className, size, weight, ...props }: HeadingProps) {
  return (
    <h2 className={headingVariants({ size, weight, className })} {...props}>
      {children}
    </h2>
  );
}
