
<template>
	<header class="app-header">
		<router-link :to="{ path: '/' }" class="title-link">
		<div class="logo">
			<img src="@/assets/logo.gif" alt="Logo" @click="playSound"/>
			<h1 class="neon-text">{{ title }}</h1>
		</div>
		</router-link>
		<nav>
      <ul>
        <li v-for="(item, index) in menuItems" :key="index">
          <a :href="generateLink(item)" class="neon-text">{{ item.text }}</a>
        </li>
        <a v-on:click="logout" class="neon-text">Logout</a>
      </ul>
		</nav>
	</header>
</template> 

<script>
import axios from 'axios';
import { Howl } from 'howler';
import honkSound from '@/assets/honk.mp3';
import store from '../../store';


export default {
  data() {
    return {
      title: "Transcendence",
      menuItems: [
        { text: "Home", link: "/" },
        { text: "Rules", link: "/rules" },
        { text: "Profile", link: "/profile" },
        { text: "People", link: "/people" },
      ],
      sound: null, 
    };
  },
  methods: {
		generateLink(item) {
		if (item.text === "Profile" && store.state.auth.user !== null) {
			return `${item.link}?uid=${store.state.auth.user.id}`;
		} else {
			return item.link;
		}
		},
	async logout() {
		try {
			await axios.get('http://127.0.0.1:8080/auth/logout', { withCredentials: true });
			store.state.auth.user = null;
			this.$router.push('/login');
		} catch (error) {
			console.error('Error Logging out:', error);
		}
	},
	playSound() {
		if (this.sound) {
			this.sound.play();
		}
	},
	},
	mounted() {
		this.sound = new Howl({
		src: [honkSound],
		});
	}
};
</script>


<style scoped>
.app-header {
  font-family: neuropol;
  background-color: #5c5e5f;
  color: #fff;
  padding: 1rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-radius: 20px;
  box-shadow: 0 0 10px 5px #00f0ff;
  animation: neonGlow 6s infinite;
  margin: 30px auto; /* Centering the header */
  max-width: 1600px;
  min-width: 950px; 
}
 
.neon-text {
	text-shadow: 0 0 10px hsl(45, 100%, 60%), 0 0 20px hsl(45, 100%, 60%), 0 0 30px hsl(45, 100%, 60%);
	cursor: pointer;
}

@keyframes neonGlow {
	0% {
		box-shadow: 0 0 10px 5px hsl(45, 100%, 60%);
	}
	25% {
		box-shadow: 0 0 10px 5px hsl(135, 100%, 60%);
	}
	50% {
		box-shadow: 0 0 10px 5px hsl(225, 100%, 60%);
	}
	75% {
		box-shadow: 0 0 10px 5px hsl(315, 100%, 60%);
	}
	100% {
		box-shadow: 0 0 10px 5px hsl(45, 100%, 60%);
	}
}

.logo {
	display: flex;
	align-items: center;
}

.logo img {
	max-width: 70px;
	margin-right: 1rem;
}

nav ul {
	list-style: none;
	display: flex;
}

nav a, .title-link {
	text-decoration: none;
	color: #fcfcfc;
	margin-right: 1rem;
	transition: color 0.3s ease;
}

nav a:hover {
	color: #000000;
}
</style>