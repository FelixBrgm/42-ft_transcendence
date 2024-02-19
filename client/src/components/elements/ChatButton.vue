<template>
  <div @click="toggleChat" style="position: fixed; right: 150px; bottom: 120px;">
    <div class="chat-button">
      <img src="@/assets/comment.png" alt="Comment" class="comment-icon" />
    </div>
  </div>
</template>

<script>
export default {
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

.chat-button {
  background-color: #00f0ff; /* Change this to the desired color */
  box-shadow: 0 0 10px 5px #00f0ff;
  animation: neonGlow 6s infinite;
  border-radius: 50%;
  width: 50px;
  height: 50px;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
  position: relative;
  margin-left: auto; 
}

/* Add your custom styles here */
</style>  
