import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import { plugin as elm } from 'vite-plugin-elm';

export default defineConfig({
	plugins: [sveltekit(), elm()]
});
