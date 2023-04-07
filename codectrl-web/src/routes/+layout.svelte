<script lang="ts">
	import { page } from '$app/stores';
	import { handleSession } from '@lucia-auth/sveltekit/client';
	import { getUser } from '@lucia-auth/sveltekit/client';
	import '../app.css';

	handleSession(page);

	const user = getUser();
	console.table($user);
</script>

{#if !$user}
	<a href={$page.data.url}>Login</a>
{:else}
	<slot />
{/if}
