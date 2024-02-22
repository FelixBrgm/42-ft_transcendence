<template>
    <div class="game-info">
      <div class="players-container">
        <div class="profile left-profile">
          <img :src="fetchFriend(leftPlayer).avatar">
          <h2>{{ fetchFriend(leftPlayer).alias }}</h2>
          <p>Wins: {{ fetchFriend(leftPlayer).wins }} Losses: {{ fetchFriend(leftPlayer).losses }}</p>
        </div>
        <span> VS </span>
        <div class="profile right-profile">
            <img :src="fetchFriend(rightPlayer).avatar">
          <h2>{{ fetchFriend(rightPlayer).name }}</h2>
          <p>Wins: {{ fetchFriend(rightPlayer).wins }} Losses: {{ fetchFriend(rightPlayer).losses }}</p>
        </div>
      </div>
    </div>
  </template>


<script>
import axios from 'axios';

export default { 
  props: {
    userId: String, // Assuming userId is passed as a prop
    leftPlayer: Object, 
    rightPlayer: Object,
  },
  methods: {
    async fetchFriend(ID) {
      try {
        const response = await axios.get(`http://127.0.0.1:8080/user/${ID}`, { withCredentials: true });
        return (response.data);
      } catch (error) {
        console.error("Error fetching friend:", error);
      }
    }
  }
};
</script>

<style scoped>
/* Your CSS styles go here */
</style>