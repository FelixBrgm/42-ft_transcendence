<template>
  <api-component></api-component>
</template>

<script>
import ApiComponent from './components/ApiComponent.vue'

export default {
  name: 'App',
  components: {
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
