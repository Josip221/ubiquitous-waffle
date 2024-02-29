<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri'
	import FolderItem from '../components/FolderItem.svelte';
	import { onMount } from 'svelte';


	interface Dir {
		name: string
		size: number
		isDir: boolean
		isFile: boolean
		created: number
		modified: number
	}

	let dir_list: Dir[] = []

	const handleMount = () => {
		invoke('get_current_dir', {message: "hello from backend"}).then((res: any) => {
			dir_list = [...res]
		})
	}

	onMount(handleMount);
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

	.folder--container {
		display: flex;
		flex-direction: column;
		gap: 10px;
		background-color: white;
	}

</style>

<div class="container">
	<h1 class="heading">Tauri File Explorer</h1>
	<div class="folder--container">
		{#each dir_list as dir}
		<FolderItem {dir} />
		{/each}
	</div>
	
</div>
