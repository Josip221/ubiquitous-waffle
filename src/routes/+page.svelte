<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri'

	interface Dir {
		name: string
		size: number
		isDir: boolean
		isFile: boolean
		created: number
		modified: number
	}

	let dir_list: Dir[] = []

	const handleClick = () => {
		invoke('get_current_dir', {message: "hello from backend"}).then((res: any) => {
			dir_list = [...dir_list, ...res]
		})
	}

	$: console.log(dir_list)
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

	.folder--item {
		color: white;
	}
</style>

<div class="container">
	<h1 class="heading">Tauri File Explorer</h1>
	<button on:click={handleClick}>Invoke</button>
	{#each dir_list as dir}
		<div class="folder--item">
			<span>{dir.name}</span>
			<span>{dir.size}</span>
		</div>
	{/each}
</div>
