<template>
  <div class="game-info">
    <div v-if="this.LP" class="profile1">
      <img :src="this.LP.avatar" class="rounded-tiny-circle">
      <div class="profile-info">
        <h2>{{ this.LP.alias }}</h2>
        <p>Wins: {{ this.LP.wins }} Losses: {{ this.LP.losses }}</p>
      </div>
    </div>
    <span> VS </span>
    <div v-if="this.RP" class="profile1">
      <img :src="this.RP.avatar" class="rounded-tiny-circle">
      <div class="profile-info">
        <h2>{{ this.RP.alias}}</h2>
        <p>Wins: {{ this.RP.wins }} Losses: {{ this.RP.losses }}</p>
      </div>
    </div>
  </div>
</template>

<script>
import axios from 'axios';

export default { 
  props: {
    leftPlayer: String, 
    rightPlayer: String,
  },
  data(){
    return{
      LP: null,
      RP: null
    };
  },
  created() {
    this.fetchFriends(); // Call fetchFriends method when the component is created
  },
  methods: {
    async fetchFriends() { // Corrected method name to fetchFriends
      try {
        const LPResponse = await axios.get(`http://127.0.0.1:8080/user/${this.leftPlayer}`, { withCredentials: true }); 
        const RPResponse = await axios.get(`http://127.0.0.1:8080/user/${this.rightPlayer}`, { withCredentials: true });
        this.LP =  LPResponse.data;
        
        this.RP =  RPResponse.data; 
      } catch (error) {
        console.error("Error fetching friend:", error);   
      }
    }
  }
};
</script>

<style>
.game-info{
  border-radius: 50px;
  padding: 40px; 
  margin: 10px; 
  box-shadow: 0 0 10px 0px #00f0ff;
  animation: neonGlow 6s infinite;
}
.rounded-tiny-circle {
  width: 75px;
  height: 75px;
  object-fit: cover;
  border-radius: 50%;
  box-shadow: 0 0 10px 0px #00f0ff;
  animation: neonGlow 6s infinite;
  cursor: pointer;
}
  .profile1 {
    display: flex;
    align-items: center;
  }

  .profile-info {
    margin-left: 10px; /* Adjust this value as needed */
  }

</style>