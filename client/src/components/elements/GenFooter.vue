<template>
	<div id="app">
		<!-- Your main content goes here -->
		
		<div class="app-footer">
			<a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ&t=0s" class="neon-text">hehe</a>
		</div>
	</div>
</template>

<style>

.app-footer { 
	display: flex;
	justify-content: flex-end;
	padding: 1rem; 
	font-family: neuropol; 
	background-color: #5c5e5f; 
	align-items: right; 
	border-radius: 20px;
	box-shadow: 0 0 10px 5px #00f0ff;
	animation: neonGlow 6s infinite;
	margin-top: 30px;
	margin-bottom: 30px;
	margin-left: 20px;
	margin-right: 20px;
	max-width: 1600px; /* Set the maximum width */
	min-width: 950px; /* Set the maximum width */
	margin: 0 auto; /* Center the footer */
}

.neon-text {
	color: white;
	text-shadow: 0 0 10px hsl(45, 100%, 60%), 0 0 20px hsl(45, 100%, 60%), 0 0 30px hsl(45, 100%, 60%);
	text-decoration: none;
	cursor: pointer;
}
</style>
