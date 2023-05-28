<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import Copy from '../../components/Copy.svelte';
	import LoadingIcon from '../../components/LoadingIcon.svelte';
	onMount(async () => {
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
			user_info = await data.json() as { api_key:string } ;
            loading = false
		}
	});
	$: loading = true;
    let user_info : { api_key:string } = { api_key:"" }
	function copy_api_key() {
		navigator.clipboard.writeText(user_info.api_key);
		alert('Key copied to clipboard');
	}
</script>

<h1 class="font-bold text-xl">Developer Profile</h1>

<div class="flex justify-between my-3 items-center">
	<h1>Your api key</h1>
	<button class="p-2 rounded-md flex justify-center bg-red-500 text-white">Reset api key</button>
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
