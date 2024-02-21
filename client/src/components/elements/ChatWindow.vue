<template>
  <div v-show="showChat" class="position-fixed bottom-0 end-0" style="width: calc(50% - 20px); border-radius: 20px; background-color: #727475; color: white; font-family: neuropol; box-shadow: 0 0 10px 5px #00f0ff; animation: neonGlow 6s infinite; z-index: 9999;">
    <div class="card">
      <div class="card-header">
        Chat Window
        <button @click="closeChat" class="close close-btn" aria-label="Close">
          <span aria-hidden="true">&times;</span>
        </button>
      </div>
      <div class="app-main">
        <div class="chat-sidebar bg-dark text-white">
          <div v-for="(room, index) in chatRooms" :key="index" @click="joinRoom(room.room_id)" class="room-item p-2 mb-2 rounded cursor-pointer">
            {{ room.room_name }}
          </div>
        </div>
        <div class="chat-container">
          <div class="chat-box card-body" style="height: 300px; overflow-y: auto; padding: 10px;">
            <div v-for="(message, index) in messages" :key="index" class="mb-2" :class="{ 'text-right': message.sender === 'User', 'text-left': message.sender === 'Bot' }">
              <strong>{{ message.sender }}:</strong> {{ message.text }}
            </div>
          </div>
          <div class="card-footer">
            <div class="input-group">
              <input type="text" v-model="newMessage" @keyup.enter="sendMessage" class="form-control" placeholder="Type your message...">
              <button @click="sendMessage" class="btn btn-primary send-button">Send</button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import store from '../../store';
import axios from 'axios';

export default {
  props: {
    showChat: Boolean,
  },
  data() {
    return {
      messages: [], // Initialize messages as an empty array
      newMessage: '',
      chatRooms: [],
      ws: null, // WebSocket connection instance
    };
  },
created() {
    this.setupWebSocketAndFetchRooms(); // Initial setup

    // Retry every 5 seconds if user data is not available
    this.retryInterval = setInterval(() => {
        if (store.state.auth.user && store.state.auth.user.id) {
            clearInterval(this.retryInterval); // Clear retry interval if user data is available
            this.setupWebSocketAndFetchRooms(); // Setup WebSocket and fetch rooms
        }
    }, 0);
}, 
destroyed() {
    clearInterval(this.retryInterval); // Clear retry interval on component destruction
},
    methods: {
    setupWebSocketAndFetchRooms() {
        const user = store.state.auth.user;
        if (user && user.id && !this.ws) {
            const userId = user.id;
            const token = user.password;
            const websocketUrl = `ws://localhost:8080/ws?id=${userId}&token=${token}`;
            this.ws = new WebSocket(websocketUrl); 

            // Set up WebSocket event listeners  
            this.ws.onopen = this.handleOpen;
            this.ws.onclose = this.handleClose;
            this.ws.onmessage = this.handleMessage;
            this.ws.onerror = this.handleError;

            // Fetch chat rooms
            this.fetchChatRooms(); 
        }
    }, 
    handleOpen() {
    },
    sendMessage() {
      if (this.newMessage.trim() === '') return;
    },
    closeChat() {
      this.$emit('close-chat');
    },
    joinRoom(roomId) {
      // Implement logic to join the selected room
      console.log('Joining room:', roomId);
    },
    fetchChatRooms() {
        axios.get('http://localhost:8080/rooms', { withCredentials: true })
            .then(response => {
                // Handle successful response here
                console.log(response.data); // For example, log the response data
            })
            .catch(error => {
                // Handle error here 
                console.error('Error fetching chat rooms:', error);
            });
    }
    // handleMessage(event) {
    // }
  },
};
</script>

<style scoped>
.text-right {
  text-align: right;
}

.text-left {
  text-align: left;
}

.room-item:hover {
  background-color: #555;
}
</style>
