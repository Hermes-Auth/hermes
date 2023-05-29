<script>
	import '../app.css';
    import { current_tab } from "../lib/store/index"
</script>

<div>
	<header class="flex justify-between p-2 border-y items-center">
		<a href="/" class="font-bold">Hermes</a>
		<a target="_blank" href="https://github.com/TheWisePigeon/hermes" class="">GitHub</a>
	</header>
	<header class="flex w-full justify-between p-2 border-b">
		<a class={` ${ $current_tab==='auth'?'bg-blue-300 text-white':'' } p-1`} href="/">Authentication</a>
		<a class={` ${ $current_tab==='apps'?'bg-blue-300 text-white':'' } p-1`} href="/apps">Applications</a>
		<a class={` ${ $current_tab==='profile'?'bg-blue-300 text-white':'' } p-1`} href="/profile">Profile</a>
	</header>
</div>
<div class="p-2">
	<slot />
</div>
