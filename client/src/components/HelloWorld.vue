<template>
  <div>
    <ul>
      <li v-for="message in reversedMessages" :key="message">{{ message }}</li>
    </ul>
  </div>
</template>

<script>
export default {
  data() {
    return {
      socket: null,
      messages: [],
    };
  },
  computed: {
    reversedMessages() {
      return this.messages.slice().reverse();
    },
  },
  methods: {
    connectWebSocket() {
      this.socket = new WebSocket("ws://localhost:4242");

      // WebSocket event listeners
      this.socket.onopen = () => {
        console.log("WebSocket connected");
      };

      this.socket.onclose = () => {
        console.log("WebSocket disconnected");
      };

      this.socket.onerror = (error) => {
        console.error("WebSocket error:", error);
      };

      this.socket.onmessage = (event) => {
        const message = event.data;
        console.log(message);
        if (message.substring(0,3) == "POS") {
          const p1 = parseInt(message.substring(3, 8), 10);
          const p2 = parseInt(message.substring(8, 13), 10);
          const x = parseInt(message.substring(13, 18), 10);
          const y = parseInt(message.substring(18, 23), 10);
          console.log("P1: " + p1 + "| P2: " + p2 + " | Ball x: " + x + " | y: " + y);
        }
      };
    },
    sendMessage(message) {
      if (this.socket && this.socket.readyState === WebSocket.OPEN) {
        this.socket.send(message);
      }
    },
    handleKeyPress(event) {
      if (event.key === "ArrowUp") {
        console.log("UP");
        this.sendMessage("u");
      } else if (event.key === "ArrowDown") {
        this.sendMessage("d");
      }
    },
    handleKeyRelease(event) {
      if (event.key === "ArrowUp" || event.key === "ArrowDown") {
        this.sendMessage("n");
      }
    },
  },
  mounted() {
    this.connectWebSocket();
    document.addEventListener("keydown", this.handleKeyPress);
    document.addEventListener("keyup", this.handleKeyRelease);
  },
  beforeUnmount() {
    document.removeEventListener("keydown", this.handleKeyPress);
    document.removeEventListener("keyup", this.handleKeyRelease);
  },
};
</script>
