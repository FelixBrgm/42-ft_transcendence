<template>
	<div id="app">
		<GenHeader />
		<div class="body">
			<UserProfile />
		</div>
		<GenFooter />
	</div>
	</template>
	
	<script>
	import GenHeader from "@/components/elements/GenHeader.vue";
	import GenFooter from "@/components/elements/GenFooter.vue";
	import UserProfile from "@/components/elements/UserProfile.vue";
	
	export default {
	components: {
		GenHeader,
		GenFooter,
		UserProfile,
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
		max-width: 1600px;
		margin: 0 auto;
	}
	</style>
	