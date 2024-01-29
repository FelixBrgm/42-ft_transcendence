<template>
	<div id="app">
		<GenHeader />
		<div id="Body">
		<div class="body">
			<h1>TEST</h1>
		</div>
		</div>
		<GenFooter />
	</div>
	</template>
	
	<script>
	import GenHeader from "@/components/elements/GenHeader.vue";
	import GenFooter from "@/components/elements/GenFooter.vue";
	
	export default {
	components: {
		GenHeader,
		GenFooter,
	},
	};
	</script>
	
	<style scoped>
	@import "./../functions/neonglow.css";

	
.body {
    font-family: neuropol;
    padding: 1rem;
    border-radius: 20px;
    margin-left: 20px;
    margin-right: 20px;
    display: flex;
    background-color: #5c5e5f;
    box-shadow: 0 0 10px 5px #00f0ff;
    animation: neonGlow 6s infinite;
    max-width: 1600px; /* Set the maximum width */
    min-width: 900px; /* Set the maximum width */
    margin: 0 auto; /* Center the element */
    width: 100%; /* Take up full width */
    box-sizing: border-box; /* Include padding and border in the element's total width */
}
	</style>
	