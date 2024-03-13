<template></template>
<div class="input-group">
    <input type="text" id="messageField" class="form-control" placeholder="Type your message..." v-model="newMessage">
    <button @click="sendMessage" class="send-button">Send</button>
</div>
</template>

<script>
export default {
  data() {
    return {
      newMessage: '' // Initialize newMessage as empty string
    };
  },
  methods: {
    sendMessage() {
      if (this.newMessage.trim() !== '') {
        this.$emit('send', this.newMessage); // Emit 'send' event with newMessage as payload
        this.newMessage = ''; // Clear newMessage after sending
      }
    }
  }
};
</script>

<style scoped>

    .input-group {
    display: flex;
    justify-content: space-between;
  }
</style>