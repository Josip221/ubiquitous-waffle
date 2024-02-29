<script lang="ts">
	export let dir = {
		name: "Folder",
		size: 0,
		isDir: true,
		isFile: false,
		created: 0,
		modified: 0
	}

	const formatTime = (time: number) => {
		return new Date(time).toLocaleString()
	}
</script>

<style>
	.folder--item {
		color: black;
		font-size: 11px;
		background-color: white;
		padding: 5px;
		border-radius: 15px;
		width: 400px;
		display: flex;
		justify-content: flex-start;
		gap: 5px;
	}
</style>

<div class="folder--item">
	<span>{dir.name}</span>
	<span>{dir.size}</span>
	<span>{formatTime(dir.created)}</span>
	<span>{formatTime(dir.modified)}</span>

</div>

