<template>
  <div v-show="showChat" class="chat-window">
    <div class="card">
      <div class="app-main">
        <div class="chat-sidebar">
          <div v-for="(room, index) in chatRooms" :key="index" @click="joinRoom(room.room_id)" class="room-item">
            {{ room.room_name }}
          </div>
        </div>
        <div class="chat-container">
          <div class="chat-box">
            <!-- Existing chat box content -->
            <div class="card-body" style="height: 300px; overflow-y: auto; padding: 10px;" id="chatBox">
              <div v-for="(message, index) in messages" :key="index" class="mb-2" :class="{ 'text-right': message.sender === 'User', 'text-left': message.sender === 'Bot' }">
                <strong>{{ message.sender }}:</strong> {{ message.text }}
              </div>
            </div>
            <!-- Existing chat input and send button -->
            <div class="card-footer">
              <div class="input-container">
                <input type="text" v-model="newMessage" @keyup.enter="sendMessage" class="form-control" placeholder="Type your message...">
                <button @click="sendMessage" class="send-button" style="font-family: 'neuropol';">Send</button>
              </div>
            </div>
          </div>
        </div>
      </div>
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
			chatRooms: [
			{ room_id: 1, room_name: 'Room 1' },
			{ room_id: 2, room_name: 'Room 2' },
			],
		};
	},
	methods: {
		sendMessage() {
			if (this.newMessage.trim() === '') return;
			this.messages.push({ sender: 'User', text: this.newMessage });
			this.messages.push({ sender: 'Bot', text: 'I got your message!' });
			this.newMessage = '';
		},
		closeChat() {
			this.$emit('close-chat');
		},
		joinRoom(roomId) {
		// Implement logic to join the selected room
		console.log('Joining room:', roomId);
		},
	},
};
</script>

<style scoped>
.chat-window {
  position: fixed;
  bottom: 20px;
  right: 100px;
  width: calc(50% - 20px);
  border-radius: 20px;
  background-color: #727475;
  color: white; 
  font-family: neuropol;
  box-shadow: 0 0 10px 5px #00f0ff;
  animation: neonGlow 6s infinite;
  z-index: 9999;
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


.chat-box { 
  padding: 20px;
  overflow-x: auto;
  border-radius: 20px;
  flex: 1; /* Make the chat-box take up all available space */
}

.form-control {
  flex: 1;
  margin-right: 10px; /* Add some space between input and button */
  height: 40px; /* Adjust the height as needed */
  font-size: 14px; /* Adjust the font size as needed */
  font-family: neuropol;
}

.send-button {
  border-radius: 20px;
  padding: 15px; 
  font-size: 14px;
  font-family: neuropol;
}

.send-button:hover {
	color: black;
}
.app-main {
  display: flex;
  align-items: stretch;
}

.chat-sidebar {
  width: 150px;
  padding: 20px;
  background-color: #333;
  color: white;
  overflow-y: auto;
  border-radius: 20px;
}

.room-item {
  cursor: pointer;
  padding: 10px;
  margin-bottom: 10px;
  transition: background-color 0.3s;
  border-radius: 20px; 
}

.room-item:hover {
  background-color: #555;
}

</style>
