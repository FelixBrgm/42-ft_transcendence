<!-- PongGame.vue -->
<template>
  <div class="game-container" @keydown="handleKeyPress" @keyup="handleKeyRelease" ref="gameContainer" tabindex="0">
    <!-- Score Counter -->
    <div class="score-counter">{{ playerScore }} - {{ enemyScore }}</div>

    <!-- Player and Enemy paddles -->
    <div class="rightPaddle" :style="{ top: rightPosition + 'px' }"></div>
    <div class="leftPaddle" :style="{ top: leftPosition + 'px' }"></div>


    <!-- Ball -->
    <div class="ball" :style="{ top: ballPosition.top + 'px', left: ballPosition.left + 'px' }"></div>
    <div class="start-button" tabindex="0" role="button" @click="startGame" :style="{ pointerEvents: startButtonEnabled ? 'auto' : 'none' }" v-html="textvalue"></div>
  </div>
</template> 

<script>

export default { 
  data() {
    return {
      textvalue: "Start Game", 
      startButtonEnabled: true,
      rightPosition: 450, 
      leftPosition: 450,
      playerScore: 0,  // Initialize player score to 0
      enemyScore: 0,   // Initialize enemy score to 0
      ballPosition: {
        left: 800,
        top: 450,
      },
      isYou: false, 
      websocket: null,
    };
  },
  methods: {
    handleKeyPress(event) {
      // Check for the key code of the 'Up' arrow key (key code 38)
      if (event.keyCode === 38) {
        // Send "u" to the backend
         this.websocket.send("u"); 
         console.log('Key pressed:', event.keyCode);
      }
      if (event.keyCode === 40) {
        // Send "d" to the backend
        this.websocket.send('d');
      }
    },
    handleKeyRelease(event) { 
      // Check for the key code of the 'Up' arrow key (key code 38)
      if (event.keyCode === 38 || event.keyCode === 40) {
        // Send "n" to the backend
        this.websocket.send('n');
      }

    },
    handleWebSocketMessage(message) {
      const parts = message.split(' ');
      if (parts[0] == 'FORMAT:' && parts[1] == '{YOU}') {
        this.isYou = true;
      } 
      if(parts[0] == 'SCR')
      {
        const rest = parts[1].split(':')
        if (rest[0] > rest[1])
        {this.textvalue = "YOU WON"}
        else{this.textvalue = "HEHE YOU LOOSE"} 
        this.playerScore = rest[0];
        this.enemyScore = rest[1];
      }
      if(parts[0] == 'Starting')
      {
        this.textvalue += "<br>Starting game in 3 Seconds";  
      } 
      if(parts[0] == 'END')
      {
      this.startButtonEnabled = true;
      }
      if(parts[0] == 'POS')
      {
        this.textvalue = "";
        this.leftPosition = 840 - parts[1];
        this.rightPosition = 840 - parts[2];
        this.ballPosition.left = parts[3];
        this.ballPosition.top = 900 - parts[4];  
      }

        // Update the colors based on the isYou property
        this.updatePaddleColors();
      },
      updatePaddleColors() {
        const playerPaddle = this.$refs.gameContainer.querySelector('.rightPaddle');
        const enemyPaddle = this.$refs.gameContainer.querySelector('.leftPaddle');

        if (this.isYou) {
          // You are the left paddle (player)
          playerPaddle.style.backgroundColor = 'red';
          enemyPaddle.style.backgroundColor = 'yellow';
          enemyPaddle.style.boxShadow = '0 0 10px yellow, 0 0 20px yellow, 0 0 30px yellow';
          playerPaddle.style.boxShadow = '0 0 10px red, 0 0 20px red, 0 0 30px red'; 
        } else {
          // You are the right paddle (enemy)
          playerPaddle.style.backgroundColor = 'yellow';
          enemyPaddle.style.backgroundColor = 'red';
          playerPaddle.style.boxShadow = '0 0 10px yellow, 0 0 20px yellow, 0 0 30px yellow';
          enemyPaddle.style.boxShadow = '0 0 10px red, 0 0 20px red, 0 0 30px red';
        }
      },
        startGame() {
      // Connect to WebSocket when the button is clicked
      this.websocket = new WebSocket('ws://localhost:8080/game/matchmake');
      this.startButtonEnabled = false;
      this.textvalue = "Waiting for game";
      // Handle WebSocket events
      this.websocket.addEventListener('open', (event) => {
        console.log('WebSocket connection opened:', event);
      });

      this.websocket.addEventListener('message', (event) => {
        console.log('WebSocket message received:', event.data);
        this.handleWebSocketMessage(event.data);
      });

      this.websocket.addEventListener('close', (event) => {
        console.log('WebSocket connection closed:', event); 
      });

      this.websocket.addEventListener('error', (event) => {
        console.error('WebSocket error:', event);
      });
    },

  },
  updated() {
  },
  mounted() {
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


.rightPaddle {
  position: absolute;
  width: 30px;
  height: 120px;
  box-shadow: 0 0 10px yellow, 0 0 20px yellow, 0 0 30px yellow;
  right: 0;
  top: 50%;
  transform: translateY(-50%);
  border-radius: 20px;
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
