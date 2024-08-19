import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react-swc';
import path from 'path';
import { nodePolyfills } from 'vite-plugin-node-polyfills';
import eslint from 'vite-plugin-eslint';

// https://vitejs.dev/config/
export default defineConfig({
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
      '@/assets': path.resolve(__dirname, './src/assets'),
    },
  },
  server: {
    port: 3000,
  },
  preview: {
    port: 3000,
  },
  base: './',
  plugins: [
    react(),
    // process is used in the error-tracking package to get envs
    nodePolyfills({
      globals: { process: false },
    }),
    eslint(),
  ],
  assetsInclude: ['**/*.wasm?inline'],
  build: { outDir: 'build' },
});
