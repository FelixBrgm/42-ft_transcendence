<!-- ChatButton.vue -->

<template>
	<div v-if="!showChat" class="chat-button" @click="toggleChat">
		<img src="@/assets/comment.png" alt="Logo" class="comment"/>
	</div>
	<div v-else class="chat-button" @click="toggleChat">
		<img src="@/assets/cross.png" alt="Logo" class="cross"/>
	</div>
	</template>
	
<script>
	export default {
	props: {
		showChat: Boolean,
	},
	methods: {
		toggleChat() {
		this.$emit('toggle-chat');
		}
	},
	};
</script>
	
<style scoped>

	.cross{
		width: 25px;
		height: 25px;
		display: center;
	}

	.comment{
		width: 30px;
		height: 30px;
		display: center;
	}
	.chat-button {
		position: fixed;
		bottom: 120px;
		right: 28px;
		background-color: #5c5e5f;
		color: #fff;
		box-shadow: 0 0 10px 5px #00f0ff;
		animation: neonGlow 6s infinite;
		border: none;
		border-radius: 30px;
		width: 50px;
		height: 50px;
		display: flex;
		justify-content: center;
		align-items: center;
		cursor: pointer;
	}
	</style>
	