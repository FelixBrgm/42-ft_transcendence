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
