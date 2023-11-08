import { defineConfig, loadEnv, splitVendorChunkPlugin } from 'vite';
import react from '@vitejs/plugin-react-swc';
import path from 'path';
import nodePolyfills from 'vite-plugin-node-stdlib-browser';
import eslint from 'vite-plugin-eslint';
import svgr from 'vite-plugin-svgr';
import { sentryVitePlugin } from '@sentry/vite-plugin';

// import { visualizer } from 'rollup-plugin-visualizer'
// import autoprefixer from 'autoprefixer'
// import wasm from "vite-plugin-wasm";
// import topLevelAwait from "vite-plugin-top-level-await";

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), '');
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
      sourcemap: true,
      rollupOptions: {
        output: {
          manualChunks(id) {
            // if (id.includes('@polkadot/types')) {
            //   return 'polkadot-types'
            // }
            if (
              id.includes('@polkadot') ||
              id.includes('@substrate') ||
              id.includes('@scure') ||
              id.includes('@noble')
              // id.includes('rxjs') ||
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
    plugins: [
      // wasm(), topLevelAwait(),
      // visualizer(),
      splitVendorChunkPlugin(),
      svgr(),
      react(),
      nodePolyfills(),
      eslint(),
      // Put the Sentry vite plugin after all other plugins
      sentryVitePlugin({
        disable: (process.env.NODE_ENV || env.NODE_ENV) !== 'production',
        url: 'https://sentry.vara-network.io',
        org: process.env.SENTRY_ORG || env.SENTRY_ORG,
        project: process.env.SENTRY_PROJECT_TTT || env.SENTRY_PROJECT_TTT,
        authToken: process.env.SENTRY_AUTH_TOKEN || env.SENTRY_AUTH_TOKEN,
        telemetry: false,
      }),
    ],
    assetsInclude: ['**/*.wasm?inline', '**/*.txt?inline'],
  };
});
