import { createStore } from 'vuex';
import axios from 'axios';

const store = createStore({
  modules: {
    auth: {
      namespaced: true,
      state: {
        user: null,
        login: null,
      },
      mutations: {
        setUser(state, user) {
          state.user = user;
        },
        setLogin(state, value) {
          state.login = value;
        }
      },
      actions: {
        async updateUser({ commit }) {
          try {
            const response = await axios.get("/api/user", { withCredentials: true });
            commit('setUser', response.data);
          } catch (error) {
            // console.error('Error updating user:', error);
          }
        },
        // Add an action to update login
        updateLogin({ commit }, value) {
          console.log("store commited");
          commit('setLogin', value);
        }
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
