import { defineConfig, splitVendorChunkPlugin } from 'vite';
import react from '@vitejs/plugin-react-swc';
import path from 'path';
import nodePolyfills from 'vite-plugin-node-stdlib-browser';
import eslint from 'vite-plugin-eslint';
import svgr from 'vite-plugin-svgr';

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => {
  return {
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
    build: {
      outDir: 'build',
      sourcemap: true,
      rollupOptions: {
        output: {
          manualChunks(id) {
            if (
              id.includes('@polkadot') ||
              id.includes('@substrate') ||
              id.includes('@scure') ||
              id.includes('@noble')
            ) {
              return 'polkadot';
            }
            if (id.includes('@open-ish') || id.includes('tslib')) {
              return '@open-ish';
            }
            if (id.includes('react-router') || id.includes('@remix-run')) {
              return '@react-router';
            }
            if (id.includes('@headlessui') || id.includes('@radix-ui') || id.includes('framer-motion')) {
              return 'app-ui';
            }
            if (id.includes('@gear-js') || id.includes('react-transition-group')) {
              return '@gear-js';
            }
            if (id.includes('@sentry')) {
              return '@sentry';
            }
          },
        },
      },
    },
    plugins: [splitVendorChunkPlugin(), svgr(), react(), nodePolyfills(), eslint()],
    assetsInclude: ['**/*.wasm?inline', '**/*.txt?inline'],
  };
});
