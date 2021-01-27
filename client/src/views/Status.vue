<template>
<div>

  <!-- Not done -->
  <div v-if='!done'>
    <h1>Progress</h1>
    <div class='container'>
      <vs-button success size='l' gradient>
        <span class='mdi mdi-check'></span>
        <span class='span'>{{$root.progress.ok}}</span>
      </vs-button>

      <vs-button danger size='l' gradient>
        <span class='mdi mdi-close'></span>
        <span class='span'>{{$root.progress.fail}}</span>
      </vs-button>

      <vs-button warn size='l' gradient>
        <span class='mdi mdi-format-list-bulleted'></span>
        <span class='span'>{{remaining}}</span>
      </vs-button>

      <vs-button primary size='l' gradient>
        <span class='mdi mdi-timelapse'></span>
        <span class='span'>{{duration($root.progress.eta)}}</span>
      </vs-button>
    </div>
  </div>

  <!-- Done -->
  <div v-if='done'>
    <h1>Done</h1>
    <div class='container'>
      <vs-button success size='l' gradient>
        <span class='mdi mdi-check'></span>
        <span class='span'>{{$root.done.ok}}</span>
      </vs-button>

      <vs-button danger size='l' gradient>
        <span class='mdi mdi-close'></span>
        <span class='span'>{{$root.done.fail}}</span>
      </vs-button>

      <vs-button warn size='l' gradient>
        <span class='mdi mdi-format-list-bulleted'></span>
        <span class='span'>{{$root.done.total}}</span>
      </vs-button>

      <vs-button primary size='l' gradient>
        <span class='mdi mdi-timelapse'></span>
        <span class='span'>{{duration($root.done.took)}}</span>
      </vs-button>
    </div>
  </div>

  <h3 class='failed-title'>Failed:</h3>
  <div class='failed' :class="[done ? 'failed-small' : 'failed-tall']">
    <pre><span v-for='(f, i) in $root.failed' :key='i'>{{f.trim()}}<br></span></pre>
  </div>


  <!-- Actiosn when done -->
  <div class='done-actions'>
    <vs-button danger size='l' @click='exit' gradient>
      EXIT
    </vs-button>
  </div>

</div>
</template>

<script>
export default {
  name: 'Status',
  methods: {
    //Seconds to duration string
    duration(s) {
      return `${Math.floor(s / 60).toString()}:${Math.floor(s % 60).toString().padStart(2, '0')}`;
    },
    exit() {
      this.$root.send('exit');  
    }
  },
  computed: {
    //Remaining items
    remaining() {
      return this.$root.progress.total - (this.$root.progress.ok + this.$root.progress.fail);
    },
    done() {
      if (this.$root.done == null) {
        return false;
      }
      return true;
    },
  }
}
</script>

<style>
.container {
  display: flex;
  justify-content: center;
}

.failed {
  overflow-y: auto;
  background: #141417;
  text-align: left;
  padding-left: 8px;  
  overflow-x: auto;
}

.failed-title {
  margin-bottom: 8px;
}

.failed-tall {
  height: 350px;
  margin-bottom: 10px;
}

.failed-small {
  animation-name: shrink;
  animation-duration: 1s;
  height: 256px;
  margin-bottom: 10px;
}

/* Shrink failed list when done */
@keyframes shrink {
  from {
    height: 350px;
  }
  to {
    height: 256px;
  }
}

.done-actions {
  font-family: 'Oswald', sans-serif;
  display: flex;
  justify-content: center;
}
</style>