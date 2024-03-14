<template>
    <div class="friend-list">
      <button @click="$emit('close')" class="close-btn" aria-label="Close">
        <span aria-hidden="false">&times;</span> 
      </button>
      <div v-for="(friend, index) in friendInfos" :key="index" @click="$emit('join-chat', friend)" class="room-item rounded cursor-pointer">
        {{ friend.alias }}
      </div>
    </div>
  </template>
  
  <script>
  export default {
    props: {
      friendInfos: Array
    }
  };
  </script>
  
<style scoped>
.friend-list {
    text-align: right;
    width: 200px; /* Adjust width as needed */
    background-color: #343a40;
    color: white;
}
</style>
