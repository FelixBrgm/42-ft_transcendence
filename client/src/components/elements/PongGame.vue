<!-- PongGame.vue -->
<template>
  <div class="game-container" @keydown="handleKeyPress" @keyup="handleKeyRelease" ref="gameContainer" tabindex="0">
    <!-- Score Counter -->
    <div class="score-counter">{{ playerScore }} - {{ enemyScore }}</div>

    <!-- Player and Enemy paddles -->
    <div class="player" :style="{ top: playerPosition + 'px' }"></div>
    <div class="enemy" :style="{ top: enemyPosition + 'px' }"></div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      playerPosition: 410, // Initial position in the middle
      enemyPosition: 410, // Initial enemy position in the middle
      playerMovingDirection: null, // To keep track of the player's movement direction
      enemyMovingDirection: null, // To keep track of the enemy's movement direction
      animationFrameId: null, // To keep track of the animation frame ID
    };
  },
  methods: {
    handleKeyPress(event) {
      if (event.key === 'ArrowUp' && this.playerMovingDirection !== 'up') {
        event.preventDefault(); // Prevent default browser scrolling behavior
        this.playerMovingDirection = 'up';
        this.movePlayer();
      } else if (event.key === 'ArrowDown' && this.playerMovingDirection !== 'down') {
        event.preventDefault(); // Prevent default browser scrolling behavior
        this.playerMovingDirection = 'down';
        this.movePlayer();
      } else if (event.key === 'w' && this.enemyMovingDirection !== 'up') {
        event.preventDefault(); // Prevent default browser scrolling behavior
        this.enemyMovingDirection = 'up';
        this.moveEnemy();
      } else if (event.key === 's' && this.enemyMovingDirection !== 'down') {
        event.preventDefault(); // Prevent default browser scrolling behavior
        this.enemyMovingDirection = 'down';
        this.moveEnemy();
      }
    },
    handleKeyRelease(event) {
      event.preventDefault(); // Prevent default browser scrolling behavior
      if (event.key === 'ArrowUp' || event.key === 'ArrowDown') {
        this.playerMovingDirection = null; // Stop the player movement on key release
      } else if (event.key === 'w' || event.key === 's') {
        this.enemyMovingDirection = null; // Stop the enemy movement on key release
      }

      cancelAnimationFrame(this.animationFrameId); // Cancel the animation frame
    },
    movePlayer() {
      const playerStep = 15; // Adjust the step size as needed

      if (this.playerMovingDirection === 'up' && this.playerPosition > 60) {
        this.playerPosition -= playerStep;
      } else if (
        this.playerMovingDirection === 'down' &&
        this.playerPosition < this.$refs.gameContainer.clientHeight - 60
      ) {
        this.playerPosition += playerStep;
      }

      this.animationFrameId = requestAnimationFrame(() => this.movePlayer()); // Smooth animation
    },
    moveEnemy() {
      const enemyStep = 15; // Adjust the step size as needed

      if (this.enemyMovingDirection === 'up' && this.enemyPosition > 60) {
        this.enemyPosition -= enemyStep;
      } else if (
        this.enemyMovingDirection === 'down' &&
        this.enemyPosition < this.$refs.gameContainer.clientHeight - 60
      ) {
        this.enemyPosition += enemyStep;
      }

      this.animationFrameId = requestAnimationFrame(() => this.moveEnemy()); // Smooth animation
    },
  },
  updated() {
    if (this.playerMovingDirection !== null || this.enemyMovingDirection !== null) {
      this.$refs.gameContainer.focus(); // Ensure the container has focus for key events
    }
  },
};
</script>

<style scoped>
.game-container {
  position: relative;
  width: 1600px;
  height: 900px;
  margin: 0 auto; /* Center the container horizontally */
  border: 1px solid #00000;
  overflow: hidden; /* Prevent player from moving outside the container */
  border-radius: 20px; 
  box-shadow: 0 0 10px 5px #00f0ff;
  animation: neonGlow 6s infinite;
}


.player {
    position: absolute;
    width: 30px;
    height: 120px;
    background-color: hsl(45, 100%, 60%); /* Same color as the text */
    box-shadow: 0 0 10px hsl(45, 100%, 60%), 0 0 20px hsl(45, 100%, 60%), 0 0 30px hsl(45, 100%, 60%);
    right: 0;
    top: 50%;
    transform: translateY(-50%);
    border-radius: 20px;
}

.enemy {
    position: absolute;
    width: 30px;
    height: 120px;
    background-color: red; /* Red color for the enemy */
    box-shadow: 0 0 10px red, 0 0 20px red, 0 0 30px red;
    left: 0;
    top: 50%;
    transform: translateY(-50%);
    border-radius: 20px;
}	
.score-counter {
  position: absolute;
  top: 20px;
  left: 50%;
  transform: translateX(-50%);
  font-family: 'neuropol', sans-serif;
  font-size: 24px;
  color: rgba(255, 255, 255, 0.8); /* Round, translucent */
  text-shadow: 0 0 10px rgba(0, 240, 255, 0.8); /* Glowing shadow */
  border-radius: 20px;
}
</style>
