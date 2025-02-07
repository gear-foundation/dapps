import path from 'path';

import react from '@vitejs/plugin-react-swc';
import { defineConfig } from 'vite';
import { checker } from 'vite-plugin-checker';
import { nodePolyfills } from 'vite-plugin-node-polyfills';
import svgr from 'vite-plugin-svgr';

// https://vite.dev/config/
const viteAppsConfig = defineConfig({
  plugins: [
    react(),
    svgr(),
    nodePolyfills(),

    // TODO: replace with one checker after eslint issues are resolved
    // TODO: eslint src only runs through the current app, need to lint workspace packages too
    checker({ typescript: true }),
    checker({ eslint: { lintCommand: 'eslint "./src/**/*.{ts,tsx}"', useFlatConfig: true }, enableBuild: false }),
  ],

  server: { port: 3000, open: true },
  preview: { port: 3000, open: true },

  resolve: {
    alias: {
      '@': path.resolve(process.cwd(), 'src'), // process.cwd to resolve to the launch directory
      '@ui': path.resolve(__dirname, 'packages/ui/src'),
      '@ez': path.resolve(__dirname, 'packages/ez-transactions/src'),
      'gear-ez-transactions': path.resolve(__dirname, 'packages/ez-transactions/src'),
    },
  },

  build: { outDir: 'build' },
});

export { viteAppsConfig };
