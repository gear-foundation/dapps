import react from '@vitejs/plugin-react-swc';
import path from 'path';
import { defineConfig } from 'vite';
import svgr from 'vite-plugin-svgr';
import { nodePolyfills } from 'vite-plugin-node-polyfills';
import { checker } from 'vite-plugin-checker';

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), svgr(), nodePolyfills(), checker({ typescript: true })],
  server: { port: 3000, open: true },
  preview: { port: 3000, open: true },
  resolve: { alias: { '@': path.resolve(__dirname, 'src') } },
  build: { outDir: 'build' },
});
