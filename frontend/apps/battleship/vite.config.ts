import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react-swc';
import path from 'path';
import { nodePolyfills } from 'vite-plugin-node-polyfills';
import svgr from 'vite-plugin-svgr';
import { checker } from 'vite-plugin-checker';

// https://vitejs.dev/config/
export default defineConfig({
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
    },
  },
  server: {
    host: '0.0.0.0',
    port: 3000,
  },
  preview: {
    port: 3000,
    host: true,
  },
  plugins: [
    svgr(),
    react(),
    // disabling modules used in packages
    nodePolyfills({
      globals: { process: false, global: false },
    }),
    checker({
      typescript: true,
      eslint: { lintCommand: 'eslint "./src/**/*.{ts,tsx}"', dev: { logLevel: ['error'] } },
    }),
  ],
  assetsInclude: ['**/*.wasm?inline', '**/*.txt?inline'],
  build: { outDir: 'build' },
});
