import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

import path from "path";

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      "@pages": path.resolve(__dirname, "src/pages"),
      "@router": path.resolve(__dirname, "src/router"),
      "@components": path.resolve(__dirname, "src/components"),
      "@containers": path.resolve(__dirname, "src/containers"),
      "@tools": path.resolve(__dirname, "src/tools"),
      "@app": path.resolve(__dirname, "src/app"),
      "@store": path.resolve(__dirname, "src/store"),
      "@/*": path.resolve(__dirname, "src/*")
    },
  },
});
