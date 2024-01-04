<template>
  <div>
    <h1>Testing the WebSocket</h1>
    <div v-if="socket.isConnected">
      <input v-model="message" placeholder="Type a message" />
      <button @click="sendMessage">Send Message</button>
    </div>
    <div v-if="socket.receivedMessages.length > 0">
      <h2>Received Messages:</h2>
      <ul>
        <li v-for="(msg, index) in socket.receivedMessages" :key="index">{{ msg }}</li>
      </ul>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      socket: {
        isConnected: false,
        message: '',
        receivedMessages: [],
      },
    };
  },
  methods: {
    sendMessage() {
      if (this.socket.isConnected) {
        this.$socket.sendObj({ type: 'chat', text: this.socket.message });
        this.socket.message = '';
      }
    },
  },
  mounted() {
    // Access WebSocket instance through this.$socket
    if (this.$socket) {
      // Handle incoming messages from the server
      this.$socket.onmessage = (event) => {
        const data = JSON.parse(event.data);
        if (data.type === 'chat') {
          this.socket.receivedMessages.push(data.text);
        }
      };

      // Handle connection opening
      this.$socket.onopen = () => {
        console.log('WebSocket connected');
        this.socket.isConnected = true;
      };

      // Handle connection closure
      this.$socket.onclose = (event) => {
        console.log('WebSocket closed:', event);
        this.socket.isConnected = false;
      };
    } else {
      console.error('$socket is undefined. WebSocket configuration may be incorrect.');
    }
  },
};
</script>
