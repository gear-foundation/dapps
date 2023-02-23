const plugin = require('tailwindcss/plugin');
const defaultTheme = require('tailwindcss/defaultTheme');

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{ts,tsx}'],
  theme: {
    extend: {
      animation: {
        'ping-slow': 'ping 3s linear infinite',
        'pulse-slow': 'pulse 3s ease-in-out infinite',
        'pulse-once': 'pulse-once 1.5s ease-in-out',
        wiggle: 'wiggle 1s ease-in-out infinite',
        'battle-turn-1': 'turn 2s linear infinite',
        'battle-turn-2': 'turn 2s linear infinite 400ms',
        'battle-turn-3': 'turn 2s linear infinite 900ms',
        wave: 'wave 3s ease-in infinite',
        'wave-2': 'wave 2s linear infinite 500ms',
      },
      keyframes: {
        wiggle: {
          '0%, 100%': { transform: 'rotate(-3deg)' },
          '50%': { transform: 'rotate(3deg)' },
        },
        turn: {
          '0%': { opacity: '50%' },
          '50%': { opacity: '25%' },
          '100%': { opacity: '15%' },
        },
        wave: {
          '0%, 100%': { opacity: 0.75, transform: 'scale(1) translateY(-50%)' },
          '50%': { opacity: 1, transform: 'scale(1.05) translateY(-50%)' },
        },
        'pulse-once': {
          '0%': { opacity: 0 },
          '100%': { opacity: 1 },
        },
      },
      colors: {
        current: 'currentColor',
        secondary: 'rgb(var(--color-secondary) / <alpha-value>)',
        primary: 'rgb(var(--color-primary) / <alpha-value>)',
        tertiary: 'rgb(var(--red) / <alpha-value>)',
        error: 'rgb(var(--color-error) / <alpha-value>)',
        'dark-500': 'rgb(var(--color-dark-500) / <alpha-value>)',
        light: 'rgb(var(--color-light) / <alpha-value>)',
      },
      fontFamily: {
        kanit: ['Kanit', ...defaultTheme.fontFamily.sans],
        poppins: ['Poppins', ...defaultTheme.fontFamily.sans],
      },
      fontSize: {
        xxs: ['10px', '18px'],
        xs: ['12px', '16px'],
        sm: ['14px', '20px'],
        base: ['16px', '24px'],
        lg: ['18px', '20px'],
        xl: ['24px', '32px'],
        '2xl': ['28px', '32px'],
        h2: ['40px', { lineHeight: '48px', fontWeight: 700, letterSpacing: '-0.02em' }],
      },
      opacity: {
        15: '.15',
      },
      screens: {
        xxs: '335px',
        xs: '375px',
        sm: '475px',
        md: '768px',
        lg: '1024px',
        xl: '1280px',
        xxl: '1540px',
        xl2k: '1920px',
        mxl: { max: '1279px' },
        mlg: { max: '1023px' },
        mmd: { max: '767px' },
        msm: { max: '474px' },
        mxs: { max: '374px' },
      },
      spacing: {
        2.5: '0.625rem',
        4.5: '1.125rem',
        5.5: '1.375rem',
        7.5: '1.875rem',
        13: '3.25rem',
        15: '3.75rem',
        17: '4.25rem',
        17.5: '4.375rem',
        18: '4.5rem',
        19: '4.75rem',
        22: '5.5rem',
        22.5: '5.625rem',
        25: '6.25rem',
        26: '6.5rem',
        27: '6.75rem',
        30: '7.5rem',
        31: '7.75rem',
        32: '8rem',
        33: '8.25rem',
        34: '8.5rem',
        35: '8.75rem',
        36: '9rem',
        37: '9.25rem',
        37.5: '9.375rem',
        38: '9.5rem',
        42: '10.5rem',
        43: '10.75rem',
        44: '11rem',
        45: '11.25rem',
        46: '11.5rem',
        47: '11.75rem',
        48: '12rem',
        49: '12.25rem',
        50: '12.5rem',
        55: '13.75rem',
        56: '14rem',
        57: '14.25rem',
        58: '14.5rem',
        59: '14.75rem',
        60: '15rem',
        62: '15.5rem',
        62.5: '15.625rem',
        63: '15.75rem',
        64: '16rem',
      },
      zIndex: {
        1: '1',
        2: '2',
      },
    },
  },
  corePlugins: {
    container: false,
    // preflight: false,
  },
  plugins: [
    plugin(function ({ addUtilities, addComponents, e, prefix, config }) {
      const newUtilities = {
        '.horizontal-tb': {
          writingMode: 'horizontal-tb',
        },
        '.vertical-rl': {
          writingMode: 'vertical-rl',
        },
        '.vertical-lr': {
          writingMode: 'vertical-lr',
        },
      };
      addUtilities(newUtilities);
    }),
  ],
};
