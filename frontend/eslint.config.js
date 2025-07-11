import js from '@eslint/js';
import globals from 'globals';
import react from 'eslint-plugin-react';
import reactHooks from 'eslint-plugin-react-hooks';
import reactRefresh from 'eslint-plugin-react-refresh';
import tseslint from 'typescript-eslint';
import jsxA11y from 'eslint-plugin-jsx-a11y';
import importPlugin from 'eslint-plugin-import';
import eslintPluginPrettierRecommended from 'eslint-plugin-prettier/recommended';

export default tseslint.config(
  { ignores: ['dist', 'build'] },

  {
    extends: [
      js.configs.recommended,
      ...tseslint.configs.recommendedTypeChecked,
      react.configs.flat.recommended,
      react.configs.flat['jsx-runtime'],
      jsxA11y.flatConfigs.recommended,
      importPlugin.flatConfigs.recommended,
      importPlugin.flatConfigs.typescript,
      eslintPluginPrettierRecommended,
    ],

    files: ['**/*.{ts,tsx}'],

    languageOptions: {
      ecmaVersion: 2021,
      globals: globals.browser,

      // https://typescript-eslint.io/getting-started/typed-linting
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },

    // TODO: simplify after updates?
    plugins: {
      'react-hooks': reactHooks,
      'react-refresh': reactRefresh,
    },

    settings: {
      react: { version: 'detect' },
      'import/resolver': {
        typescript: {
          alwaysTryTypes: true,
          project: ['./apps/*/tsconfig.json', './packages/*/tsconfig.json'],
        },
      },
    },

    rules: {
      // plugins
      ...reactHooks.configs.recommended.rules,
      'react-refresh/only-export-components': ['warn', { allowConstantExport: true }],

      // import sort
      'import/order': [
        1,
        {
          groups: ['builtin', 'external', 'internal', 'parent', 'sibling', 'index'],
          'newlines-between': 'always',
          alphabetize: { order: 'asc' },
          pathGroups: [
            { pattern: '@dapps-frontend/**', group: 'internal', position: 'before' },
            { pattern: '@/**', group: 'internal', position: 'before' },
          ],
          pathGroupsExcludedImportTypes: ['builtin', 'object'], // override default for internal
        },
      ],

      // dx
      '@typescript-eslint/no-unused-vars': [
        'warn',
        { argsIgnorePattern: '^_', varsIgnorePattern: '^_', caughtErrorsIgnorePattern: '^_' },
      ],

      'no-shadow': 'error',
      'no-shadow-restricted-names': 'error',

      // react-hook-form onSubmit
      '@typescript-eslint/no-misused-promises': [2, { checksVoidReturn: { attributes: false } }],

      // we're using typescript
      'react/prop-types': 'off',
      'import/no-named-as-default': 'off',
    },
  },
);
