import { resolve } from 'path';

import react from '@vitejs/plugin-react-swc';
import { defineConfig } from 'vite';
import dts from 'vite-plugin-dts';
import svgr from 'vite-plugin-svgr';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), svgr(), dts()],
  resolve: { alias: { '@ez': resolve(__dirname, 'src') } },
  build: {
    lib: {
      entry: resolve(__dirname, 'src/index.ts'),
      formats: ['es'],
    },

    rollupOptions: {
      external: ['react', 'react-dom', '@gear-js/api', '@gear-js/react-hooks'],
      output: {
        intro: 'import "./gear-ez-transactions.css";',
      },
    },
  },
});
