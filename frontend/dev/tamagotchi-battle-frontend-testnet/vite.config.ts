import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import path from 'path'
import nodePolyfills from 'vite-plugin-node-stdlib-browser'
import eslint from 'vite-plugin-eslint'
import svgr from 'vite-plugin-svgr'
// import autoprefixer from 'autoprefixer'

// import wasm from "vite-plugin-wasm";
// import topLevelAwait from "vite-plugin-top-level-await";

// https://vitejs.dev/config/
export default defineConfig({
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
      // "@/app": path.resolve(__dirname, "./src/app"),
      // "@/assets": path.resolve(__dirname, "./src/assets"),
      // "@/components": path.resolve(__dirname, "./src/components"),
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
    // wasm(), topLevelAwait(),
    svgr(),
    react(),
    nodePolyfills(),
    eslint(),
  ],
  assetsInclude: ['**/*.wasm?inline', '**/*.txt?inline'],
  // css: {
  //   postcss: {
  //     plugins: [autoprefixer],
  //   },
  // },
})
