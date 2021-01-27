<template>
<div>

  <img :src="require('@/assets/traxsource-icon.png').default">
  <h1>Easy app to automatically update your<br>audio tags & cover from Traxsource</h1>
  <br><br>
  <div class='repository'>
    <vs-button  @click="$root.url('https://github.com/marekkon5/traxsourcetagger')" gradient>REPOSITORY</vs-button>
  </div>

  <p>Developer: Marekkon5<br>
  GUI, icon, video: Bas Curtiz</p>

  <div class='next'>
    <vs-button @click='next' size='l' gradient>NEXT</vs-button>
  </div>

</div>
</template>

<script>
import router from '../js/router.js';

export default {
  name: 'Intro',
  methods: {
    next() {
      router.push('/step1')
    }
  }
}
</script>
