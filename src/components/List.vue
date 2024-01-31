<script setup>
const props = defineProps({
  items: {
    type: Array,
    default: []
  },
  index: {
    type: Number,
    default: -1
  }
})
</script>

<template>
    <ul class="main">
      <li v-for="(item, i) in items" :key="item.obj_id" :class="i === index ? 'selected' : 'normal'">
        {{ item.name }}
      </li>
    </ul>
</template>

<style scoped>
li.normal {
  background-color: white;
  list-style-type: none;
  padding: 1vh;
}
li.selected {
  background-color: #ddd;
  list-style-type: none;
  padding: 1vh;
}
ul.main {
  list-style-type: none;
  padding: 0 1vh;
}
</style>
