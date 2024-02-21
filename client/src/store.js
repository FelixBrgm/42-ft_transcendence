import { createStore } from 'vuex';
import axios from 'axios';

const store = createStore( {
	modules: {
    auth: {
      namespaced: true,
      state: {
        user: null,
      },
      mutations: {
        setUser(state, user) {
          state.user = user;
        },
      },
      actions: {
        async updateUser({ commit }) {
          try {
            const response = await axios.get("http://127.0.0.1:8080/user", { withCredentials: true });
            commit('setUser', response.data);
          } catch (error) {
            console.error('Error updating user:', error);
          }
        },
      },
    },
  },
});

export default store;
