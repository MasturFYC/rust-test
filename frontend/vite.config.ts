import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';

export default defineConfig({
	plugins: [sveltekit()],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	},
	server: {
		watch: { usePolling: true }
	},
	optimizeDeps: {
		exclude: [
			'carbon-components-svelte',
			'carbon-icons-svelte',
			'carbon-pictograms-svelte'
		]
	},
	css: {
		preprocessorOptions: {
			scss: {
				// additionalData: "@use \"src/variables.scss\" as *;"
				api: 'modern-compiler' // or 'modern'
			}
		}
	}
});
