import { createApp } from 'vue'
import App from './App.vue'

createApp(App).mount('#app')

const address = 'ws://localhost:4242';
const numConnections = 200;

for (let i = 0; i < numConnections; i++) {
  const client = new WebSocket(address);

  client.onopen = () => {
    // console.log("WebSocket connected" + i);
  };  
  client.onmessage = () => {
    // const message = event.data;
    // console.log(message);
    client.send('u');
  };
//   client.on('connect', connection => {
//     console.log('Connected to server');
    
//     // Send messages or perform actions on the server
//     connection.send('Hello, server!');
    
//     // Handle server responses or continue interacting with the server
//     connection.on('message', message => {
//       console.log('Received message:', message.utf8Data);
//     });
//   });
  
//   client.on('connectFailed', error => {
//     console.log('Connection failed:', error.toString());
//   });
  
//   client.connect(address);
}