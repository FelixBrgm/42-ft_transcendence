<template>
  <div id="app">
    <GenHeader />
    <div id="Body" class="body">
      <div class="card-body text-center">
        <div class="profile-container">
          <div class="profile-pic-container">
            <!-- Avatar image -->
            <img
              v-if="user !== null && user !== undefined"
              class="rounded-circle profile-pic"
              alt="profile avatar"
              :src="user.avatar"
            />
          </div>
          <h1 v-if="this.isblocked()" style="color: red;">BLOCKED!!</h1>
        </div>
        <div v-show="!isUidMatch" class="icons-container">
          <!-- Block and add icons -->
          <img
            v-if="user !== null && user !== undefined"
            class="icon"
            :alt="'Add Friend'" 
            :src="this.friendimg"
            @click="addFriend"
          />
          <img
            v-if="user !== null && user !== undefined"
            class="icon"
            alt="block user"
            src="@/assets/block-user.png"
            @click="blockUser"
          />
        </div>
        <h1 class="neon-text" id="editableHeader" @click="changeUsername">{{ user == null || user == undefined ? "Loading..." : (user.alias || "User") }}</h1>
        <div>
          <div class="mhistory">
            <div>Matchmaking history</div> 
            <span>{{ this.seperator }}</span>
            <ul v-if="matchs !== null && matchs.length > 0" >
              <li v-for="match in matchs" :key="match.id">
                {{ match.name }}
              </li>
            </ul>
            <div v-else> 
              no game . _. 
            </div>
          </div>
          <div v-show="isUidMatch" class="mhistory">
            <div>Friends: </div>
            <span>{{ this.seperator }}</span>
            <ul v-if="friends !== null && friends.length > 0" >
              <li v-for="friend in friends" :key="friend.id" @click="goToProfile(friend.id)">
                {{ friend.name }}
              </li>
            </ul>
            <div v-else> 
              no friends . _.
            </div>
          </div>
        </div>
      </div>
    </div>
    <GenFooter />
  </div>
</template>

<script>

import axios from 'axios';
import store from '../../store';
import GenHeader from "@/components/elements/GenHeader.vue";
import GenFooter from "@/components/elements/GenFooter.vue";

export default {
  components: {
    GenHeader,
    GenFooter,
  },
  data() {
    return {
      user: null,
      friends: null,
      friendimg: require("@/assets/add-user.png"),
      matchs: null,
      uid: "",
      seperator: "-------------------------------------------------------------------"
    };
  },
  mounted() {
    this.$store.subscribe((mutation) => {
      this.user = mutation.payload; 
    });
    this.$store.dispatch("auth/updateUser");
    this.fetchFriends();
    this.fetchMatchs();
    this.uid = store.state.auth.user.id; 
  },
  methods: {
        changeUsername() {
      const newUsername = prompt("Enter new username:");
      if (newUsername !== null) {
        // Assuming you have an API endpoint to update the username
        axios.post(`http://127.0.0.1:8080/user`, { alias: newUsername }, { withCredentials: true })
          .then(() => {
            this.user.alias = newUsername;
          })
          .catch(error => {
            console.error('Error updating username:', error);
          });
      }
  }, 
    blockUser() {
      if (this.isblocked()) {
        axios.get(`http://127.0.0.1:8080/block/remove/${this.$route.query.uid}`, { withCredentials: true });
      } else {
        axios.get(`http://127.0.0.1:8080/block/${this.$route.query.uid}`, { withCredentials: true });
      }
    },
    addFriend() {
    },
    goToProfile(friendId) {
      this.$router.push({ path: `/profile/${friendId}` });
    },
    async fetchFriends() {
      try {
        const response = await axios.get(`http://127.0.0.1:8080/friend/list/${this.$route.query.uid}`, { withCredentials: true });
        this.friends = response.data;
      } catch (error) {
        console.error('Error fetching friends:', error);
      }
    },
    async isblocked() {
      try {
        const response = await axios.get(`http://127.0.0.1:8080/block/check/${this.$route.query.uid}`, { withCredentials: true });
        return response.data; 
      } catch (error) {
        console.error('Error fetching blocked:', error);
      }
    },
    async fetchMatchs() {
      try {
        const response = await axios.get(`http://127.0.0.1:8080/game/list/${this.$route.query.uid}`, { withCredentials: true });
        this.matchs = response.data;
      } catch (error) {
        console.error('Error fetching matches:', error);
      }
    }
  },
    computed: {
isUidMatch() {
  const routeUid = this.$route.query.uid;
  const componentUid = this.uid;

  // Check if both values are defined and not null
    const routeUidConverted = isNaN(Number(routeUid)) ? routeUid.toString() : Number(routeUid);
    const componentUidConverted = isNaN(Number(componentUid)) ? componentUid.toString() : Number(componentUid);

    if (routeUidConverted === componentUidConverted)
      return true;
    else  
      return false;
  } 
    }
};
</script>

<style>
@import "./../functions/neonglow.css";

.body {
  font-family: neuropol;
  padding: 1rem;
  border-radius: 20px;
  margin-left: 20px;
  margin-right: 20px;
  flex: 1;

  background-color: #5c5e5f;
  box-shadow: 0 0 10px 0px #00f0ff;
  animation: neonGlow 6s infinite;
  color: white; 
  max-width: 1600px; /* Set the maximum width */
  margin: 0 auto; /* Center the footer */ 
}
.btn {
  background: rgb(59, 57, 57);
}
.card {
  background: rgb(163, 165, 169);
  width: 18rem;
}
.rounded-circle {
  width: 125px;
  height: 125px;
  object-fit: cover;
  border-radius: 50%;
  box-shadow: 0 0 10px 0px #00f0ff;
  animation: neonGlow 6s infinite; 
}

.mhistory {
  background-color: #5c5e5f;
  box-shadow: 0 0 10px 5px #00f0ff;
  animation: neonGlow 6s infinite;
  margin: 20px;
  border-radius: 20px;  
  padding: 15px;
} 

.profile-container {
  display: flex;
  flex-direction: column; /* Align items in a column */
  align-items: center; /* Center items horizontally */
}

.icons-container {
  display: flex;
  justify-content: center; /* Center icons horizontally */
  margin-top: 10px; /* Add space between profile pic and icons */
}
 
.icon {
  width: 30px;
  height: 30px;
  margin: 0 5px; /* Adjust margin as needed */
  cursor: pointer; 
  border-radius: 8px;
    box-shadow: 0 0 10px 0px #00f0ff;
  animation: neonGlow 6s infinite;
}
.neon-text {
	text-shadow: 0 0 10px hsl(45, 100%, 60%), 0 0 20px hsl(45, 100%, 60%), 0 0 30px hsl(45, 100%, 60%);
	cursor: pointer;
}
</style>
