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
          <div v-for="(friend, index) in friends" :key="index" @click="joinFriendChat(friend.id)" class="room-item p-2 mb-2 rounded cursor-pointer">
            {{ friend.name }}
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
      friends: [], // Initialize friends as an empty array
      ws: null, // WebSocket connection instance
    };
  },
  created() {
    this.setupWebSocketAndFetchFriends(); // Initial setup

    // Retry every 5 seconds if user data is not available
    this.retryInterval = setInterval(() => {
        if (store.state.auth.user && store.state.auth.user.id) {
            clearInterval(this.retryInterval); // Clear retry interval if user data is available
            this.setupWebSocketAndFetchFriends(); // Setup WebSocket and fetch friends
        }
    }, 5000);
  },
  destroyed() {
    clearInterval(this.retryInterval); // Clear retry interval on component destruction
  },
  methods: {
    async setupWebSocketAndFetchFriends() {
        const user = store.state.auth.user;
        if (user && user.id && !this.ws) {
            const userId = user.id;
            const token = user.password;
            const websocketUrl = `ws://localhost:8080/ws?id=${userId}&token=${token}`;
            this.ws = new WebSocket(websocketUrl); 

            // Set up WebSocket event listeners 
            if(this.roomid != undefined)
            {
              this.ws.onopen = this.handleOpen;
              this.ws.onclose = this.handleClose;
              this.ws.onmessage = this.handleMessage;
              this.ws.onerror = this.handleError;
            }

            // Fetch friends
            await this.fetchFriends(); 
        }
    },
    handleOpen() {
      this.updateChat(this.roomid);
      // Handle WebSocket connection open
    },
    async sendMessage() {
      if (this.ws && this.newMessage.trim() !== '') {
        this.ws.send(this.roomid + ":" + this.newMessage);
        this.newMessage = ''; // Reset newMessage after sending
      }
    },
    closeChat() {
      this.$emit('close-chat');
    },
    async updateChat(roomid) {
      try {
        const response = await axios.get(`http://127.0.0.1:8080/messages/${roomid}`, { withCredentials: true });
        this.messages = response.data;
      } catch (error) {
        console.error('Error fetching messages:', error); 
      }
    },
    async fetchFriends() {
      try {
        const response = await axios.get(`http://127.0.0.1:8080/friend/list/${this.$route.query.uid}`, { withCredentials: true });
        this.friends = response.data;
      } catch (error) {
        console.error('Error fetching friends:', error);
      }
    },
    async joinFriendChat(friendId, roomid) {
      this.roomid = roomid;
      this.updateChat(roomid); 
      console.log('Joining chat with friend:', friendId);
    },
    handleMessage()
    {
      this.updateChat(this.roomid);
    }
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
