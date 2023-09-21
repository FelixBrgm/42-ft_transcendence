<template>
  <div>
    <h3>Health:</h3>
    <button @click="home">Home</button>
    <h3>Authentication:</h3>
    <button @click="login">LOGIN</button>
    <button @click="logout">LOGOUT</button>
    <button @click="auth_check">CHECK</button>
    <h3>Users:</h3>
    <button @click="users">all Users</button>
    <button @click="user">my User</button>
    <form @submit.prevent="update_user">
      <input type="text" v-model="updateUser.login" placeholder="login" />
      <input type="text" v-model="updateUser.status" placeholder="status" />
      <button type="submit">Update User</button>
    </form>
    <h3>Rooms:</h3>
    <button @click="rooms">all Rooms</button>
    <form @submit.prevent="create_room">
      <input type="text" v-model="newRoom.name" placeholder="name" />
      <input type="text" v-model="newRoom.topic" placeholder="topic (optional)" />
      <label>
        <input type="checkbox" v-model="newRoom.is_public" /> Public Room
      </label>
      <button type="submit">Create Room</button>
    </form>
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
      updateUser: {
      login: '',
      status: '',
     },
     newRoom: {
      name: '',
      topic: '',
      is_public: '',
     },
    };
  },
  methods: {
	async login() {
      try {
        window.location.href = 'http://127.0.0.1:8080/auth/login';
      } catch (error) {
        console.error('Error Initiating login:', error);
      }
    },
	async logout() {
      try {
        const response = await axios.get('http://127.0.0.1:8080/auth/logout', {
		withCredentials: true,
		});
        this.data = response.data;
      } catch (error) {
        console.error('Error Logging out:', error);
      }
    },
	async auth_check() {
		try {
			const response = await axios.get('http://127.0.0.1:8080/auth/check', {
			withCredentials: true,
			});
			this.data = response.data;
		} catch (error) {
			this.data = 'User is not authorised'
		}
	},
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
		const response = await axios.get('http://127.0.0.1:8080/user', {
		withCredentials: true,
		});
		this.data = response.data;
	} catch (error) {
		console.error('Error fetching user:', error);
	}
	},
	async update_user() {
	try {
		const response = await axios.post('http://127.0.0.1:8080/user', this.updateUser, {
		withCredentials: true,
		});
		this.data = response.data;
	} catch (error) {
		console.error('Error fetching user:', error);
	}
	},
	async rooms() {
	try {
		const response = await axios.get('http://127.0.0.1:8080/rooms');
		this.data = response.data;
	} catch (error) {
		console.error('Error fetching user:', error);
	}
	},
	async create_room() {
	try {
		const response = await axios.post('http://127.0.0.1:8080/room/create', this.newRoom, {
		withCredentials: true,
		});
		this.data = response.data;
	} catch (error) {
		console.error('Error fetching user:', error);
	}
	},
  },
};
</script>

<style scoped>
</style>
