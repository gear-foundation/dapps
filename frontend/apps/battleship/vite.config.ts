import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react-swc';
import path from 'path';
import nodePolyfills from 'vite-plugin-node-stdlib-browser';
import eslint from 'vite-plugin-eslint';
import svgr from 'vite-plugin-svgr';

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
  plugins: [svgr(), react(), nodePolyfills(), eslint()],
  assetsInclude: ['**/*.wasm?inline', '**/*.txt?inline'],
  build: { outDir: 'build' },
});
