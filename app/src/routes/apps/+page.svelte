<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
    import { current_tab } from '$lib/store';

	onMount(() => {
        current_tab.set("apps")
		const token = localStorage.getItem('token') ?? '';
		if (token === '') {
			alert('You are not authenticated');
			goto('/');
		}
	});
</script>

<h1 class="font-bold text-xl">Your Apps</h1>
<h1>Apps exist because you might want to use Hermes to authenticate users from many of your apps</h1>
<h1>Creating an app is as easy as filling the form below. Here are some explanations about what every field mean</h1>
