<template>
  <div id="app">
    <GenHeader />
    <div id="Body" class="body">
      <div class="card-body text-center">
        <div>
          <div class="mhistory">
            <div>People:</div>
            <div v-if="friends !== null && friends.length > 0"> <!-- Use div instead of ul -->
              <div v-for="friend in friendInfos" :key="friend.id" @click="this.$router.push({ path: `/profile`, query: { uid: friend.id } })">
                <div class="mhistory">
                <img
              class="roundeder-circle profile-pic" 
              alt="profile avatar"
              :src="friend.avatar"
                />
            {{ friend.alias }} </div>
              </div>
            </div>
            <div v-else>
              No People.
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
  },
  methods: {
    async fetchFriends() {
      try {
        const response = await axios.get(`/api/users`,{ withCredentials: true });
        this.friends = response.data;
        this.friendInfos = [];
        for (const friend of this.friends) {
          try {
            const response = await axios.get(
              `/api/user/${friend.id}`,
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
  background-color: #5C5E5F;
  box-shadow: 0 0 10px 0px #00F0FF;
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
.roundeder-circle { 
  width: 75px;
  height: 75px;
  object-fit: cover;
  border-radius: 50%;
  box-shadow: 0 0 10px 0px #00F0FF;
  animation: neonGlow 6s infinite;
  cursor: pointer;
}
.mhistory {
  background-color: #5C5E5F;
  box-shadow: 0 0 10px 5px #00F0FF;
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
  box-shadow: 0 0 10px 0px #00F0FF;
  animation: neonGlow 6s infinite;
}
.neon-text {
  text-shadow: 0 0 10px hsl(45, 100%, 60%), 0 0 20px hsl(45, 100%, 60%),
    0 0 30px hsl(45, 100%, 60%);
  cursor: pointer;
}
</style>