<!-- PongGame.vue -->
<template>
<div>
  <div v-if="this.isYou != null" class="playerinfo"> 
    <div class="players-container">
      <div class="profile left-profile">
        <img :src="leftPlayerimg" class="rounded-circle profile-pic">
        <h2 style="padding: 10px;" >{{ this.leftPlayername }} </h2>
        <p>Wins: {{ this.leftPlayerwin }} Losses: {{ this.leftPlayerloss }}</p>
      </div>
      <div v-if="this.$route.query.startTournament || this.$route.query.joinTournament" class="profile right-profile"> Round: {{ this.round }} </div>
      <div class="profile right-profile"> 
        <img :src="rightPlayerimg" class="rounded-circle profile-pic">
        <h2 style="padding: 10px;" >{{ this.rightPlayername }} </h2> 
        <p>Wins: {{ this.rightPlayerwin }} Losses: {{ this.rightPlayerloss }}</p>
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
    <div class="start-button" tabindex="0" role="button" :style="{ pointerEvents: startButtonEnabled ? 'auto' : 'none' }" v-html="textvalue"></div>
  </div>
  <div v-if="showtournament" class="playerinfo">
    <h1>Other ongoing games</h1> 
  </div>
  <div v-if="showtournament" class="playerinfo">
  <div class="players-container">
  <GameInfo
  v-for="(game, index) in games"
  :key="index"
  :leftPlayer="game.leftPlayer" 
  :rightPlayer="game.rightPlayer"
  />
  </div>
  </div>  
</div> 
</template> 

<script> 

import router from '../../router.js';
import store from '../../store';
import axios from 'axios';
import GameInfo from "@/components/elements/GameInfo.vue";

export default {
  components: {
    GameInfo,
  },
  data() {
    return {  
      games: [],
      manclose: false,
      matchreset: false,
      msgrcvd: false,
      textvalue: "Start Game",
      showtournament: false , 
      startButtonEnabled: true,
      rightPosition: 450,
      leftPosition: 450,
      enemyid: 0,
      enemy: null,
      istournament: false, 
      won: false,
      round: 1, 
      leftScore: 0 ,
      rightScore: 0 , 
      leftPlayerimg: "", 
      rightPlayerimg: "",
      leftPlayername: "" ,
      rightPlayername: "" ,
      leftPlayerwin: 0 ,
      rightPlayerwin: 0 ,
      leftPlayerloss: 0 ,
      rightPlayerloss: 0 ,
      ballPosition: { 
        xaxis: 800,  
        yaxis: 450,
      },
      isYou: null,  
      websocket: null,
    };
  },
  methods: {
    handleKeyPress(event) {
      if (this.websocket && this.websocket.readyState === WebSocket.OPEN)
      {
        if (event.keyCode === 87) {this.websocket.send("u");}
        if (event.keyCode === 83) {this.websocket.send('d');}
      }
    },
    handleKeyRelease(event) { 
      if (this.websocket && this.websocket.readyState === WebSocket.OPEN)
      {
      if (event.keyCode === 87 || event.keyCode === 83) {this.websocket.send('n');}
      }
    },
    handleWebSocketMessage(message) {
      const parts = message.split(' ');
      this.startButtonEnabled = false;
      if (parts[0] == 'FORMAT:' && parts[1] == 'YOU') {
        this.isYou = true;
        this.enemyid = parts[2];
        this.updatePaddleColors();
      }
      else if (parts[0] == 'FORMAT:' && parts[2] == 'YOU') {
        this.isYou = false;
        this.enemyid = parts[1];
        this.updatePaddleColors();
      }  
      if (parts[0] == 'MATCH')
      {
        if(this.matchreset == true)
          this.games = [];
        this.games.push({ leftPlayer: parts[1], rightPlayer: parts[2] });
      }
      if (parts[0] == 'SCR')
      {
        const rest = parts[1].split(':')
        if ((rest[0] > this.leftScore && this.isYou == true) ||
        (rest[0] == this.leftScore && this.isYou == false))
        {this.textvalue = "YOU WON"; this.won = true;}
        else if((rest[0] == this.leftScore && this.isYou == true) ||
        (rest[0] != this.leftScore && this.isYou == false))
        {this.textvalue = "HEHE YOU LOOSE"; this.won = false;} 
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
        this.matchreset = true;
          if (this.won == false){
            this.$router.push('/'); 
            alert("Game over you lost");
          }
          else if (this.won == true && this.istournament == true && this.round != 1){
            this.textvalue = "Waiting for next game";
          }
          else{
            alert("Congrats, you won");
            this.$router.push('/');
          }
      }
      if(parts[0] == 'POS')
      {
        this.textvalue = "";
        this.leftPosition = 840 - parts[1];
        this.rightPosition = 840 - parts[2];
        this.ballPosition.xaxis = parts[3];
        this.ballPosition.yaxis = 900 - parts[4];  
      }
      if(parts[0] == 'SZE')
      {
        this.round = parts[1] / 2;
        this.leftScore = 0;
        this.rightScore = 0;
      }
      },
      updatePaddleColors() {
        const playerPaddle = this.$refs.gameContainer.querySelector('.rightPaddle');
        const enemyPaddle = this.$refs.gameContainer.querySelector('.leftPaddle');
        this.leftPlayerimg = null;
        this.leftPlayername = null;
        this.leftPlayerwin = null;
        this.leftPlayerloss = null;
        this.rightPlayerimg = null;
        this.rightPlayername = null;
        this.rightPlayerwin = null;
        this.rightPlayerloss = null;
        if (this.isYou) {
          playerPaddle.style.backgroundColor = 'red';
          enemyPaddle.style.backgroundColor = 'yellow';
          enemyPaddle.style.boxShadow = '0 0 10px yellow, 0 0 20px yellow, 0 0 30px yellow';
          playerPaddle.style.boxShadow = '0 0 10px red, 0 0 20px red, 0 0 30px red'; 
          this.leftPlayerimg = store.state.auth.user.avatar;
          this.leftPlayername = store.state.auth.user.alias;
          this.leftPlayerwin = store.state.auth.user.wins ;
          this.leftPlayerloss = store.state.auth.user.losses ;
        } else {
          playerPaddle.style.backgroundColor = 'yellow';
          enemyPaddle.style.backgroundColor = 'red';
          playerPaddle.style.boxShadow = '0 0 10px yellow, 0 0 20px yellow, 0 0 30px yellow';
          enemyPaddle.style.boxShadow = '0 0 10px red, 0 0 20px red, 0 0 30px red';
          this.rightPlayerimg = store.state.auth.user.avatar;
          this.rightPlayername = store.state.auth.user.alias;
          this.rightPlayerwins = store.state.auth.user.wins ;
          this.rightPlayerloss = store.state.auth.user.losses ;
        }
        if (this.enemy === null)
        {
          axios.get(`/api/user/${this.enemyid}`, { withCredentials: true })
            .then(response => {
              this.enemy = response.data;
              if (this.isYou) {
                playerPaddle.style.backgroundColor = 'red';
                enemyPaddle.style.backgroundColor = 'yellow';
                enemyPaddle.style.boxShadow = '0 0 10px yellow, 0 0 20px yellow, 0 0 30px yellow';
                playerPaddle.style.boxShadow = '0 0 10px red, 0 0 20px red, 0 0 30px red'; 
                this.leftPlayerimg = store.state.auth.user.avatar;
                this.leftPlayername = store.state.auth.user.alias; 
                this.leftPlayerwin = store.state.auth.user.wins ;
                this.leftPlayerloss = store.state.auth.user.losses ;
                this.rightPlayerimg = this.enemy.avatar; 
                this.rightPlayername = this.enemy.alias;
                this.rightPlayerwin = this.enemy.wins;
                this.rightPlayerloss = this.enemy.losses;
              } else {
                playerPaddle.style.backgroundColor = 'yellow';
                enemyPaddle.style.backgroundColor = 'red';
                playerPaddle.style.boxShadow = '0 0 10px yellow, 0 0 20px yellow, 0 0 30px yellow';
                enemyPaddle.style.boxShadow = '0 0 10px red, 0 0 20px red, 0 0 30px red';
                this.leftPlayerimg = this.enemy.avatar; 
                this.leftPlayername = this.enemy.alias;
                this.leftPlayerwin = this.enemy.wins;
                this.leftPlayerloss = this.enemy.losses;
                this.rightPlayerimg = store.state.auth.user.avatar;
                this.rightPlayername = store.state.auth.user.alias;
                this.rightPlayerwins = store.state.auth.user.wins ;
                this.rightPlayerloss = store.state.auth.user.losses ;
              }
            })
            .catch(error => {
              console.error('Error fetching enemy data:', error);
            });
        }
      },
      startGame(numPlayers, ID) {
      // Connect to WebSocket when the button is clicked
      this.startButtonEnabled = false;
      const userId = store.state.auth.user.id;
      const token = store.state.auth.user.token; 
      let websocketUrl = "";
      if (numPlayers === -1)
      {
        websocketUrl = `wss://${process.env.VUE_APP_IP}/api/game/matchmake/?id=${userId}&token=${token}`;
      }
      else if (numPlayers === -2) 
        websocketUrl = `wss://${process.env.VUE_APP_IP}/api/game/one_vs_one/${ID}?id=${userId}&token=${token}`;
      else  
      {
        this.showtournament = true; 
        websocketUrl = `wss://${process.env.VUE_APP_IP}/api/game/connect_tournament/${numPlayers}?id=${userId}&token=${token}`;
      }
      this.websocket = new WebSocket(websocketUrl);
      this.textvalue = "Waiting for game"; 
      // Handle WebSocket events
      this.websocket.addEventListener('message', (event) => {
        this.msgrcvd = true;
        this.handleWebSocketMessage(event.data);
      });

	// handle if websocket connection failed
        this.websocket.addEventListener('error', (event) => {
        console.error('WebSocket error:', event);
      });

      this.websocket.addEventListener('close', (event) => {
        if (event.code === 1006 && this.msgrcvd != false) {
          console.error('WebSocket closed due to an error');
        } else if (router.currentRoute._value.path === '/pong'){
          // WebSocket closed normally
          if (this.$route.query.joinTournament != 0) {
            this.$router.push('/'); 
            alert("This game does not exist");
          } else if (this.won === false) {
            this.$router.push('/'); 
            alert("You have lost");
          }
        }
      });
    },
    delay(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
    }
  },
    closeWebSocket() {
      if (this.websocket && this.websocket.readyState === WebSocket.OPEN) {
        this.manclose = true;
        this.websocket.close();
        this.websocket = null;
      }
    },
  beforeRouteLeave(to, from, next) {
    this.closeWebSocket();
    next();
  },
  updated() {
  },
  mounted() {
    if (this.$route.query.startGame === 'true') {
      this.startGame(-1);
    }
    if (this.$route.query.joinTournament !== undefined) {
      this.istournament = true;
      this.startGame(this.$route.query.joinTournament);
    }
    if (this.$route.query.joinvs !== undefined) {
      this.startGame(-2, this.$route.query.joinvs); 
    }
    this.$refs.gameContainer.focus();
  },
};
</script>


<style scoped>
.game-container {
  position: relative;
  width: 1600px; 
  height: 900px;
  margin: 0 auto;
  overflow: hidden;
  border-radius: 20px; 
  box-shadow: 0 0 10px 5px #00f0ff; 
  animation: neonGlow 6s infinite;
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