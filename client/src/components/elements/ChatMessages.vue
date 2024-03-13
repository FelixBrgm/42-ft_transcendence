<!-- ChatMessages.vue -->
<template>
    <div class="chat-box">
      <div v-for="(message, index) in messages" :key="index" class="message" :class="{ 'sent': message.sender === 'User', 'received': message.sender === 'Bot' }">
        <strong>{{ message.sender_id == myId.id ? myId.alias : foundFriend.alias }}:</strong> {{ message.message }}
      </div>
    </div>
  </template>
  
  <script>
  export default {
    props: {
      messages: Array,
      myId: Object,
      foundFriend: Object
    }
  };
  </script>

<style scoped> 
  .chat-box {
    height: 300px;
    overflow-y: auto;
    padding: 10px;
  }
</style>