import Vue from 'vue';
import Vuesax from 'vuesax';
import App from './App.vue';
import router from './js/router';

import 'vuesax/dist/vuesax.css';

Vue.use(Vuesax, {
  colors: {
    primary: '#2291FF',
    warn: '#E88400',
    danger: '#EE1111',
    success: '#00AA09',
  }
});

Vue.config.productionTip = false;

new Vue({
  data() {
    return {
      ws: null,
      path: '',
      config: {
        title: false,
        artists: false,
        genre: true,
        album: true,
        date: true,
        label: true,
        bpm: true,
        key: false,
        separator: ", ",
        fuzziness: 80,
        art: false,
        overwrite: true,
        id3v24: true
      },
      progress: {
        ok: 0,
        fail: 0,
        total: 0,
        eta: 0
      },
      failed: [],
      done: null
    }
  },
  methods: {
    //Websocket send to backend
    send(action, data = {}) {
      this.ws.send(JSON.stringify({
        action,
        ...data
      }));
    },
    //Open external URL
    url(url) {
      this.send('url', {url});
    },
    //Start tagging
    start() {
      this.send('start', {
        path: this.path,
        config: this.config
      });
    }
  },
  created() {
    //Connect Websocket, don't use "localhost" or Edge will shit itself
    this.ws = new WebSocket('ws://127.0.0.1:36910');
    this.ws.addEventListener('message', (event) => {
      if (event.data) {
        let data = JSON.parse(event.data);

        //Update path
        if (data.action == 'path') {
          this.path = data.path;
        }

        //Update progress
        if (data.action == 'progress') {
          this.progress = data.progress;

          if (!data.last.ok) {
            this.failed.push(data.last.path);
          }
        }

        //Done, show stats
        if (data.action == 'done') {
          this.done = data;
          //Add fake "succesful failed"
          if (this.failed.length == 0)
            this.failed.push('All files successful!')
        }

      }
    });
  },

  router,
  render: h => h(App)
}).$mount('#app');
