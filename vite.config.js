import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import tailwindcss from "@tailwindcss/vite";
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  define: {
    __IS_MOBILE__: !!/android|ios/.exec(process.env.TAURI_ENV_PLATFORM),
  },
	plugins: [tailwindcss(), sveltekit()],
	 // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  server: {
    host: host || false,
    port: 1420,
    strictPort: true,
    hmr: host
      ? {
          protocol: 'ws',
          host: host,
          port: 1430,
        }
      : undefined,
  },
});