import { createStore } from 'vuex';
import axios from 'axios';

const store = createStore({
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
            const response = await axios.get("/api/user", { withCredentials: true });
            commit('setUser', response.data);
          } catch (error) {
            console.error('Error updating user:', error);
          }
        },
      },
    },
    chat: {
      namespaced: true,
      state: {
          chatOpen: false,
      },
      mutations: {
          TOGGLE_CHAT(state) {
              state.chatOpen = !state.chatOpen;
          },
      },
      actions: {
          toggleChat({ commit }) {
              commit('TOGGLE_CHAT');
          }, 
      },
  },
  }, 
});

export default store;
