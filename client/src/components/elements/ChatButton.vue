<template>
  <div @click="toggleChat" class="fixed-bottom" style="right: 28px; bottom: 120px;">
    <div v-if="!showChat" class="chat-button btn btn-primary">
      <img src="@/assets/comment.png" alt="Comment" class="comment-icon" />
    </div>
    <div v-else class="chat-button btn btn-danger">
      <img src="@/assets/cross.png" alt="Cross" class="cross-icon" />
    </div>
  </div>
</template>

<script>
export default {
  props: {
    showChat: Boolean,
  },
  methods: {
    toggleChat() {
      this.$emit('toggle-chat');
    },
  },
};
</script>

<style scoped>
.comment-icon {
  width: 30px;
  height: 30px;
}

.cross-icon {
  width: 25px;
  height: 25px;
}

.chat-button {
  box-shadow: 0 0 10px 5px #00f0ff;
  animation: neonGlow 6s infinite;
  border-radius: 50%;
  width: 50px;
  height: 50px;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
}

/* Add your custom styles here */
</style>
