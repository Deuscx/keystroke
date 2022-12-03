<script setup lang="ts">
import { onUnmounted, ref } from "vue";
import { emit, listen } from '@tauri-apps/api/event'

const greetMsg = ref("");
const name = ref("");



// listen to the `click` event and get a function to remove the event listener
// there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
const unlisten = listen('KeyRelease', (event) => {
  // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
  // event.payload is the payload object
  greetMsg.value = event.payload || ''
})

</script>

<template>
  <div data-tauri-drag-region class="titlebar">
    <p>{{ greetMsg }}</p>
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