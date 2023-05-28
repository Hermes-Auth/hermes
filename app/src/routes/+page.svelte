<script lang="ts">
	import LoadingIcon from '../components/LoadingIcon.svelte';
	const API_URL = 'http://localhost:5173';
	let loading = false;
	let error = '';
	let auth_state: 'Get Code' | 'Log in' = 'Get Code';
	let code_sent = false;
	let email = '';
	let code = '';

	async function authenticate() {
		loading = true;
		error = '';
		if (email === '') {
			error = 'Email is required';
			loading = false;
			return;
		}
		if (auth_state === 'Get Code') {
			const response = await fetch(`${API_URL}/api/v1/request`, {
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ email })
			});
			if (response.status === 200) {
				code_sent = true;
				loading = false;
				auth_state = 'Log in';
				return;
			}
			if (response.status === 500) {
				loading = false;
				error = 'Something went wrong. Please retry or contact the developer';
				return;
			}
		}
		if (auth_state === 'Log in') {
			if (email === '' || code === '') {
				loading = false;
				error = 'Email and Code are required';
				return;
			}
            const response = await fetch(
                `${API_URL}/api/v1`
            )
		}
	}
</script>

<h1 class="hover:underline text-xl font-bold"># Welcome to Hermes documentation</h1>

<h1>
	Hermes is a simple, developer friendly API that allows you to authenticate your users without
	thinking about passwords. Keep reading to know how you can use Hermes in your app today.
</h1>

<h1 id="#create-account" class="hover:underline mt-5 text-xl font-bold"># How it works</h1>

<!--be back-->

<h1 id="#create-account" class="hover:underline mt-5 text-xl font-bold">
	# Creating your developer account
</h1>

<form on:submit|preventDefault={authenticate} class="flex flex-col gap-2">
	<input
		class="p-2 border rounded-md border-gray-500 focus:outline-none"
		type="text"
		placeholder="Your email address"
	/>
	{#if code_sent}
		<input
			class="p-2 border rounded-md border-gray-500 focus:outline-none"
			type="text"
			placeholder="Your auth code"
		/>
	{/if}
	<span class="text-center text-red-600 text-xs">{error}</span>
	<button
		disabled={loading}
		class={`${
			loading ? 'bg-gray-500' : 'bg-gray-700'
		} flex justify-center p-2 rounded-md text-white`}
	>
		{#if loading}
			<LoadingIcon />
		{:else}
			{auth_state}
		{/if}
	</button>
</form>
