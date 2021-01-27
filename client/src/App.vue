<template>
  <div id="app" vs-theme='dark'>
    <img :src="require('./assets/traxsource-tagger-logo.png').default" class='logo'>
    <transition name='fade'>
      <router-view class='router' />
    </transition>
  </div>
</template>


<style>
  @import 'style/app.css';
</style>