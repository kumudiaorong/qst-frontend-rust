<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const content = ref("");
const emit = defineEmits(['submit', 'select', 'search'])

async function submit() {
  emit('submit')
}
async function input() {
  emit('search', content.value)
}
function trans(event) {
  switch (event.key) {
    case 'ArrowUp':
    case 'ArrowDown':
      // 向下移动选中项
      emit('select', event)
      event.preventDefault(); // 阻止默认下移操作
      break;
    default:
      // 其他按键操作
      break;
  }
}
</script>

<template>
  <form @submit.prevent="submit">
    <input id="top-input" v-model="content" @input="input" @keydown="trans" placeholder="Enter something...">
    <!-- <button type="submit">Greet</button> -->
  </form>
  <!-- <p>{{ result }}</p> -->
</template>


<style scoped>
#top-input {
  border-color: blue;
  padding: 1vh;
  width: 100%;
  height: 100%;
  box-sizing: border-box;
}
</style>