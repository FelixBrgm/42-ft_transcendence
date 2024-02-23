  <template>
    <div v-show="showChat" class="chat-window">
      <div class="chat-container-wrapper">
      <div class="card">
        <div class="card-header">
          <div v-if="foundFriend" style="cursor: pointer;" @click="goToFriend(foundFriend.id)"><img :src="foundFriend.avatar" class="rounded-tinier-circle"> {{ foundFriend.alias }} </div> 
          <button @click="closeChat" class="close-btn" aria-label="Close">
            <span aria-hidden="false">&times;</span> 
          </button>
        </div>
        <div class="app-main">
        </div>
        <div class="chat-container">
          <div v-if="chatReload" class="chat-box">
            <div v-for="(message, index) in messages" :key="index" class="message" :class="{ 'sent': message.sender === 'User', 'received': message.sender === 'Bot' }">
              <strong>
                {{ message.sender_id == myId.id ? myId.alias : foundFriend.alias }}: </strong> {{ message.message }}
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
      <div v-if="chatReload" class="friend-list">
        <div v-for="(friend, index) in friendInfos" :key="index" @click="joinFriendChat(friend)" class="room-item p-2 mb-2 rounded cursor-pointer">
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
    props: {
      showChat: Boolean,
    },
      watch: {
      showChat() {
          this.fetchFriends();
          this.messages = [];
          this.friendid = null; 
      },
      $route() {
        this.setupWebSocketAndFetchFriends(); 
      },
    },
    computed: {
      foundFriend() {
        return this.friendInfos.find(friend => friend.id === this.friendid);
      },
      myId() {
        return store.state.auth.user ;
      }
    }, 
    data() {
      return {
        friendInfos: [],
        messages: [],
        newMessage: '',
        friends: [],
        friendid: null, 
        roomid: null,
        chatReload: true, 
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
          this.ws.onopen = this.handleOpen;
          this.ws.onclose = this.handleClose;
          this.ws.onmessage = this.handleMessage;
          this.ws.onerror = this.handleError;
          await this.fetchFriends();
        }
      },
      handleError(event) {  
        console.log('ERROR:', event.data); // Logging the incoming message
      }, 
      async sendMessage() {
        if (axios.get(`http://127.0.0.1:8080/block/check/${this.friendid}`,{ withCredentials: true }) == true)
        {
          alert("This user is blocked or has blocked you");
        }
        else (this.ws && this.newMessage.trim() !== '' && (this.roomid != undefined)) 
        {
          this.ws.send(this.friendid + ":" + this.newMessage);
          this.updateChat();
        }
        this.newMessage = '';
      },
      closeChat() {
        this.$emit('close-chat');
      },
      goToFriend(id) {
        this.$router.push({ path: '/profile', query: { uid: id } });  
      },
      async updateChat() {
        this.chatReload = !this.chatReload;
        this.chatReload = !this.chatReload;
        if (this.roomid == null)
          return; 
        try {
          const response = await axios.get(`http://127.0.0.1:8080/messages/${this.roomid}`, { withCredentials: true });
          this.messages = response.data;
        } catch (error) { 
          console.error('Error fetching messages:', error);
        } 
      }, 
      async fetchFriends() {
        try {
          const response = await axios.get(`http://127.0.0.1:8080/friend/list/${store.state.auth.user.id}`, { withCredentials: true });
          this.friends = response.data;  
          this.friendInfos = []; // Clear friendInfos array
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
      async joinFriendChat(friend) {
        try {
          const response = await axios.get(`http://127.0.0.1:8080/chat/${friend.id}`, { withCredentials: true });
          this.friendid = friend.id;
          this.roomid = response.data;  
        } catch (error) {
          alert(error.response.data);
        } 
        this.updateChat(); 
      },
      handleMessage(event) {
        console.log('Incoming message:', event.data); // Logging the incoming message
      }
    }, 
  };
  </script>

  <style scoped>
  .chat-window {
    flex: 1; /* Take remaining space */
    border-radius: 20px;
    background-color: #727475;
    color: white;
    font-family: neuropol;
    box-shadow: 0 0 10px 5px #00f0ff;
    animation: neonGlow 6s infinite;
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .close-btn {
    background: none;
    border: none;
    color: rgb(3, 3, 3);
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
