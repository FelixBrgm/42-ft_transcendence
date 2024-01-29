import { createStore } from 'vuex';

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
        // You can add actions if needed
      },
    },
  },
});

export default store;
