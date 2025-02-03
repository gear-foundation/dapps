import react from '@vitejs/plugin-react-swc';
import path from 'path';
import { defineConfig } from 'vite';
import svgr from 'vite-plugin-svgr';
import { nodePolyfills } from 'vite-plugin-node-polyfills';
import { checker } from 'vite-plugin-checker';

// https://vite.dev/config/
const viteAppsConfig = defineConfig({
  plugins: [react(), svgr(), nodePolyfills(), checker({ typescript: true })],
  server: { port: 3000, open: true },
  preview: { port: 3000, open: true },

  // process.cwd to resolve to the launch directory
  resolve: { alias: { '@': path.resolve(process.cwd(), 'src') } },

  build: { outDir: 'build' },
});

export { viteAppsConfig };
