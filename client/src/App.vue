<template>
  <div id="app">
    <ChatButton v-if="showChatButton" :showChat="showChat" @toggle-chat="toggleChat" />
    <ChatWindow :showChat="showChat" @close-chat="closeChat" />
    <router-view/>
  </div>
</template>

<script>
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