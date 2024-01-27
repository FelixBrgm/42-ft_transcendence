<template>
    <div style="color: white;">
        Hello
        <div>{{ auth.user }}</div>
        <button @click="login">Login</button>
    </div>
</template>
  
<script setup>
import { auth } from '../../router';

</script>

<script>
// import axios from 'axios'; // You might need to adjust the path

export default {
    name: 'LoginPage',
    setup() {
    },
    methods: {
        async login() {
            // Perform OAuth2 verification
            // const response = await axios.get('http://127.0.0.1:8080/auth/login', {
            //     // Your OAuth2 credentials go here
            // });
            // if (response.data.access_token) {
            //     auth.user = response.data.access_token; // Set the user value in router.js
            //     console.log("Logged in");
            // } else {
            //     console.log("Failed to log in");
            // }

            try {
                window.location.href = 'http://127.0.0.1:8080/auth/login';
            } catch (error) {
                console.error('Error Initiating login:', error);
            }
            this.$router.push('/');
        }
    }
};
</script>
  
<style scoped>
/* Your styles go here */
</style>