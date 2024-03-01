<script setup>
import Input from "./components/Input.vue";
import List from "./components/List.vue";
import { invoke } from "@tauri-apps/api/tauri";

import { ref } from 'vue'

const items = ref([])
const listIndex = ref(0)
const handleSearch = (content) => {
  invoke("search", { info: content }).then((result) => {
    items.value = result
  })
  listIndex.value = 0
}
const handleInput = (event) => {
  if (items.value.length) {
    switch (event.key) {
      case 'ArrowUp':
        // 向上移动选中项
        listIndex.value = (listIndex.value - 1 + items.value.length) % items.value.length;
        break;
      case 'ArrowDown':
        // 向下移动选中项
        listIndex.value = (listIndex.value + 1) % items.value.length;
        break;
      default:
        // 其他按键操作
        break;
    }
  }
}
const handleSubmit = () => {
  invoke("submit", { objId: items.value[listIndex.value].obj_id });
}
</script>

<template>
  <div class="cont">
    <div>
      <Input @submit="handleSubmit" @select="handleInput" @search="handleSearch" />
    </div>
    <div class="clist">
      <List :items="items" :index="listIndex" />
    </div>
  </div>
</template>

<style scoped>
.cont {
  display: grid;
  grid-row-gap: 1vh;
}

.clist {
  height: 93vh;
}
</style>
