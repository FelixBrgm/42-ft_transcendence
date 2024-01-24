<template>
	<div id="app">
		<GenHeader />
		<div id="Body">
		<PongGame/>
		</div>
		<GenFooter />
	</div>
</template>
	
<script>
	import GenHeader from "@/components/elements/GenHeader.vue";
	import GenFooter from "@/components/elements/GenFooter.vue";
	import PongGame from "@/components/elements/PongGame.vue";
	
	export default {
	components: {
		GenHeader,
		GenFooter,
		PongGame,
	},
	};
</script>
	
<style>
	@import "./../functions/neonglow.css";

	.body {
		font-family: neuropol;
		padding: 1rem;
		border-radius: 20px;
		margin-left: 20px;
		margin-right: 20px;
		flex: 1;
		
		background-color: #5c5e5f;
		box-shadow: 0 0 10px 5px #00f0ff;
		animation: neonGlow 6s infinite;
		color: white;
	}
</style>
