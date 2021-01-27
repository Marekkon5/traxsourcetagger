<template>
<div>
  
  <h1>Select folder</h1>

  Supported formats: <b>MP3, FLAC, AIFF</b>
  <br>
  Subdirectories are included.
  <br>

  <div class='path-container'>
    <vs-input placeholder="Path" v-model='$root.path' primary class='path-input'></vs-input>
    <vs-button @click='browse' gradient>BROWSE</vs-button>
  </div>
  
  <div class='next' v-if='$root.path'>
    <vs-button @click='next' size='l' gradient>NEXT</vs-button>
  </div>

</div>
</template>

<script>
import router from '../js/router';

export default {
  name: 'FolderPicker',
  methods: {
    browse() {
      this.$root.send('browse');
    },
    next() {
      //Validate path
      this.$root.ws.addEventListener('message', (event) => {
        if (event.data) {
          let data = JSON.parse(event.data);
          if (data.action == 'validate') {
            if (data.status) {
              router.push('step2');
            } else {
              alert('Invalid path!');
            }
          }
        }
      });

      this.$root.send('validate', {path: this.$root.path});
    }
  },
}
</script>

<style>
.path-container {
  padding-top: 16px;
  display: flex;
  justify-content: center;
}
.path-input {
  margin-right: 8px;
}
</style>