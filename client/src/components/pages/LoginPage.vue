<template>
  <div class="welcome-container" @mouseover="moveImage(true)" @mouseleave="moveImage(false)">
    <div class="moving-image" :class="{ 'moved-out': imageMoved }"></div>
    <div class="welcome-box">
      Hello
      <div></div> 
      <button @click="login" class="button">Login</button>
    </div>
  </div>
</template>

  
<script setup> 
import { ref } from 'vue';


const imageMoved = ref(false);
</script>
 
<script>
 
export default {
	name: 'LoginPage',
	setup() {
	},
	methods: {
		async login() {
		try {
			window.location.href = '/api/auth/login';
		} catch (error) {
			// console.error('Error Initiating login:', error);
		}
		},
		moveImage(out) {
		this.imageMoved = out;
		},
	},
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
