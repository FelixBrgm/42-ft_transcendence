<template>
  <div id="app">
    <ChatButton v-if="showChatButton" :showChat="showChat" @toggle-chat="toggleChat" />
    <ChatWindow :showChat="showChat" @close-chat="closeChat" />
    <router-view />
    <button @click="fetchData">TESTING</button>
  </div>
</template>

<script>
import axios from 'axios';
import ChatButton from './components/elements/ChatButton.vue';
import ChatWindow from './components/elements/ChatWindow.vue';
import { auth } from './router'; // Import auth object from router.js

export default {
  components: {
    ChatButton,
    ChatWindow,
  },
  data() {
    return {
      showChat: false,
    };
  },
  computed: {
    showChatButton() {
      return auth.logged_in;
    },
  },
  methods: {
    fetchData() {
      console.log("HELLO");
      // Make a GET request
      axios.get('http://127.0.0.1:8080/auth/check')
        .then(response => {
          // Handle successful response
          this.responseData = response.data;
          console.log(this.responseData);
        })
        .catch(error => {
          // Handle error
          this.error = error.message;
        });
    },
    toggleChat() {
      this.showChat = !this.showChat;
    },
  },
};
</script>

<style>
#Body {
  min-height: 76vh;
  display: flex;
  flex-direction: column;
  margin-bottom: 30px;
}

@font-face {
  font-family: "neuropol";
  src: url('assets/fonts/neuropol.ttf');
}
</style>