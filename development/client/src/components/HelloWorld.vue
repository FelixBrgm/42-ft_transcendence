<template>
  <div></div>
</template>

<script>
export default {
  data() {
    return {
      socket: null,
    };
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
    },
    sendMessage(message) {
      if (this.socket && this.socket.readyState === WebSocket.OPEN) {
        this.socket.send(message);
      }
    },
    handleKeyPress(event) {
      if (event.key === "ArrowUp") {
        console.log("PRESSED u");
        this.sendMessage("u");
      } else if (event.key === "ArrowDown") {
        console.log("PRESSED d");
        this.sendMessage("d");
      }
    },
    handleKeyRelease(event) {
      if (event.key === "ArrowUp" || event.key === "ArrowDown") {
        console.log("PRESSED n");
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
