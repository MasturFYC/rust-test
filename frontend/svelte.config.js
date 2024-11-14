import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { optimizeCss, optimizeImports } from "carbon-preprocess-svelte";
import path from 'path';
import preprocess from 'svelte-preprocess';
import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: [
        vitePreprocess(),
        preprocess(),
        optimizeImports(),
        optimizeCss()
    ],

	kit: {
		// adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
		// If your environment is not supported or you settled on a specific environment, switch out the adapter.
		// See https://kit.svelte.dev/docs/adapters for more information about adapters.
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			precompress: false,
			strict: true,
			fallback: 'index.html'
		}),
		alias: {
			$assets: path.resolve('./src/assets')
		}
		// prerender: {
		// 	handleHttpError: ({ path, referrer, message }) => {
		// 		// ignore deliberate link to shiny 404 page
		// 		if (path === '/not-found' && referrer === '/blog/how-we-built-our-404-page') {
		// 			return;
		// 		}

		// 		// otherwise fail the build
		// 		throw new Error(message);
		// 	}
		// }
	}
};

export default config;
