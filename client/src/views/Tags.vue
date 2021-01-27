<template>
<div>
  
  <h1>Select Tags</h1>
  Select tags which you want to update:
  <br>
  <div class='tagsdiv'>
    <h3>

      <Checkbox var='title' title='Title' class='grid'></Checkbox>
      <Checkbox var='artists' title='Artists' class='grid'></Checkbox>
      <Checkbox var='album' title='Album' class='grid'></Checkbox>
      <Checkbox var='label' title='Label' class='grid'></Checkbox>
      <Checkbox var='date' title='Date' class='grid'></Checkbox>
      <Checkbox var='genre' title='Genre' class='grid' checked></Checkbox>
      <Checkbox var='bpm' title='BPM' class='grid' checked></Checkbox>
      <Checkbox var='key' title='Key' class='grid' checked></Checkbox>

      <Checkbox var='art' title='Album Art' class='art'></Checkbox>

    </h3>
  </div>

  <div class='next'>
    <vs-button @click='next' size='l' gradient>NEXT</vs-button>
  </div>

</div>
</template>

<script>
import router from '../js/router.js';
import Checkbox from '../components/Checkbox.vue';

export default {
  name: 'Tags',
  components: {Checkbox},
  methods: {
    next() {
      router.push('step3')
    }
  }
}
</script>

<style>
.grid {
  margin-top: -20px;
  padding: 20px;
  justify-content: center;
  float: left;
  width: 100px;  
}

.art {    
  display: flex;
  justify-content: center;
  padding-right: 184px;
}

.tagsdiv {
  padding-top: 10px;
  padding-left: 70px;
}
</style>