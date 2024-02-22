<template>
  <div class="card d-flex justify-content-center align-items-center">
    <div class="card-body text-center">
      <div class="profile-pic overflow-hidden">
        <img
          v-if="user != undefined"
          class="rounded-circle"
          alt="profile avatar"
          :src="user.avatar"
        />
      </div>
      <a href="#" class="btn btn-primary">{{
        user == undefined ? "Loading..." : user.alias
      }}</a> 
    </div>
  </div> 
</template>
<script>
export default {
  data() {
    this.$store.subscribe((mutation) => {
      this.user = mutation.payload;
    });
    this.$store.dispatch("auth/updateUser");
    return {
      user: null,
    };
  },
};
</script>
<style scoped>
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
</style>

<style scoped>
.user-profile {
  font-family: neuropol;
  background-color: #838a8d;
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
  text-shadow: 0 0 10px hsl(45, 100%, 60%), 0 0 20px hsl(45, 100%, 60%),
    0 0 30px hsl(45, 100%, 60%);
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


.body {
  font-family: neuropol;
  padding: 1rem;
  background-color: #5c5e5f;
  box-shadow: 0 0 10px 5px #00f0ff;
  animation: neonGlow 6s infinite;
  max-width: 1600px;
  min-width: 900px;
  width: 100%;
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

nav a,
.title-link {
  text-decoration: none;
  color: #fcfcfc;
  margin-right: 1rem;
  transition: color 0.3s ease;
}

nav a:hover {
  color: #000000;
}
</style>