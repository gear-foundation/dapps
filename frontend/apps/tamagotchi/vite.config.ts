import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react-swc';
import path from 'path';
import nodePolyfills from 'vite-plugin-node-stdlib-browser';
import eslint from 'vite-plugin-eslint';
import checker from 'vite-plugin-checker';

// https://vitejs.dev/config/
export default defineConfig({
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
    },
  },
  server: {
    port: 3000,
  },
  preview: {
    port: 3000,
  },
  plugins: [react(), nodePolyfills(), eslint(), checker({ typescript: true })],
  assetsInclude: ['**/*.wasm?inline'],
  build: { outDir: 'build' },
});
