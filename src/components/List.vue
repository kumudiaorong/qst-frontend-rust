<template>
  <ul id="scroll_ul" class="main">
    <li v-for="(item, i) in items" :key="item.obj_id" :class="i === index ? 'selected' : 'normal'">
      {{ item.name }}
    </li>
  </ul>
</template>

<style scoped>
li.normal {
  background-color: white;
  list-style-type: none;
  padding: 5px;
}

li.selected {
  background-color: #ddd;
  list-style-type: none;
  padding: 5px;
}

ul.main {
  height: 100%;
  list-style-type: none;
  overflow-y: scroll;
  margin: 0vh;
  padding: 0vh 1vh;
  text-align: center;
}
</style>
<script setup>
import { onMounted, watch, ref } from 'vue';

const props = defineProps({
  items: {
    type: Array,
    default: []
  },
  index: {
    type: Number,
    default: 0
  },
})
var last_index = 0;
var scroll_list = null;
watch(() => props.index, (index) => {
  const item = scroll_list.childNodes.item(index + 1);
  if (last_index < index) {
    const expectScrollTop = item.offsetTop - scroll_list.offsetTop + item.clientHeight - scroll_list.clientHeight;
    if (expectScrollTop > scroll_list.scrollTop) {
      scroll_list.scrollTop = expectScrollTop;
    }
  }
  else {
    const expectScrollTop = item.offsetTop - scroll_list.offsetTop;
    if (expectScrollTop < scroll_list.scrollTop) {
      scroll_list.scrollTop = expectScrollTop;
    }
  }
  last_index = index;
  // console.log("rect", scroll_list.getBoundingClientRect());
  // console.log("scroll_list.scrollTop", scroll_list.scrollTop);
  // console.log("scroll_list.clientHeight", scroll_list.clientHeight);
  // console.log("scroll_list.offsetTop", scroll_list.offsetTop);
  // console.log("scroll_list.offsetHeight", scroll_list.offsetHeight);
  // console.log("scroll_list.scrollHeight", scroll_list.scrollHeight);
  // console.log("scroll_list.clientTop", scroll_list.clientTop);
  // console.log("scroll_list.height", scroll_list.height);
  // console.log("item.offsetTop", item.offsetTop);
  // console.log("item.clientHeight", item.clientHeight);
});
onMounted(() => {
  scroll_list = document.getElementById('scroll_ul')
});
</script>
