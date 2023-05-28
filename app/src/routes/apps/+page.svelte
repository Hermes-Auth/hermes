<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	onMount(() => {
		const token = localStorage.getItem('token') ?? '';
		if (token === '') {
			alert('You are not authenticated');
			goto('/');
		}
	});
</script>
