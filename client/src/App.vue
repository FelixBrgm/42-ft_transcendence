<template>
  <div id="app">
    <ChatButton v-if="showChatButton" :showChat="showChat" @toggle-chat="toggleChat" />
    <ChatWindow :showChat="showChat" @close-chat="toggleChat" />
    <router-view />
  </div>
</template>

<script>
import store from './store';
import ChatButton from './components/elements/ChatButton.vue';
import ChatWindow from './components/elements/ChatWindow.vue';

export default {
  components: {
    ChatButton,
    ChatWindow,
  },
  computed: {
    showChatButton() {
      return !!store.state.auth.user;
    },
  },
  data() {
    return {
      showChat: false,
    };
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
