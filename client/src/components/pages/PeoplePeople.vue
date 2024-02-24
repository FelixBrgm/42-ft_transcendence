<template>
  <div id="app">
    <GenHeader />
    <div id="Body" class="body">
      <div class="card-body text-center">
        <div>
          <div class="mhistory">
            <div>Friends:</div>
            <div v-if="friends !== null && friends.length > 0"> <!-- Use div instead of ul -->
              <div v-for="friend in friendInfos" :key="friend.id" @click="this.$router.push({ link: `/profile`, query: { uid: friend.id } })">
                <!-- Use div instead of span -->
                {{ friend.alias }}
              </div>
            </div>
            <div v-else>
              No friends. 
            </div>
          </div>
        </div>
      </div>
    </div>
    <GenFooter />
  </div>
</template>

<script>
import axios from "axios";
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
      friendInfos: [],
      matchInfos: [],
      friends: null,
      friendimg: null,
      ism: false,
      isb: false,
      isf: false,
      uid: "",
    };
  },
  created() {
    this.fetchFriends();
    console.log("BASE_URL:", process.env.BASE_URL);
  },
  watch: {
    $route() {
      this.fetchFriends();
    },
  },
  methods: {
    async fetchFriends() {
      try {
        const response = await axios.get(
          `http://127.0.0.1:8080/user/list`,
          { withCredentials: true }
        );
        this.friends = response.data;
        this.friendInfos = [];
        for (const friend of this.friends) {
          // Added missing 'const' and 'of' keywords
          const userId =
            friend.user1 === this.$route.query.uid
              ? friend.user1
              : friend.user2;
          try {
            const response = await axios.get(
              `http://127.0.0.1:8080/user/${userId}`,
              { withCredentials: true }
            );
            this.friendInfos.push(response.data); // Save user info to array
          } catch (error) {
            console.error("Error fetching user info:", error);
          }
        }
      } catch (error) {
        console.error("Error fetching friends:", error);
      }
    },
    }, 
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
  cursor: pointer;
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
  text-shadow: 0 0 10px hsl(45, 100%, 60%), 0 0 20px hsl(45, 100%, 60%),
    0 0 30px hsl(45, 100%, 60%);
  cursor: pointer;
}
</style>