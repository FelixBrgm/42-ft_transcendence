<template>
  <div id="app">
    <GenHeader />
    <div id="Body">
      <div class="body">
        <div class="text-center">
          Here are some options:
          <div class="my-container" @click="playGame">
            <span>Play game</span>
          </div>
          <div class="my-container">
            <span>Create tournament</span>
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
  methods: {
    playGame() {
      axios
        .get("http://localhost:8080/game/matchmake", { withCredentials: true })
        .then(() => {
          this.$router.push("/pong");
        })
        .catch((error) => {
          console.error("Error occurred while initiating game:", error);
        });
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
