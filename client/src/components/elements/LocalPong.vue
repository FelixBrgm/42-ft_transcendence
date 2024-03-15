<template>
  <div>
    <div class="game-container" 
         @keydown="handleKeyPress" 
         @keyup="handleKeyRelease" 
         ref="gameContainer" 
         tabindex="0">
      <div class="score-counter">{{ leftScore }} - {{ rightScore }}</div>
      <div class="rightPaddle" :style="{ top: rightPosition + 'px' }"></div>
      <div class="leftPaddle" :style="{ top: leftPosition + 'px' }"></div>
      <div class="ball" 
           :style="{ top: ballPosition.y + 'px', left: ballPosition.x + 'px' }"></div>
      <div class="start-button" 
           tabindex="0" 
           role="button" 
           :style="{ pointerEvents: startButtonEnabled ? 'auto' : 'none' }" 
           v-html="textValue" 
           ></div>
    </div>
  </div>
</template>



<script>
import router from '../../router.js';

export default {
  data() {
    return {
      leftPosition: 450,
      rightPosition: 450, 
      leftScore: 0,
      rightScore: 0,
      ballPosition: { x: 800, y: 450 },
      ballDirectionX: 1,
      ballDirectionY: 1,
      ballSpeed: 5,
      paddleSpeed: 10, // Adjust paddle speed as needed
      startButtonEnabled: true,
      textValue: "Start Game",
      keysPressed: new Set(),
    };
  },
  mounted() {
    window.addEventListener('keydown', this.handleKeyDown);
    window.addEventListener('keyup', this.handleKeyUp);
    this.startGame();
  },
  beforeUnmount() {
    window.removeEventListener('keydown', this.handleKeyDown);
    window.removeEventListener('keyup', this.handleKeyUp);
  },
  computed: {
    leftPaddleTarget() {
      return this.leftPosition + (this.isKeyPressed('w') ? -this.paddleSpeed : 0) + (this.isKeyPressed('s') ? this.paddleSpeed : 0);
    },
    rightPaddleTarget() {
      return this.rightPosition + (this.isKeyPressed('ArrowUp') ? -this.paddleSpeed : 0) + (this.isKeyPressed('ArrowDown') ? this.paddleSpeed : 0);
    },
  },
  methods: {

    handleKeyDown(event) {
      this.keysPressed.add(event.key);
    },
    handleKeyUp(event) {
      this.keysPressed.delete(event.key);
    },
    isKeyPressed(key) {
      return this.keysPressed.has(key);
    },
      startGame() {
        this.textValue = "Starting Game in 3";
        setTimeout(() => {
            this.textValue = "Starting Game in 2";
            setTimeout(() => {
                this.textValue = "Starting Game in 1";
                setTimeout(() => {
                    this.textValue = "";
                    this.gameLoop();
                }, 1000);
            }, 1000);
        }, 1000);
      },
    movePaddles() {
      // Move left paddle
      if (this.leftPosition !== this.leftPaddleTarget) {
        const targetPosition = Math.max(60, Math.min(840, this.leftPaddleTarget)); // Clamp target position
        this.leftPosition += Math.sign(targetPosition - this.leftPosition) * this.paddleSpeed;
      }

      // Move right paddle
      if (this.rightPosition !== this.rightPaddleTarget) {
        const targetPosition = Math.max(60, Math.min(840, this.rightPaddleTarget)); // Clamp target position
        this.rightPosition += Math.sign(targetPosition - this.rightPosition) * this.paddleSpeed;
      }
    },
      moveBall() {

        this.ballPosition.x += this.ballSpeed * this.ballDirectionX;
        this.ballPosition.y += this.ballSpeed * this.ballDirectionY;

        if (this.ballPosition.y <= 0 || this.ballPosition.y >= 880) {
          this.ballDirectionY *= -1;
        }
      if (!this.paddleCollisionCooldown) {
        if (this.ballPosition.x <= 40 && 
            this.ballPosition.y >= this.leftPosition - 60&& 
            this.ballPosition.y <= this.leftPosition + 60) {
          this.ballDirectionX *= -1;
          this.ballSpeed += 0.1;
        }

        if (this.ballPosition.x >= 1550 && 
            this.ballPosition.y >= this.rightPosition - 60 && 
            this.ballPosition.y <= this.rightPosition + 60) {
          this.ballDirectionX *= -1;
          this.ballSpeed += 0.1;
          this.activatePaddleCollisionCooldown();
        }
      }
        if (this.ballPosition.x <= 0) {
          this.rightScore++;
          this.resetBall();
        } else if (this.ballPosition.x >= 1600) {
          this.leftScore++; 
          this.resetBall();
          this.ballSpeed = 5;
          this.activatePaddleCollisionCooldown();
        }
      },
    resetBall() {
      this.ballPosition = { x: 800, y: 450 };
      this.ballDirectionX = Math.random() < 0.5 ? 1 : -1;
      this.ballDirectionY = Math.random() < 0.5 ? 1 : -1; 
    },
    gameLoop() {
      if (this.leftScore < 3 && this.rightScore < 3) {
        this.movePaddles(); 
        this.moveBall();
        requestAnimationFrame(this.gameLoop);
      }
      else
      {
        console.log(router.currentRoute._value.path );
          if (this.leftScore >= 3 && router.currentRoute._value.path === '/local') {
            alert("Red side has won");
            this.$router.push('/');
            return;
          }
          // Check if right side wins
          if (this.rightScore >= 3 && router.currentRoute._value.path === '/local') {
            alert("Yellow side has won");
            this.$router.push('/');
            return;
          }
      }
    },
      activatePaddleCollisionCooldown() {
    this.paddleCollisionCooldown = true;
    setTimeout(() => {
      this.paddleCollisionCooldown = false;
    }, 500);
  },
  },
}; 
</script>


<style scoped>
.game-container {
  position: relative;
  width: 1600px; 
  height: 900px;
  margin: 0 auto;
  border: 1px solid #000000;
  overflow: hidden;
  border-radius: 20px; 
  box-shadow: 0 0 10px 5px #00f0ff; 
  animation: neonGlow 6s infinite;
  z-index: 1;
  position: relative;
}


.rightPaddle {
  position: absolute;
  width: 30px;
  height: 120px;
  right: 0;
  top: 50%;
  transform: translateY(-50%);
  border-radius: 20px;
  box-shadow: 0 0 10px yellow, 0 0 20px yellow, 0 0 30px yellow;
}

.leftPaddle {
  position: absolute;
  width: 30px;
  height: 120px;
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
  color: white;
}
.ball {
  position: absolute;
  width: 20px;
  height: 20px;
  background-color: #fff; /* White color for the ball */
  border-radius: 50%;
  box-shadow: 0 0 10px #fff, 0 0 20px #fff, 0 0 30px #fff;
}
.start-button {
  height: 100px;
  font-size: 35px;
  font-family: 'neuropol', sans-serif;
  color: white;
  background-color: transparent;
  text-align: center;
  display: flex; 
  align-items: center;
  justify-content: center;
  margin: 50vh auto 0;
  box-shadow: none;
  cursor: pointer; /* Add pointer cursor */
}

.start-button:focus {
  outline: none; /* Remove default focus outline */
}

</style>
