<template>
  <api-component></api-component>
  <div id="app">
    <WebSocket />
  </div>
</template>

<script>
import ApiComponent from './components/ApiComponent.vue'
import WebSocket from './components/WebSocket.vue';

export default {
  name: 'App',
  components: {
	WebSocket,
	ApiComponent
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
