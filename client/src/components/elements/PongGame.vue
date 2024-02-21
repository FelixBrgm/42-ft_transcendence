<!-- PongGame.vue -->
<template>
<div>
<div v-if="showtournament" class="playerinfo">
  <div class="players-container">
    <div class="profile left-profile">
      <img :src="'../../assets/logo.gif'">
      <h2>Player 1</h2>
      <p>Score: 100</p>
    </div>
    <div class="profile right-profile">
      <img :src="'../../assets/logo.gif'">
      <h2>Player 2</h2>
      <p>Score: 90</p>
    </div>
  </div>
</div>

  <div class="game-container" @keydown="handleKeyPress" @keyup="handleKeyRelease" ref="gameContainer" tabindex="0">
    <!-- Score Counter -->
    <div class="score-counter">{{ leftScore }} - {{ rightScore }}</div>

    <!-- Player and Enemy paddles -->
    <div class="rightPaddle" :style="{ top: rightPosition + 'px' }"></div>
    <div class="leftPaddle" :style="{ top: leftPosition + 'px' }"></div>


    <!-- Ball -->
    <div class="ball" :style="{ top: ballPosition.yaxis + 'px', left: ballPosition.xaxis + 'px' }"></div>
    <div class="start-button" tabindex="0" role="button" @click="startGame" :style="{ pointerEvents: startButtonEnabled ? 'auto' : 'none' }" v-html="textvalue"></div>
  </div>
</div>
</template> 

<script>

import store from '../../store';
import axios from 'axios';


export default { 
  data() {
    return { 
      textvalue: "Start Game",
      showtournament: true,
      startButtonEnabled: true,
      rightPosition: 450,
      leftPosition: 450,
      leftScore: 0 ,
      leftPlayerimg: "", 
      rightPlayerimg: "",
      leftPlayername: 0 ,
      rightPlayername: 0 ,
      leftPlayerscore: 0 ,
      rightPlayerscore: 0 ,
      rightScore: 0 ,
      ballPosition: { 
        xaxis: 800, 
        yaxis: 450,
      },
      isYou: false, 
      websocket: null,
    };
  },
  methods: {
    
    handleKeyPress(event) {
      if (this.websocket && this.websocket.readyState === WebSocket.OPEN)
      {
        if (event.keyCode === 38) {this.websocket.send("u");}
        if (event.keyCode === 40) {this.websocket.send('d');}
      }
    },
    handleKeyRelease(event) { 
      if (this.websocket && this.websocket.readyState === WebSocket.OPEN)
      {
      if (event.keyCode === 38 || event.keyCode === 40) {this.websocket.send('n');}
      }
    },
    handleWebSocketMessage(message) {
      const parts = message.split(' ');
      if (parts[0] == 'FORMAT:' && parts[1] == '{YOU}') {
        this.isYou = true;
        this.startButtonEnabled = false;
      } 
      if(parts[0] == 'SCR')
      {
        const rest = parts[1].split(':')
        if ((rest[0] > this.leftScore && this.isYou == true) ||
        (rest[0] == this.leftScore && this.isYou == false))
        {this.textvalue = "YOU WON";}
        else if((rest[0] == this.leftScore && this.isYou == true) ||
        (rest[0] != this.leftScore && this.isYou == false))
        {this.textvalue = "HEHE YOU LOOSE"} 
        this.leftScore = rest[0];
        this.rightScore = rest[1];
      }
      if (parts[0] == 'Starting') {
        (async () => {
          this.textvalue = "Starting game in 3 Seconds";
          await this.delay(1000);

          this.textvalue = "Starting game in 2 Seconds";
          await this.delay(1000);

          this.textvalue = "Starting game in 1 Second";
        })();
      }
      if(parts[0] == 'END')
      {
        setTimeout(() => {
          this.textvalue = "Start game";
          this.startButtonEnabled = true;
        }, 3000);
      }
      if(parts[0] == 'POS')
      {
        this.textvalue = "";
        this.leftPosition = 840 - parts[1];
        this.rightPosition = 840 - parts[2];
        this.ballPosition.xaxis = parts[3];
        this.ballPosition.yaxis = 900 - parts[4];  
      }

        // Update the colors based on the isYou property
        this.updatePaddleColors();
      },
      updatePaddleColors() {
        const playerPaddle = this.$refs.gameContainer.querySelector('.rightPaddle');
        const enemyPaddle = this.$refs.gameContainer.querySelector('.leftPaddle');
        axios.get(`http://127.0.0.1:8080/user/${userid}`, { withCredentials: true })
          .then(response => {
            const enemy = response.data;
            if (this.isYou) {
              playerPaddle.style.backgroundColor = 'red';
              enemyPaddle.style.backgroundColor = 'yellow';
              enemyPaddle.style.boxShadow = '0 0 10px yellow, 0 0 20px yellow, 0 0 30px yellow';
              playerPaddle.style.boxShadow = '0 0 10px red, 0 0 20px red, 0 0 30px red'; 
              leftPlayerimg = store.state.auth.user.img;
              leftPlayername = store.state.auth.user.name;
              leftPlayerscore = store.state.auth.user.score ;
              rightPlayerimg = enemy.img;
              rightPlayername = enemy.name;
              rightPlayerscore = enemy.score;
            } else {
              playerPaddle.style.backgroundColor = 'yellow';
              enemyPaddle.style.backgroundColor = 'red';
              playerPaddle.style.boxShadow = '0 0 10px yellow, 0 0 20px yellow, 0 0 30px yellow';
              enemyPaddle.style.boxShadow = '0 0 10px red, 0 0 20px red, 0 0 30px red';
              leftPlayerimg = enemy.img;
              leftPlayername = enemy.name;
              leftPlayerscore = enemy.score;
              rightPlayerimg = store.state.auth.user.img;
              rightPlayername = store.state.auth.user.name;
              rightPlayerscore = store.state.auth.user.score ;
            }
          })
          .catch(error => {
            console.error('Error fetching enemy data:', error);
          });
      },
      startGame() {
      // Connect to WebSocket when the button is clicked
      this.startButtonEnabled = false;
      const userId = store.state.auth.user.id;
      const token = store.state.auth.user.password;
      const websocketUrl = `ws://localhost:8080/game/matchmake/?id=${userId}&token=${token}`;
      this.websocket = new WebSocket(websocketUrl);
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

	// handle if websocket connection failed
      this.websocket.addEventListener('error', (event) => {
        console.error('WebSocket error:', event);
      });
    },
      startTournament(numPlayers) {
      // Connect to WebSocket when the button is clicked
      this.startButtonEnabled = false;
      const userId = store.state.auth.user.id;
      const token = store.state.auth.user.password;
      axios.get(`http://127.0.0.1:8080/game/create_tournament/${numPlayers}`, { withCredentials: true });
      const websocketUrl = `ws://localhost:8080/game/connect_tournament/${userId}?id=${userId}&token=${token}`;
      this.websocket = new WebSocket(websocketUrl);
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

	// handle if websocket connection failed
      this.websocket.addEventListener('error', (event) => {
        console.error('WebSocket error:', event);
      });
    },
    delay(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
    }
  },
    closeWebSocket() {
      if (this.websocket && this.websocket.readyState === WebSocket.OPEN) {
        this.websocket.close();
      }
    },
  beforeRouteLeave(to, from, next) {
    // Close WebSocket connection when leaving the route
    this.closeWebSocket();
    next();
  },
  updated() {
  },
  mounted() {
    // Automatically start the game if redirected with the startGame query parameter
    if (this.$route.query.startGame === 'true') {
      this.startGame();
    }
    if (this.$route.query.startTournament !== undefined) {
      this.startTournament(this.$route.query.startTournament);
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

.playerinfo{
  font-family: neuropol;
  background-color: #5c5e5f;
  color: #fff;
  padding: 1rem;
  display: flex;
  border-radius: 20px;
  box-shadow: 0 0 10px 5px #00f0ff;
  animation: neonGlow 6s infinite;
  margin: 30px auto; /* Centering the header */
  max-width: 1600px; 
  min-width: 950px;  
}

.playerinfo {
  display: flex;
  justify-content: center; /* Adjust as needed */
}

.players-container {
  display: flex;
  justify-content: space-between;
  width: 80%; /* Adjust as needed */
}

.profile {
  text-align: center;
}

.left-profile { 
  order: 1; /* Player 1 on the left */
}

.right-profile {
  order: 2; /* Player 2 on the right */
}

</style>

ws.addEventListener('open', onWebSocketOpen);