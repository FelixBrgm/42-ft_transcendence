<template>
  <div id="app">
    <GenHeader />
    <div id="Body" class="body">
      <div class="card-body text-center">
        <div class="profile-pic overflow-hidden">
          <img
            v-if="user !== null && user !== undefined"
            class="rounded-circle"
            alt="profile avatar"
            :src="user.avatar"
          />
        </div>
        <h1>{{ user == null || user == undefined ? "Loading..." : (user.alias || "User") }}</h1>
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
              <li v-for="friend in friends" :key="friend.id">
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
    async fetchFriends() {
      try {
        const response = await axios.get(`http://127.0.0.1:8080/friend/list/${this.$route.query.uid}`, { withCredentials: true });
        this.friends = response.data;
      } catch (error) {
        console.error('Error fetching friends:', error);
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
}

.mhistory {
  background-color: #5c5e5f;
  box-shadow: 0 0 10px 5px #00f0ff;
  animation: neonGlow 6s infinite;
  margin: 20px;
}
</style>
