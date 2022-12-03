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
  <p>{{ greetMsg }}</p>
</template>
