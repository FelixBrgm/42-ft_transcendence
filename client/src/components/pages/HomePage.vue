<template>
  <div id="app">
    <GenHeader />
    <div id="Body">
      <div class="body">
        <div class="text-center">
          Here are some options:
          <div class="my-container" @click="playGame">
            <span>Single player</span>
          </div>
          <div class="my-container">
            <div> Create Tournament </div>
            <span> Number of players: </span>
            <select v-model="selectedNumberOfPlayers">
              <option v-for="number in numbers" :key="number" :value="number">
                {{ number }}
              </option>
            </select>
          <div class="mybutton" @click="playTournament" >
            <span > Start Tournament</span>
          </div>
            
          </div>
        </div>
      </div>
    </div>
    <GenFooter />
  </div>
</template>

<script>
import GenHeader from "@/components/elements/GenHeader.vue";
import GenFooter from "@/components/elements/GenFooter.vue";
import axios from "axios";

export default {
  components: {
    GenHeader,
    GenFooter,
  },
  data() {
    return {
      selectedNumberOfPlayers: null,
      numbers: Array.from({ length: 64 }, (_, index) => (index + 1) * 2), // Example numbers for the dropdown
    };
  },
  methods: {
    playGame() {
      this.$router.push({ path: "/pong", query: { startGame: true } }); // Pass query parameter
    },
    playTournament() {
      if (this.selectedNumberOfPlayers) {
        this.$router.push({ path: "/pong", query: { startTournament: this.selectedNumberOfPlayers } });
      } else {
        alert("Please select the number of players for the tournament.");
      }
    },
  },
};
</script>
 
<style>
@import "./../functions/neonglow.css";

.body {
  font-family: neuropol;
  padding: 1rem;
  background-color: #5c5e5f;
  box-shadow: 0 0 10px 5px #00f0ff;
  animation: neonGlow 6s infinite;
  max-width: 1600px;
  min-width: 900px;
  width: 100%;
}
.mybutton {
  background-color: #7a7d7e;
  color: #ffffff;
  width: 300px;
  height: 30px; 
  padding: 10px;
  border-radius: 15px; /* Adjust the value to control the roundness */
  display: flex;
  justify-content: center;
  align-items: center;
  margin: 0 auto; /* Center horizontally */
}
 
.my-container {
  padding: 10px;
  color: #ffffff;
  border-radius: 5px;
  margin-top: 20px;
  margin-bottom: 20px;
  box-shadow: 0 0 10px 5px #00f0ff;
  animation: neonGlow 6s infinite;
  cursor: pointer;
}
</style>
