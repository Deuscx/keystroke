<script setup lang="ts">
import { onUnmounted, ref } from "vue";
import { emit, listen } from '@tauri-apps/api/event'
import {  tryOnBeforeUnmount, useDebounceFn, useThrottleFn } from '@vueuse/core';


const iconsMap = {
  "Enter": "i-mdi:keyboard-return",
  "Escape": "i-mdi:keyboard-esc",
  "Tab": "i-mdi:keyboard-tab",
  "ShiftLeft": "i-mdi:apple-keyboard-shift",
  "Backspace": "i-mdi:keyboard-backspace",
  "ControlLeft": "i-mdi:apple-keyboard-control",
  "DownArrow": "i-mdi:arrow-down",
  "LeftArrow": "i-mdi:arrow-left",
  "RightArrow": "i-mdi:arrow-right",
  "UpArrow": "i-mdi:arrow-up",
} as Record<string, any>


const typeKeys = ref([""]);

(async () => {
  const unListenKeyRelease = await listen('KeyRelease', useDebounceFn((event) => {
    console.log("Release ~ event", event)
    typeKeys.value = []
  }, 1000))

  const unListenKeyPress = await listen('KeyPress', useDebounceFn((event) => {
    console.log("🚀 ~ file: Index.vue:24 ~ unListenKeyRelease ~ event", event.payload)
    if (!event.payload) return
    typeKeys.value.push(event.payload)
  }, 100))

  tryOnBeforeUnmount(() => {
    console.log("unmount")
    unListenKeyRelease()
    unListenKeyPress()
  })
})()



</script>

<template>
  <div data-tauri-drag-region class="titlebar">
    <div v-for="(key, index) in typeKeys" :key="index">
      <div v-if="iconsMap[key]" :class="[iconsMap[key]]"></div>
      <div v-else>{{ key }}</div>
    </div>


    <a class="i-mdi:keyboard-esc"></a>
  </div>
</template>

<style scoped>
.titlebar {
  height: 30px;
  background: #329ea3;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
}
</style>