import { type VariantProps } from 'class-variance-authority';
import { BaseHTMLAttributes } from 'react';

import { textVariants } from './text-variants';

export interface TextProps extends BaseHTMLAttributes<HTMLParagraphElement>, VariantProps<typeof textVariants> {}

export function Text({ children, className, size, weight, ...props }: TextProps) {
  return (
    <p className={textVariants({ size, weight, className })} {...props}>
      {children}
    </p>
  );
}
