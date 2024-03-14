<template>
  <div v-show="chatOpen" class="chat-window"> 
    <div class="chat-container-wrapper">
    <div class="card">
      <div class="card-header">
        <div v-if="foundFriend" style="cursor: pointer;" @click="goToFriend(foundFriend.id)"><img :src="foundFriend.avatar" class="rounded-tinier-circle"> {{ foundFriend.alias }} </div> 
          <div v-if="foundFriend" >           
            <img
          class="icon"
          alt="vsgame"
          src="@/assets/vs.png"
          @click="vsgame" 
          /> 
        </div>
        </div>
      <div class="app-main">
      </div>
      <div class="chat-container">
        <div class="chat-box">
          <div v-for="(message, index) in messages" :key="index" class="message" :class="{ 'sent': message.sender === 'User', 'received': message.sender === 'Bot' }">
            <strong>
              {{ message.sender_id == myId.id ? myId.alias : foundFriend.alias }}: </strong> {{ message.message }}
          </div>
        </div>
        <div class="card-footer">
          <div class="input-group">
            <input type="text" id="messageField" class="form-control" placeholder="Type your message..." v-model="newMessage">
            <button @click="sendMessage" class="send-button">Send</button>
          </div>
        </div>
      </div>
    </div>
    <div class="friend-list">
        <button @click="this.$store.dispatch('chat/toggleChat')" class="close-btn" aria-label="Close">
          <span aria-hidden="false">&times;</span> 
        </button>
      <div v-for="(friend, index) in friendInfos" :key="index" @click="joinFriendChat(friend)" class="room-item rounded cursor-pointer">
        {{ friend.alias }}
      </div>
    </div>
  </div> 
  </div>
</template>

<script>
import store from '../../store';
import axios from 'axios';

export default {
  $route() {
    this.setupWebSocketAndFetchFriends(); 
  },
  computed: {
    foundFriend() {
      return this.friendInfos.find(friend => friend.id === this.friendid);
    },
    myId() {
      return store.state.auth.user ;
    },
    chatOpen() {
      return this.$store.state.chat.chatOpen;
    },
  }, 
  data() {
    return {
      friendInfos: [],
      messages: [],
      newMessage: '',
      friends: [],
      friendid: null, 
      roomid: null,
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
    clearInterval(this.getFriendsInterval);
    this.getFriendsInterval = setInterval(() => {
      this.fetchFriends();
      this.updateChat();
    }, 3000); 
  },
  methods: {
    vsgame()
    {
      this.$router.push({ path: "/pong", query: { joinvs: this.friendid } });
    },
    async setupWebSocketAndFetchFriends() {
      const user = store.state.auth.user;
      if (user && user.id && !this.ws) {
        const userId = user.id;
        const token = user.token;
        const websocketUrl = `wss://${process.env.VUE_APP_IP}/api/ws?id=${userId}&token=${token}`;
        this.ws = new WebSocket(websocketUrl);
        this.ws.onopen = this.handleOpen;
        this.ws.onclose = this.handleClose;
        this.ws.onmessage = this.handleMessage;
        this.ws.onerror = this.handleError;
        await this.fetchFriends();
      }
    },
    async sendMessage() {
      if (this.friendid)
      {
      if (axios.get(`/api/block/check/${this.friendid}`,{ withCredentials: true }) == true)
        {
          alert("This user is blocked or has blocked you");
        }
        else (this.ws && this.newMessage.trim() !== '' && (this.roomid != undefined)) 
        {
          this.ws.send(this.friendid + ":" + this.newMessage);
        }
      }
      this.newMessage = '';
    },
    goToFriend(id) {
      this.$router.push({ path: '/profile', query: { uid: id } });  
    },
    async updateChat() {
      if (this.roomid == null)
        return; 
        try {
        const response = await axios.get(`/api/messages/${this.roomid}`, { withCredentials: true });
        if (JSON.stringify(this.messages) !== JSON.stringify(response.data)) {
            this.messages = response.data;
          }
        } catch (error) { 
        console.error('Error fetching messages:', error);
        } 
    }, 
    async fetchFriends() {
      try {
        const response = await axios.get(`/api/friend/list/${store.state.auth.user.id}`, { withCredentials: true });
        if(this.friends.length === 0 || JSON.stringify(this.friends) !== JSON.stringify(response.data))
        {
          this.friends = response.data;  
          this.friendInfos = []; // Clear friendInfos array
          for (const friend of this.friends) {
            const userId = friend.user1 !== store.state.auth.user.id ? friend.user1 : friend.user2;
            try {
              const response1 = await axios.get(`/api/user/${userId}`, { withCredentials: true });
              this.friendInfos.push(response1.data);
            } catch (error) {
              console.error('Error fetching user info:', error); 
            }
          } 
        }
      } catch (error) {
        console.error('Error fetching friends:', error); 
      } 
    },
    async joinFriendChat(friend) {
      try {
        const response = await axios.get(`/api/chat/${friend.id}`, { withCredentials: true });
        this.friendid = friend.id;
        this.roomid = response.data;  
      } catch (error) {
        alert(error.response.data);
      } 
    },
  }, 
};
</script>

<style scoped>
.chat-window {
  background-color: #727475;
  color: white;
  font-family: neuropol;
  z-index: 999;
}

.card-header {
display: flex;
justify-content: space-between;
align-items: center;
}


.card-header .item-wrapper {
display: flex;
flex-direction: column;
}

.close-btn {
  font-size: 24px; 
  padding-right: 10px;
  background: none;
  border: none;
  color: rgb(255, 255, 255);
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

.friend-list {
  text-align: right;
  width: 200px; /* Adjust width as needed */
  background-color: #343a40;
  color: white;
}

.chat-container-wrapper {
  display: flex;
  justify-content: flex-end; /* Align items to the right */
  position: fixed; 
  bottom: 0;
  right: 0;
  width: calc(50% - 20px); /* Adjust width as needed */
}

.rounded-tinier-circle {
width: 50px;
height: 50px;
object-fit: cover;
border-radius: 50%;
box-shadow: 0 0 10px 0px #00f0ff;
animation: neonGlow 6s infinite;
cursor: pointer;
}
</style>
