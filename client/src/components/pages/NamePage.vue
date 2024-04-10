<template>
  <div class="welcome-container" @mouseover="moveImage(true)" @mouseleave="moveImage(false)">
    <div class="moving-image" :class="{ 'moved-out': imageMoved }"></div>
    <div class="welcome-box">
      Please enter your desired username
      <input type="text" id="uname" v-model="uname">
      <div></div> 
      <button @click="changeUsername(uname)" class="button">Set</button>
    </div>
  </div>
</template>

<script setup> 
import { ref } from 'vue';
import axios from "axios";
const imageMoved = ref(false);
const uname = ref('');
const changeUsername = (newUsername) => {
  if (newUsername.value !== null) {
    const alias = newUsername.value.trim().substring(0, 20);
    axios
      .post(
        `/user`,
        { alias: alias},
        { withCredentials: true }
      )
      .catch(() => {
        // console.error("Error updating username:", error);
      });
  }
};
const moveImage = (out) => {
  imageMoved.value = out;
};
</script>
  
<style scoped>
.welcome-container {
  display: flex;
  justify-content: center; 
  align-items: center;
  height: 100vh;
}

.welcome-box {
  background-color: #5c5e5f;
  color: white;
  border-radius: 20px;
  font-family: neuropol;
  font-size: 50px;
  text-align: center;
  padding: 20px;
  box-shadow: 0 0 10px 5px #00f0ff;
  animation: neonGlow 6s infinite;
  margin: 30px auto; /* Centering the header */
  z-index: 2;
}

.content {
  padding: 20px;
  padding: 20px;
}

.button {
  border-radius: 20px;
  font-family: neuropol;
  font-size: 20px; 
  margin-top: 20px;
  margin-bottom: 20px;
  font-family: neuropol; 
  box-shadow: 0 0 10px 5px #00f0ff;
  animation: neonGlow 6s infinite;
}

.moving-image {
  background-image: url('../../assets/logo.gif'); /* Set your image path */
  background-size: cover;
  width: 100px; /* Set your image width */
  height: 100px; /* Set your image height */
  position: absolute;
  top: 50%; /* Initial position in the middle of the page */
  left: 49%;
  transform: translate(-50%, -50%);
  transition: top 0.5s ease; /* Add a smooth transition effect */
  z-index: 1;
}


.moved-out {
  top: calc(50% - 80px); /* Move the image up by 50px when hovered over the container */
}

</style>
