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
    checker({ typescript: true }),
    checker({ eslint: { lintCommand: 'eslint "./src/**/*.{ts,tsx}"', useFlatConfig: true }, enableBuild: false }),
  ],

  server: { port: 3000, open: true },
  preview: { port: 3000, open: true },

  // process.cwd to resolve to the launch directory
  resolve: { alias: { '@': path.resolve(process.cwd(), 'src') } },

  build: { outDir: 'build' },
});

export { viteAppsConfig };
