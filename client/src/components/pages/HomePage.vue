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
          <div class="my-container" @click="playLocal">
            <span>Local game</span>
          </div>
          <div class="my-container">
            <div>1 v 1</div>
            <span>Enter other player ID: </span> 
            <input type="text" id="pID" v-model="vsID" placeholder="Enter player ID">
            <span class="mybutton" style="margin-top: 6px;" @click="joinVs">Start</span> 
          </div>
          <div class="my-container">
            <div> Create Tournament </div>
            <span> Number of players: </span>
            <select id="selectOptions" v-model="selectedNumberOfPlayers">
              <option v-for="number in numbers" :key="number" :value="number">
                {{ number }}
              </option>
            </select >
          <div class="mybutton" style="margin-top: 6px;" @click="playTournament" >
            <span > Create Tournament</span>
          </div>
          <div v-if="togglenum"> Tournament ID: {{this.userId }}</div>
          </div>
          <div class="my-container">
            <div>Join Tournament</div>
            <span>Enter ID: </span> 
            <input type="text" id="tID" v-model="tournamentID" :placeholder="this.togglenum ? this.userId : 'Enter tournament ID'">
            <span class="mybutton" style="margin-top: 6px;" @click="joinTournament">Start</span> 
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
import store from '../../store';
import axios from 'axios'; 

export default {
  components: {
    GenHeader,
    GenFooter,
  },
  data() {
    return {
      selectedNumberOfPlayers: 4,
      tournamentID: null,
      vsID: null, 
      togglenum: false,
      userId: null, 
      numbers: Array.from({ length: 6 }, (_, index) => Math.pow(2, index + 2)),
    };
  },
  methods: {
    playGame() {
      this.$router.push({ path: "/pong", query: { startGame: true } }); 
    },
    playLocal() {
      this.$router.push({ path: "/local"}); 
    },
    playTournament() {
      if (this.selectedNumberOfPlayers) {
        this.togglenum = true; 
        this.userId = store.state.auth.user.id 
        this.tournamentID = this.userId;
        axios.get(`/api/game/create_tournament/${this.selectedNumberOfPlayers}`, { withCredentials: true })
        .catch(error => {
          alert("An error occurred: " + error.message);
        }); 
      }
      else 
        alert("Please select the number of players for the tournament.");
    },
    async joinTournament() {
      if (this.tournamentID && (await this.checkExUid(this.tournamentID)) === true) {
        this.$router.push({ path: "/pong", query: { joinTournament: this.tournamentID } });
      } else {
        alert("Please enter a valid Tournament ID");
      }
    },
    async joinVs() {
      if (this.vsID && (await this.checkExUid(this.vsID)) === true) { 
        this.$router.push({ path: "/pong", query: { joinvs: this.vsID } });
      } else {
        alert("Please enter a valid player ID");
      }
    },
    async checkExUid(num){
      try {
        if (typeof num === 'number') {
          const response = await axios.get(`/api/user/check/${num}`, { withCredentials: true });
          console.log(response.data);
          return response.data === true;
        }
        return false;
      } catch (error) {
        console.error("Error fetching data:", error);
        return false;
      }
    },
  },
};
</script>
 
<style scoped> 
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