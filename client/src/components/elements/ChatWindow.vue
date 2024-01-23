<template>
  <div v-show="showChat" class="chat-window">
    <div class="card">
      <div class="card-header">
        Chat Room
      </div>
      <main class="app-main">
        <div class="chat-container">
          <div class="chat-box">
            <div class="card-body" style="height: 300px; overflow-y: auto; padding: 10px;" id="chatBox">
              <div v-for="(message, index) in messages" :key="index" class="mb-2" :class="{ 'text-right': message.sender === 'User', 'text-left': message.sender === 'Bot' }">
                <strong>{{ message.sender }}:</strong> {{ message.text }}
              </div>
            </div>
            <div class="card-footer">
              <div class="input-container">
                <input type="text" v-model="newMessage" @keyup.enter="sendMessage" class="form-control" placeholder="Type your message...">
                <button @click="sendMessage" class="send-button" style="font-family: 'neuropol';">Send</button>
              </div>
            </div>
          </div>
        </div>
      </main>
    </div>
  </div>
</template>

<script>
export default {
	props: {
		showChat: Boolean,
	},
	data() {
		return {
			messages: [
				{ sender: 'User', text: 'Hello!' },
				{ sender: 'Bot', text: 'Hi there!' },
			],
			newMessage: '',
		};
	},
	methods: {
		sendMessage() {
			if (this.newMessage.trim() === '') return;
			this.messages.push({ sender: 'User', text: this.newMessage });
			// Simulate a bot response (you can replace this with actual backend communication)
			this.messages.push({ sender: 'Bot', text: 'I got your message!' });
			this.newMessage = '';
		},
		closeChat() {
			this.$emit('close-chat');
		},
	},
};
</script>

<style scoped>
/* Add your own styles for the chat window */
.chat-window {
	position: fixed;
	bottom: 20px;
	right: 100px;
	left: 80px;
	width: calc(100% - 160px); /* Adjust the calculation based on your preferences */
	margin: 10px;
	border-radius: 10px;
	background-color: #727475;
	color: white; 
	font-family: neuropol;
	box-shadow: 0 0 10px 5px #00f0ff;
	animation: neonGlow 6s infinite;
}

.card-header {
	padding-top: 10px;
	margin-left: 20px;
	flex: auto;
}

.text-right {
	text-align: right;
}

.text-left {
	text-align: left;
}


.card-footer {
	padding-bottom: 10px;
	padding-left: 10px;
}

.close-btn {
	background: none;
	border: none;
	color: #000;
	cursor: pointer;
	font-size: 18px;
}
.app-main {
	display: flex;
	flex-direction: column;
	align-items: stretch;
}

.chat-container {
	justify-content: center;
	align-items: center;
	padding: 20px;
}

.chat-box {
	border: 1px solid #ccc;
	padding: 20px;
	box-sizing: border-box;
	overflow-y: auto; /* Enable vertical scroll if the content exceeds the box height */
}

.input-container {
  display: flex;
  align-items: center;
  justify-content: flex-end; /* Align to the right */
  font-family: neuropol;
}

.form-control {
  flex: 1; /* Take up remaining space in the container */
  margin-right: 10px; /* Add some space between input and button */
  height: 40px; /* Adjust the height as needed */
  font-size: 14px; /* Adjust the font size as needed */
  font-family: neuropol;
}

.send-button {
  padding: 15px; /* Increase button padding */
  font-size: 14px; /* Increase button font size */
  margin-top: 0; /* Remove margin-top */
  margin-left: 0; /* Remove margin-left */
  font-family: neuropol;
}

.send-button:hover {
	color: black;
}

</style>
