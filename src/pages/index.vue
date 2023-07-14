<script setup lang="ts">
import { listen } from '@tauri-apps/api/event'

const specialKeysMap = {
  Enter: 'i-mdi:keyboard-return',
  Escape: 'i-mdi:keyboard-esc',
  Tab: 'i-mdi:keyboard-tab',
  ShiftLeft: 'i-mdi:apple-keyboard-shift',
  Backspace: 'i-mdi:keyboard-backspace',
  ControlLeft: 'i-mdi:apple-keyboard-control',
  DownArrow: 'i-mdi:arrow-down',
  LeftArrow: 'i-mdi:arrow-left',
  RightArrow: 'i-mdi:arrow-right',
  UpArrow: 'i-mdi:arrow-up',
  MetaLeft: 'i-mdi:apple-keyboard-command',
  // "Space": "i-mdi:keyboard-space"
  Space: ' ',
}

const specialKeys = Object.keys(specialKeysMap)

const pressedKeys = ref([])
const needRefreshKeys = ['ShiftLeft', 'ControlLeft', 'MetaLeft', 'AltLeft']
let disposeFn: Function[] = []
const maxKeys = 8
onMounted(async () => {
  // listen to the `click` event and get a function to remove the event listener
  // there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
  const unlisten = await listen('KeyPress', (event) => {
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
    // console.log(event.event, event.payload)
    if (pressedKeys.value.length >= maxKeys)
      pressedKeys.value.shift()
    if (needRefreshKeys.includes(event.payload))
      pressedKeys.value = []
    pressedKeys.value.push(event.payload)
  })
  disposeFn.push(unlisten)
})

tryOnUnmounted(() => {
  disposeFn.forEach(fn => fn())
  disposeFn = []
})
</script>

<template>
  <div data-tauri-drag-region class="h-100vh overflow-hidden bg-#333/50 grid place-items-center rounded text-xl">
    <div class="flex gap-1 items-center select-none text-white" data-tauri-drag-region>
      <div v-for="(key, index) in pressedKeys" :key="key + index" data-tauri-drag-region>
        <div v-if="specialKeys.includes(key)">
          <div :class="specialKeysMap[key]" data-tauri-drag-region />
        </div>
        <div v-else data-tauri-drag-region>
          {{ key }}
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped></style>
