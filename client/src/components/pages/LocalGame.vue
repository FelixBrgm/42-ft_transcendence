<template>
	<div id="app">
		<GenHeader />
		<div id="Body">
			<LocalPong/>
		</div>
		<GenFooter />
	</div>
	</template>
	
	<script>
	import GenHeader from "@/components/elements/GenHeader.vue";
	import GenFooter from "@/components/elements/GenFooter.vue";
	import LocalPong from "@/components/elements/LocalPong.vue"; 
	
	export default {
	components: {
		GenHeader,
		GenFooter, 
		LocalPong,
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
	