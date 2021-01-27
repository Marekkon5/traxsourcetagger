<template>
<div>
  <h1>Advanced</h1>

  <div class='option'>
    <span class='input-title'>Artist Separator: </span>
    <vs-input v-model='$root.config.separator' primary class='vs-input-edit'></vs-input>
  </div>

  <div class='option'>
    <span class='input-title'>Strictness (%): </span>
    <vs-input v-model='$root.config.fuzziness' primary type='number' class='vs-input-edit'></vs-input>
  </div>

  <div class='option'>
    <Checkbox var='overwrite' title='Overwrite tags' checked></Checkbox>
  </div>

  <div class='option'>
    <span class='id3version'>ID3 Version:</span>
    <vs-switch v-model='$root.config.id3v24' @change='id3change'>
      <template #off>
        ID3v2.3
      </template>
      <template #on>
        ID3v2.4
      </template>
    </vs-switch>
  </div>

  <div class='start'>
    <vs-button @click='start' size='l' success gradient>START</vs-button>
  </div>


</div>
</template>

<script>
import router from '../js/router';
import Checkbox from '../components/Checkbox.vue';

export default {
  name: 'Advanced',
  components: {Checkbox},
  methods: {
    start() {
      this.$root.start();
      router.push('status')
    },
    //Another stupid wrapper because Edge cannot cooperate
    id3change(e) {
      this.$root.config.id3v24 = e.target.checked;
    }
  }
}
</script>

<style>
.option {
  display: flex;
  justify-content: center;
  margin-bottom: 8px;
  padding-top: 5px;
  padding-right: 6px;
}

.option-edit {
  display: flex;
  justify-content: center;
  margin-bottom: 8px;
  padding-top: 8px;
  padding-left: 5px;
}

.input-title {
  font-size: 14px;
  padding-left: 8px;
  padding-right: 8px;
  padding-top: 8px;
}

.id3version {
  padding-right: 8px;
  padding-top: 4px;
  font-size: 14px;
}

.start {
  font-family: 'Oswald', sans-serif;
  position: absolute;
  display: flex;
  justify-content: center;
  width: 100%;
  bottom: 50px;
}

.vs-input-edit {
   width: 200px;
}
</style>