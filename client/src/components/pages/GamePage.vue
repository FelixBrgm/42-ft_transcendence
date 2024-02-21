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
	
<style scoped>
	@import "./../functions/neonglow.css";

	
.body {
    font-family: neuropol;
    border-radius: 20px;
    margin-left: 20px;
    margin-right: 20px;
}
</style>
	