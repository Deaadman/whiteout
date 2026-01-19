import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
    plugins: [tailwindcss(), sveltekit()],
    server: {
        proxy: {
            '/tldmods': {
                target: 'https://tldmods.com',
                changeOrigin: true,
                rewrite: (path) => path.replace(/^\/tldmods/, '')
            }
        }
    }
});
