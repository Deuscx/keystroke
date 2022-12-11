<script setup lang="ts">
import { onUnmounted, ref } from "vue";
import { emit, listen } from '@tauri-apps/api/event'
import { tryOnBeforeUnmount, useDebounceFn } from '@vueuse/core';


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
  "MetaLeft": 'i-mdi:apple-keyboard-command',
  "Space": "i-mdi:keyboard-space"
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
    <div data-tauri-drag-region class="typing-keys">
      <div v-for="(key, index) in typeKeys" :key="index">
        <div v-if="iconsMap[key]" class="key" :class="[iconsMap[key]]"> </div>
        <div v-else class="key"> {{ key }}</div>
      </div>
    </div>

    <div data-tauri-drag-region class="specific-keys">
      <div class="key specific pressed">
        <div class=" i-mdi:apple-keyboard-shift  "></div>
      </div>
      <div class="key specific">
        <div class=" i-mdi:apple-keyboard-control "></div>
      </div>
      <div class="key specific">
        <div class="i-mdi:apple-keyboard-option "></div>
      </div>
      <div class="key specific">
        <div class="i-mdi:apple-keyboard-command "></div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.key {
  @apply font-mono px-1 py-1 rounded text-white justify-center display-flex;
}

.typing-keys {
  @apply flex justify-end items-center;
  height: calc(100% - 35px);
}

.specific-keys {
  @apply flex items-center w-full justify-around position-absolute bottom-0 h-35px;
}

.key.specific {
  @apply py-2;
  color: gray;
  box-sizing: border-box;
  border-radius: 0;
  flex: 1;
}

.key.specific.pressed {
  @apply text-white;
}

.titlebar {
  @apply bg-#000 opacity-90 h-full rounded-3;
  user-select: none;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
}
</style>