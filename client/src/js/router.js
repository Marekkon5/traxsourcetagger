import Vue from 'vue';
import VueRouter from 'vue-router';
import Intro from '../views/Intro.vue';
import FolderPicker from '../views/FolderPicker.vue';
import Tags from '../views/Tags.vue';
import Advanced from '../views/Advanced.vue';
import Status from '../views/Status.vue';

Vue.use(VueRouter);

const routes = [
  {
    path: '/',
    name: 'intro',
    component: Intro
  },
  {
    path: '/step1',
    name: 'path',
    component: FolderPicker
  },
  {
    path: '/step2',
    name: 'tags',
    component: Tags
  },
  {
    path: '/step3',
    name: 'advanced',
    component: Advanced
  },
  {
    path: '/status',
    name: 'status',
    component: Status
  }
];

export default new VueRouter({
  routes
});