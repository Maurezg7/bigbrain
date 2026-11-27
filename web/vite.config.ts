import adapter from '@sveltejs/adapter-static';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

const api = process.env.BIGBRAIN_API ?? 'http://localhost:8787';

export default defineConfig({
	plugins: [
		sveltekit({
			compilerOptions: {
				runes: ({ filename }) =>
					filename.split(/[/\\]/).includes('node_modules') ? undefined : true
			},
			// SPA estática: el servidor Rust la sirve junto con la API.
			adapter: adapter({ fallback: 'index.html' })
		})
	],
	server: {
		proxy: { '/api': api, '/media': api }
	}
});
