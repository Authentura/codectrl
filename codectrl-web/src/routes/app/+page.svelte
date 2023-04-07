<script lang="ts">
	import { onMount } from 'svelte';
	import { Elm } from './(app)/Main.elm';

	onMount(() => {
		let node = document.getElementById('app');
		const app = Elm.Main.init({ node });

		app.ports.sayHello.subscribe(function () {
			alert('Hello');
		});
	});
</script>

<div class="bg-backgroundDark" id="app" />
