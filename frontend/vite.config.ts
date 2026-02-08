import { defineConfig } from 'vite';

export default defineConfig({
  server: {
    port: 3234,
  },
  build: {
    outDir: 'dist',
    emptyOutDir: true,
    minify: 'esbuild',
    sourcemap: false,
  },
});
