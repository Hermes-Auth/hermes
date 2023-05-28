<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import Copy from '../../components/Copy.svelte';
	onMount(() => {
		const token = localStorage.getItem('token') ?? '';
		if (token === '') {
			alert('You are not authenticated');
			goto('/');
		}
	});
	function copy_api_key() {
		navigator.clipboard.writeText('some shitty api key');
        alert("Key copied to clipboard")
	}
    function hide_api_key(key:string){
            let str = ""
            for (const _ of key) {
                str+="*"
            }
            return str
        }
</script>

<h1 class="font-bold text-xl">Developer Profile</h1>

<div class="flex justify-between my-3 items-center">
    <h1>Your api key</h1>
    <button class="p-2 rounded-md flex justify-center bg-red-500 text-white">Reset api key</button>
</div>
<div class=" flex justify-between bg-gray-400 rounded-md text-white p-2">
	<h1>{hide_api_key("some-shitty-uuid")}</h1>
	<button title="Copy to clipboard" on:click={copy_api_key}>
		<Copy />
	</button>
</div>
