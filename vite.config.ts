import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path'

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [react()],

    resolve: {
        alias: {
            '@': path.resolve(__dirname, './src'),
            '@apps': path.resolve(__dirname, './apps'),
        },
    },

    // Tauri development server
    server: {
        port: 5173,
        strictPort: true,
    },

    // Clear screen on reload
    clearScreen: false,

    // Env prefix for Tauri
    envPrefix: ['VITE_', 'TAURI_'],

    build: {
        // Tauri supports es2021
        target: ['es2021', 'chrome100', 'safari13'],
        // Don't minify for debug builds
        minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
        // Produce sourcemaps for debug builds
        sourcemap: !!process.env.TAURI_DEBUG,
    },
})
