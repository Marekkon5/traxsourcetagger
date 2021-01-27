<template>
<div>

  <vs-checkbox @change='update' v-model='model' :checked=checked>{{title}}</vs-checkbox>

</div>
</template>

<script>
export default {
  name: 'Checkbox',
  data() {
    return {
      model: false
    }
  },
  props: {
    var: String,
    title: String,
    checked: {
      type: Boolean,
      default: false
    }
  },
  methods: {
    //Updating the value based on events, because Edge is shit
    update(e) {
      this.$root.config[this.var] = e.target.checked;
    }
  }
}
</script>