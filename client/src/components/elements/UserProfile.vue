<template>
  <div class="user-profile">
    <div class="logo">
      <img src="logo.png" alt="Logo">
      <h1 class="neon-text">User Profile</h1>
    </div>
    <nav>
      <ul>
        <li><a href="#">Home</a></li>
        <li><a href="#">About</a></li>
        <li><a href="#">Contact</a></li>
      </ul>
    </nav>
  </div>
</template>

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
