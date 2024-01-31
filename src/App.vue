<script setup>
import Input from "./components/Input.vue";
import List from "./components/List.vue";
import { ref } from 'vue'

const items = ref('')
const selectedIndex = ref(0)
const handleinput = (event) => {
  switch (event.key) {
    case 'ArrowUp':
      // 向上移动选中项
      selectedIndex.value = (selectedIndex.value - 1 + items.value.length) % items.value.length;
      break;
    case 'ArrowDown':
      // 向下移动选中项
      selectedIndex.value = (selectedIndex.value + 1) % items.value.length;
      break;
    default:
      // 其他按键操作
      break;
  }
}
</script>

<template>
    <Input @list="(msg) => items = msg" @select="handleinput" class="cinput"/>
    <List :items="items" :index="selectedIndex" />
</template>

<style scoped>
.cinput {
  padding: 1vh;
  display: flex;
  flex-direction: column;
  gap: 10vh;
}
</style>
