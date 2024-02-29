<script>
	import { invoke } from '@tauri-apps/api/tauri'
</script>


<style>
	.container {
		width: 100%;
		display: flex;
		justify-content: center;
		align-items: center;
		flex-direction: column;
		gap: 10px;
		
	}
	.heading {
		color: white;
	}
</style>

<div class="container">
	<h1 class="heading">Tauri app test</h1>
	<button on:click={() => invoke('test_function', {message: "hello from backend"})}>Invoke</button>
</div>
