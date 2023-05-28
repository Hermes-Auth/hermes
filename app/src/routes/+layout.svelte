<script>
	import '../app.css';
</script>

<div>
	<header class="flex justify-between p-2 border-y items-center">
		<a href="/" class="font-bold">Hermes</a>
		<a target="_blank" href="https://github.com/TheWisePigeon/hermes" class="">GitHub</a>
	</header>
	<header class="flex w-full justify-between p-2 border-b">
		<a href="/">Authentication</a>
		<a href="/apps">Applications</a>
		<a href="/profile">Profile</a>
	</header>
</div>
<div class="p-2">
	<slot />
</div>
