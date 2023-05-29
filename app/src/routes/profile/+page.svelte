<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import Copy from '../../components/Copy.svelte';
	import LoadingIcon from '../../components/LoadingIcon.svelte';
	import { browser } from '$app/environment';
    import { current_tab } from '$lib/store';
	onMount(async () => {
        current_tab.set("profile")
		if (browser) {
			try {
				const token = localStorage.getItem('token') ?? '';
				if (token === '') {
					alert('You are not authenticated');
					goto('/');
				}
				const data = await fetch('http://localhost:5173/api/v1/profile', {
					headers: {
						Authorization: token
					}
				});
				if (data.status === 200) {
					user_info = (await data.json()) as { api_key: string };
					loading = false;
				}
			} catch (err) {
				console.log(err);
                alert("Something went wrong. Please retry or contact developer support")
			}
		}
	});
	$: loading = true;
	let user_info: { api_key: string } = { api_key: '' };
	function copy_api_key() {
		navigator.clipboard.writeText(user_info.api_key);
		alert('Key copied to clipboard');
	}
	async function reset_api_key() {
		const response = await fetch('http://localhost:5173/api/v1/profile', {
			method: 'PUT',
			headers: {
				Authorization: localStorage.getItem('token')!
			}
		});
		if (response.status === 200) {
			alert('Your API key was reset. Reload the page to get access to the new one');
			window.location.href = '/profile';
		}
		alert('Something went wrong. Please retry or contact developer support');
	}
</script>

<h1 class="font-bold text-xl">Developer Profile</h1>

<div class="flex justify-between my-3 items-center">
	<h1>Your api key</h1>
	<button
		on:click={reset_api_key}
		class="w-32 p-2 rounded-md flex justify-center bg-red-500 text-white"
	>
		{#if loading}
			<LoadingIcon />
		{:else}
			Reset API key
		{/if}
	</button>
</div>
<div class=" flex justify-between bg-gray-400 rounded-md text-white p-2">
	<h1>******************</h1>
	<button title="Copy to clipboard" on:click={copy_api_key}>
		{#if loading}
			<LoadingIcon />
		{:else}
			<Copy />
		{/if}
	</button>
</div>

<h1 class="font-bold text-xl my-4">Stats</h1>
<h1>Coming soon</h1>
