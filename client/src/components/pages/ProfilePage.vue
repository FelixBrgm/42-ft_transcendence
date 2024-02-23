<template>
  <div id="app">
    <GenHeader />
    <div id="Body" class="body">
      <div class="card-body text-center">
        <div class="profile-container">
          <div class="profile-pic-container">
            <!-- Avatar image -->
            <img
              @click="changePic"
              v-if="user !== null && user !== undefined"
              class="rounded-circle profile-pic"
              alt="profile avatar"
              :src="user.avatar"
            />
          </div>
          <h1 v-if="this.isb" style="color: red">BLOCKED!!</h1>
        </div>
        <div v-show="!isUidMatch" class="icons-container">
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
        <h1 class="neon-text" id="editableHeader" @click="changeUsername">
          {{
            user == null || user == undefined
              ? "Loading..."
              : user.alias || "User"
          }}
        </h1>
        <div>
          <div class="mhistory">
            <div>Matchmaking history</div>
            <span>{{ this.seperator }}</span>
              <div v-if="matchInfos !== null "> 
                <div v-for="match in matchInfos" :key="match.id" > 
                  On: {{ formattedTimestamp(match.timestamp) }}
                  Winner: {{ match.winner.alias }}
                  Looser: {{ match.looser.alias }}     
                </div>
              </div> 
              <div v-else>no game . _.</div>
          </div>
          <div v-show="isUidMatch" class="mhistory">
            <div>Friends:</div>
            <span>{{ this.seperator }}</span>
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
import store from "../../store";
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
      seperator:
        "-------------------------------------------------------------------",
    };
  },
  created() {
    this.fetchData();
  },
  watch: {
    $route() {
      this.fetchData();
    },
  },
  mounted() {
    this.$store.dispatch("auth/updateUser");
    this.fetchMatchs();
    this.fetchFriends();
    this.uid = `${this.$route.query.uid}`;
  },
  methods: {
    async fetchData() {
      this.uid = this.$route.query.uid;
      await this.getUser();
      this.isf = await this.isfriend();
      this.isb = await this.isblocked();
      this.ism = this.isUidMatch;
      if (this.isf === undefined)
        this.isf = false;
      if (this.isb === undefined)
        this.isb = false;
      this.friendimg = this.isf
        ? require("@/assets/add-user.png")
        : require("@/assets/delete-user.png"); 
      if (!this.isf) {
        this.friendimg = require("@/assets/add-user.png");
      } else {
        this.friendimg = require("@/assets/delete-user.png");
      }
    },
    changeUsername() {
      if (this.ism) {
        const newUsername = prompt("Enter new username:");
        if (newUsername !== null) {
          // Assuming you have an API endpoint to update the username
          axios
            .post(
              `http://127.0.0.1:8080/user`,
              { alias: newUsername },
              { withCredentials: true }
            )
            .then(() => {
              this.user.alias = newUsername;
            })
            .catch((error) => {
              console.error("Error updating username:", error);
            });
        }
      }
    },
    changePic() {
      if (this.ism) {
        const newAvatar = prompt("Enter new avatar link");
        if (newAvatar !== null) {
          // Preload image to check validity
          const img = new Image();
          img.onload = async () => {
            // Image loaded successfully, update avatar
            try {
              await axios.post(
                `http://127.0.0.1:8080/user`,
                { avatar: newAvatar },
                { withCredentials: true }
              );
              this.user.avatar = newAvatar;
            } catch (error) {
              console.error("Error updating avatar:", error);
            }
          };
          img.onerror = () => {
            // Image failed to load, display error message
            alert("Invalid image link! Fix that");
            console.error("Invalid image link");
          };
          img.src = newAvatar;
        }
      }
    },
    blockUser() {
      axios.get(`http://127.0.0.1:8080/block/${this.$route.query.uid}`, {
        withCredentials: true,
      });
      this.isb = !this.isb; 
    },
    addFriend() {
      axios.get(
        `http://127.0.0.1:8080/friend/${this.$route.query.uid}`,
        { withCredentials: true }
      );
      this.friendimg = this.isf
        ? require("@/assets/add-user.png")
        : require("@/assets/delete-user.png");
      this.isf = !this.isf;
    },
    async fetchSingle(tofind){
      try {
              const response = await axios.get(`http://127.0.0.1:8080/user/${tofind}`, { withCredentials: true });
              return (response.data.alias);
            } catch (error) {  
              console.error('Error fetching user info:', error); 
            }
    },
    async fetchFriends() {
      try {
        const response = await axios.get(
          `http://127.0.0.1:8080/friend/list/${this.$route.query.uid}`,
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
    async fetchMatchs() {
        try {
          const response = await axios.get(
            `http://127.0.0.1:8080/game/list/${this.$route.query.uid}`, { withCredentials: true }
          );
          this.matchInfos = [];
            console.log(response.data);
          for (const match of response.data) {
            console.log(match);
            try {
              const response1 = await axios.get(`http://127.0.0.1:8080/user/${match.winner}`, { withCredentials: true });
              const response2 = await axios.get(`http://127.0.0.1:8080/user/${match.looser}`, { withCredentials: true });
              this.matchInfos.push({timestamp: match.timestamp, winner: response1.data, looser: response2.data });
            } catch (error) { 
              console.error("Error fetching match info:", error);
            }
          }
        } catch (error) {
          console.error("Error fetching matches:", error);
        }
      }, 
    async isblocked() {
        try {
          const response = await axios.get(
            `http://127.0.0.1:8080/block/check/${this.$route.query.uid}`,
            { withCredentials: true }
          );
          return response.data;
        } catch (error) {
          console.error("Error fetching blocked:", error);
        }
    },
    async isfriend() {
        try {
          const response = await axios.get(
            `http://127.0.0.1:8080/friend/check/${this.$route.query.uid}`,
            { withCredentials: true }
          );
          return response.data;
        } catch (error) {
          console.error("Error fetching friend:", error);
        }
    },
    async getUser() {
      try {
        const response = await axios.get(
          `http://127.0.0.1:8080/user/${this.$route.query.uid}`,
          { withCredentials: true }
        );
        this.user = response.data;
      } catch (error) {
        console.error("Error fetching user:", error);
      }
    },
  },
  computed: {
    formattedTimestamp() {
      return (timestamp) => {
        const date = new Date(timestamp);
        return date.toLocaleString(); // Customize this as per your requirement
      };
    },
    isUidMatch() {
      const routeUid = this.$route.query.uid;
      const user = store.state.auth.user;

      // Check if routeUid is defined and not null
      if (routeUid && user && user.id) {
        const componentUid = user.id;

        const routeUidConverted = isNaN(Number(routeUid))
          ? routeUid.toString()
          : Number(routeUid);
        const componentUidConverted = isNaN(Number(componentUid))
          ? componentUid.toString()
          : Number(componentUid);

        return routeUidConverted === componentUidConverted;
      } else {
        return false;
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