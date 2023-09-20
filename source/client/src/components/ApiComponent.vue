<template>
  <div>
    <button @click="home">Home</button>
    <button @click="users">Users</button>
    <button @click="login">LOGIN</button>
    <button @click="logout">LOGOUT</button>
    <button @click="user">User</button>
    <div v-if="data">
      <h2>API Response:</h2>
      <pre>{{ data }}</pre>
    </div>
  </div>
</template>

<script>
import axios from 'axios';

export default {
  data() {
    return {
      data: null,
    };
  },
  methods: {
    async home() {
      try {
        const response = await axios.get('http://127.0.0.1:8080');
        this.data = response.data;
      } catch (error) {
        console.error('Error fetching home data:', error);
      }
    },
    async users() {
      try {
        const response = await axios.get('http://127.0.0.1:8080/users');
        this.data = response.data;
      } catch (error) {
        console.error('Error fetching users:', error);
      }
    },
	async user() {
      try {
        const response = await axios.get('http://127.0.0.1:8080/user');
        this.data = response.data;
      } catch (error) {
        console.error('Error fetching user:', error);
      }
    },
	async login() {
      try {
        window.location.href = 'http://127.0.0.1:8080/auth/login';
      } catch (error) {
        console.error('Error Initiating login:', error);
      }
    },
	async logout() {
      try {
        const response = await axios.get('http://127.0.0.1:8080/auth/logout'); // Replace with your API endpoint
        this.data = response.data;
      } catch (error) {
        console.error('Error Logging out:', error);
      }
    },
    // Add more methods for other requests as needed
  },
};
</script>

<style scoped>
/* Your component styles here */
</style>
