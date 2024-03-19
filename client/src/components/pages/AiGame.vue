<template>
	<div id="app">
		<GenHeader />
		<div id="Body">
			<AiPong/>
		</div>
		<GenFooter />
	</div>
	</template>
	
	<script>
	import GenHeader from "@/components/elements/GenHeader.vue";
	import GenFooter from "@/components/elements/GenFooter.vue";
	import AiPong from "@/components/elements/AiPong.vue"; 
	
	export default {
	components: {
		GenHeader,
		GenFooter, 
		AiPong,
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
	