<template>
	<div class="body">
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
		<button @click="user_rooms">my Rooms</button>
		<form @submit.prevent="create_room">
		<input type="text" v-model="newRoom.name" placeholder="Room Name" required />
		<input type="text" v-model="newRoom.topic" placeholder="Topic (optional)" />
		<label>
			Is Public:
			<input type="checkbox" v-model="newRoom.is_public" />
		</label>
		<button type="submit">Create Room</button>
		</form>
		<form @submit.prevent="create_personal_room">
		<input type="number" v-model="partnerId" placeholder="Partner ID" required />
		<button type="submit">Create personal Room</button>
		</form>
		<form @submit.prevent="update_room">
		<input type="number" v-model="updateRoomData.id" placeholder="Room ID" required />
		<input type="text" v-model="updateRoomData.name" placeholder="New Room Name (optional)" />
		<input type="text" v-model="updateRoomData.topic" placeholder="New Topic (optional)" />
		<label>
			Is Public:
			<input type="checkbox" v-model="updateRoomData.is_public" />
		</label>
		<button type="submit">Update Room</button>
		</form>
		<form @submit.prevent="list_room">
		<input type="number" v-model="listRoomId" placeholder="Room ID" required />
		<button type="submit">List Room Users</button>
		</form>
		<form @submit.prevent="join_room">
		<input type="number" v-model="createRoomId" placeholder="Room ID" required />
		<button type="submit">Join Room</button>
		</form>
		<form @submit.prevent="part_room">
		<input type="number" v-model="partRoomId" placeholder="Room ID" required />
		<button type="submit">Part Room</button>
		</form>
		<h3>Messages:</h3>
		<form @submit.prevent="messages_room">
		<input type="number" v-model="messageRoomId" placeholder="Room ID" required />
		<button type="submit">Room messages</button>
		</form>
		<h3>Reset:</h3>
		<button @click="clear">CLEAR</button>
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
		is_public: false,
		},
		updateRoomData: {
			id: null,
			name: '',
			topic: '',
			is_public: false,
		},
		partRoomId: null,
		messageRoomId: null,
		createRoomId: null,
		listRoomId: null,
		partnerId: null,
		newMessage: {
			sender_id: null,
			room_id: null,
			message: '',
		},
		};
	},
	methods: {
		async login() {
		try {
			window.location.href = '/auth/login';
		} catch (error) {
			console.error('Error Initiating login:', error);
		}
		},
		async logout() {
		try {
			const response = await axios.get('/auth/logout', {
			withCredentials: true,
			});
			this.data = response.data;
		} catch (error) {
			console.error('Error Logging out:', error);
		}
		},
		async auth_check() {
			try {
				const response = await axios.get('/auth/check', {
				withCredentials: true,
				});
				this.data = response.data;
			} catch (error) {
				console.error('Error Logging out:', error);
			}
		},
		async home() {
		try {
			const response = await axios.get('');
			this.data = response.data;
		} catch (error) {
			console.error('Error fetching home data:', error);
		}
		},
		async users() {
		try {
			const response = await axios.get('/users');
			this.data = response.data;
		} catch (error) {
			console.error('Error fetching users:', error);
		}
		},
		async user() {
		try {
			const response = await axios.get('/user', {
			withCredentials: true,
			});
			this.data = response.data;
		} catch (error) {
			console.error('Error fetching user:', error);
		}
		},
		async update_user() {
		try {
			const response = await axios.post('/user', this.updateUser, {
			withCredentials: true,
			});
			this.data = response.data;
		} catch (error) {
			console.error('Error fetching user:', error);
		}
		},
		async rooms() {
		try {
			const response = await axios.get('/rooms');
			this.data = response.data;
		} catch (error) {
			console.error('Error fetching user:', error);
		}
		},
		async user_rooms() {
		try {
			const response = await axios.get('/user/room', {
			withCredentials: true,
			});
			this.data = response.data;
		} catch (error) {
			console.error('Error fetching user:', error);
		}
		},
		async create_room() {
		try {
			this.newRoom.is_public = Boolean(this.newRoom.is_public);
			const response = await axios.post('/room/create', this.newRoom, {
			withCredentials: true,
			});
			this.data = response.data;
		} catch (error) {
			console.error('Error fetching user:', error);
		}
		},
		async create_personal_room() {
		try {
			const url = `/room/create/personal/${this.partnerId}`;
			const response = await axios.post(url, null, {
			withCredentials: true,
			});
			this.data = response.data;
		} catch (error) {
			console.error('Error fetching user:', error);
		}
		},
		async update_room() {
		try {
			this.updateRoomData.is_public = Boolean(this.updateRoomData.is_public);
	
			const response = await axios.post('/room/update', this.updateRoomData, {
			withCredentials: true,
			});
	
			this.data = response.data;
		} catch (error) {
			console.error('Error updating room:', error);
		}
		},
		async list_room() {
		try {
			const url = `/room/list/${this.listRoomId}`;
			const response = await axios.get(url, null, {
			withCredentials: true,
			});
			this.data = response.data;
		} catch (error) {
			console.error('Error fetching user:', error);
		}
		},
		async join_room() {
		try {
			const url = `/room/join/${this.createRoomId}`;
			const response = await axios.post(url, null, {
			withCredentials: true,
			});
			this.data = response.data;
		} catch (error) {
			console.error('Error fetching user:', error);
		}
		},
		async part_room() {
		try {
			const url = `/room/part/${this.partRoomId}`;
			const response = await axios.post(url, null, {
			withCredentials: true,
			});
			this.data = response.data;
		} catch (error) {
			console.error('Error fetching user:', error);
		}
		},
		async messages_room() {
		try {
			const url = `/room/messages/${this.messageRoomId}`;
			const response = await axios.get(url, null, {
			withCredentials: true,
			});
			this.data = response.data;
		} catch (error) {
			console.error('Error fetching user:', error);
		}
		},
		async clear() {
		try {
			const url = `/clear`
			const response = await axios.get(url);
			this.data = response.data;
		} catch (error) {
			console.error('Error fetching user:', error);
		}
		},
	},
	};
	</script>
	
