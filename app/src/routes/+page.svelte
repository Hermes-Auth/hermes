<script lang="ts">
	import LoadingIcon from '../components/LoadingIcon.svelte';
	import { user_is_logged_in } from '$lib/store';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
    import { current_tab } from '$lib/store';
	onMount(() => {
        current_tab.set("auth")
		if (browser) {
			const token = localStorage.getItem('token') ?? '';
			if (token !== '') {
				user_is_logged_in.set(true);
			}
		}
	});
	const API_URL = 'http://localhost:5173';
	let loading = false;
	let error = '';
	let auth_state: 'Get Code' | 'Log in' = 'Get Code';
	let code_sent = false;
	let email = '';
	let code = '';

	async function authenticate() {
		try {
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
					body: JSON.stringify({ email }),
					method: 'POST'
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
				const response = await fetch(`${API_URL}/api/v1/authenticate`, {
					headers: {
						'Content-Type': 'application/json'
					},
					body: JSON.stringify({ user: email, code }),
					method: 'POST'
				});
				if (response.status === 200) {
					const { id } = (await response.json()) as { id: string };
					localStorage.setItem('token', id);
                    loading = false;
					alert('You are now authenticated. Check the `Profile` tab to see your api key');
                    user_is_logged_in.set(true)
					return;
				}
				if (response.status === 400) {
					error = 'Code invalid or expired';
					loading = false;
				}
				if (response.status === 500) {
					error = 'Something went wrong. Please retry or contact the developer';
					loading = false;
				}
			}
		} catch (err) {
			console.log(err);
			loading = false;
			error = 'Something went wrong. Please retry or contact the developer';
		}
	}

	function logout() {
		localStorage.removeItem('token');
		user_is_logged_in.set(false);
	}
	const steps: { link: string; description: string }[] = [
		{
			link: '#create-account',
			description: 'Create or log in your developer account'
		},
		{
			link: '/profile',
			description: 'Check out your profile'
		},
		{
			link: '/apps',
			description: 'Create and manage your apps'
		}
	];
</script>

<h1 class="hover:underline text-xl font-bold"># Welcome to Hermes documentation</h1>

<h1>
	Hermes is a simple, developer friendly API that allows you to authenticate your users without
	thinking about passwords. Keep reading to know how you can use Hermes in your app today.
</h1>

<h1 id="#create-account" class="hover:underline mt-5 text-xl font-bold"># How it works</h1>
<ol>
	{#each steps as step}
		<li>
			<a href={step.link} class="text-blue-500 underline">{step.description}</a>
		</li>
	{/each}
</ol>

<!--be back-->

<h1 id="#create-account" class="hover:underline my-5 text-xl font-bold">
	# Creating your developer account
</h1>

{#if !$user_is_logged_in}
	<form on:submit|preventDefault={authenticate} class="flex flex-col gap-2">
		<input
			bind:value={email}
			class="p-2 border rounded-md border-gray-500 focus:outline-none"
			type="text"
			placeholder="Your email address"
		/>
		{#if code_sent}
			<input
				bind:value={code}
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
{:else}
	<h1 class="text-center">You are logged in</h1>
	<button on:click={logout} class=" bg-red-600 w-full flex justify-center p-2 rounded-md text-white"
		>Log out</button
	>
{/if}
