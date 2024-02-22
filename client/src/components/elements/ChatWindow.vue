<template>
  <div v-show="showChat" class="chat-window">
    <div class="card">
      <div class="card-header">
        Chat Window
        <button @click="closeChat" class="close-btn" aria-label="Close">
          <span aria-hidden="true">&times;</span>
        </button>
      </div>
      <div class="app-main">
        <div class="chat-sidebar">
          <div v-for="(friend, index) in friendInfos" :key="index" @click="joinFriendChat(friend.id)" class="room-item">
            {{ friend.alias }}
          </div>
        </div>
        <div class="chat-container">
          <div class="chat-box">
            <div v-for="(message, index) in messages" :key="index" class="message" :class="{ 'sent': message.sender === 'User', 'received': message.sender === 'Bot' }">
              <strong>{{ message.sender }}:</strong> {{ message.text }}
            </div>
          </div>
          <div class="card-footer">
            <div class="input-group">
              <input type="text" v-model="newMessage" @keyup.enter="sendMessage" class="form-control" placeholder="Type your message...">
              <button @click="sendMessage" class="send-button">Send</button>
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
      friendInfos: [],
      messages: [],
      newMessage: '',
      friends: [],
      ws: null,
    };
  },
  created() {
    axios.interceptors.request.use(config => {
      this.setupWebSocketAndFetchFriends();
      return config; 
    });

    this.retryInterval = setInterval(() => {
      if (store.state.auth.user && store.state.auth.user.id) {
        clearInterval(this.retryInterval);
        this.setupWebSocketAndFetchFriends();
      }
    }, 5000); 
  },
  methods: {
    async setupWebSocketAndFetchFriends() {
      const user = store.state.auth.user;
      if (user && user.id && !this.ws) {
        const userId = user.id;
        const token = user.password;
        const websocketUrl = `ws://localhost:8080/ws?id=${userId}&token=${token}`;
        this.ws = new WebSocket(websocketUrl);

        if (this.roomid !== undefined) {
          this.ws.onopen = this.handleOpen;
          this.ws.onclose = this.handleClose;
          this.ws.onmessage = this.handleMessage;
          this.ws.onerror = this.handleError;
        }

        await this.fetchFriends();
      }
    },
    handleOpen() { 
      this.updateChat(this.roomid);
    },
    async sendMessage() {
      if (this.ws && this.newMessage.trim() !== '') {
        this.ws.send(this.roomid + ":" + this.newMessage);
        this.newMessage = '';
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
        const response = await axios.get(`http://127.0.0.1:8080/friend/list/${store.state.auth.user.id}`, { withCredentials: true });
        this.friends = response.data; 
        this.friendInfos = [];
        for (const friend of this.friends) {
          const userId = friend.user1 === this.$route.query.uid ? friend.user1 : friend.user2;
          try {
            const response = await axios.get(`http://127.0.0.1:8080/user/${userId}`, { withCredentials: true });
            this.friendInfos.push(response.data);
          } catch (error) {
            console.error('Error fetching user info:', error);
          }
        }
      } catch (error) {
        console.error('Error fetching friends:', error);
      }
    },
    async joinFriendChat(friendId, roomid) {
      this.roomid = roomid;
      this.updateChat(roomid); 
      console.log('Joining chat with friend:', friendId); 
    },
    handleMessage() {
      this.updateChat(this.roomid);
    }
  },
};
</script>

<style scoped>
.chat-window {
  position: fixed;
  bottom: 0;
  right: 0;
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
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.close-btn {
  background: none;
  border: none;
  color: white;
}

.chat-sidebar {
  background-color: #343a40;
  color: white;
}

.room-item {
  padding: 10px;
  margin-bottom: 10px;
  cursor: pointer;
}

.room-item:hover {
  background-color: #555;
}

.chat-box {
  height: 300px;
  overflow-y: auto;
  padding: 10px;
}

.message {
  margin-bottom: 10px;
}

.sent {
  text-align: right;
}

.received {
  text-align: left;
}

.card-footer {
  margin-top: 10px;
}

.input-group {
  display: flex;
  justify-content: space-between;
}

.form-control {
  flex: 1;
  margin-right: 10px;
}

.send-button {
  flex: 0 0 auto;
}
</style>
