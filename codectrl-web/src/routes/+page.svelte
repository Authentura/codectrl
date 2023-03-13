<script lang="ts">
	import { enhance } from '$app/forms';
	import { getUser } from '@lucia-auth/sveltekit/client';

	const user = getUser();
	console.table($user);
</script>

<form use:enhance method="post">
	<input type="submit" class="button" value="Sign out" />
</form>

<h1>Hello {$user?.name} (@{$user?.username})!</h1>

<img src={$user?.profile_picture} alt={`${$user?.name}'s Profile Picture`} />
