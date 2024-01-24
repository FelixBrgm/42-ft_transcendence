<!-- PongGame.vue -->
<template>
  <div class="game-container" @keydown="handleKeyPress" @keyup="handleKeyRelease" ref="gameContainer" tabindex="0">
    <!-- Score Counter -->
    <div class="score-counter">{{ playerScore }} - {{ enemyScore }}</div>

    <!-- Player and Enemy paddles -->
    <div class="player" :style="{ top: playerPosition + 'px' }"></div>
    <div class="enemy" :style="{ top: enemyPosition + 'px' }"></div>

    <!-- Ball -->
    <div class="ball" :style="{ top: ballPosition.top + 'px', left: ballPosition.left + 'px' }"></div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      playerPosition: 410,
      enemyPosition: 410,
      playerMovingDirection: null,
      enemyMovingDirection: null,
      animationFrameId: null,
      playerScore: 0,  // Initialize player score to 0
      enemyScore: 0,   // Initialize enemy score to 0
      ballPosition: {
        top: 445,
        left: 785,
      },
      ballSpeed: {
        x: 5,
        y: 5,
      },
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
    moveBall() {
      const ballStepX = this.ballSpeed.x;
      const ballStepY = this.ballSpeed.y;

      // Update ball position
      this.ballPosition.left += ballStepX;
      this.ballPosition.top += ballStepY;

      // Check for collisions with top and bottom walls
      if (
        this.ballPosition.top <= 0 ||
        this.ballPosition.top + 20 >= this.$refs.gameContainer.clientHeight
      ) {
        this.ballSpeed.y = -this.ballSpeed.y; // Reverse the vertical direction on collision
      }

      // Check for collisions with player paddle (right side)
      const playerCollision =
        this.ballPosition.left + 20 >= this.$refs.gameContainer.clientWidth - 30 &&
        this.ballPosition.top + 20 >= this.playerPosition &&
        this.ballPosition.top <= this.playerPosition + 120;

      // Check for collisions with enemy paddle (left side)
      const enemyCollision =
        this.ballPosition.left <= 30 &&
        this.ballPosition.top + 20 >= this.enemyPosition &&
        this.ballPosition.top <= this.enemyPosition + 120;

      if (playerCollision || enemyCollision) {
        // Collision with a paddle
        this.ballSpeed.x = -this.ballSpeed.x; // Reverse the horizontal direction on collision
        // You can add additional logic to modify the ball's speed here if needed
      }

      if (this.ballPosition.left <= 0) {
        // Ball went out of bounds on the left side
        this.enemyScore++;
        this.resetBall();
      } else if (this.ballPosition.left + 20 >= this.$refs.gameContainer.clientWidth) {
        // Ball went out of bounds on the right side
        this.playerScore++;
        this.resetBall();
      }

      this.animationFrameId = requestAnimationFrame(() => this.moveBall());
    },

    resetBall() {
      // Reset the ball position to the center
      this.ballPosition = {
        top: this.$refs.gameContainer.clientHeight / 2 - 10,
        left: this.$refs.gameContainer.clientWidth / 2 - 10,
      };

      // Reset the ball speed to its initial values
      this.ballSpeed = { x: 5, y: 5 };
    },
  },
  updated() {
    if (this.playerMovingDirection !== null || this.enemyMovingDirection !== null) {
      this.$refs.gameContainer.focus(); // Ensure the container has focus for key events
    }
  },
  mounted() {
    this.moveBall();
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
  text-color: white;
}
.ball {
  position: absolute;
  width: 20px;
  height: 20px;
  background-color: #fff; /* White color for the ball */
  border-radius: 50%;
  box-shadow: 0 0 10px #fff, 0 0 20px #fff, 0 0 30px #fff;
}
</style>
