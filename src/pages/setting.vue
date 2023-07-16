<script setup lang="ts">
import { settings } from '~/composables/settings'
</script>

<template>
  <div class="flex gap-4 flex-col h-100vh bg-#cfcfcf/50">
    <div class="bg-white p-4 flex flex-col gap-4">
      <h1 class="text-xl">
        Settings
      </h1>
      <div id="fontSize">
        <div class="text-slate-900 leading-tight">
          Font Size
        </div>
        <div class="flex items-center flex-1">
          <input v-model="settings.fontSize" class="flex-1" type="range" min="16" max="32">
          <div>{{ `${settings.fontSize}px` }}</div>
        </div>
      </div>
      <div id="fontColor">
        <div class="text-slate-900 leading-tight">
          Font Color
        </div>
        <div class="flex items-center flex-1">
          <input v-model="settings.fontColor" type="color">
        </div>
      </div>
      <div id="KeyLen">
        <div class="text-slate-900 leading-tight">
          Key Length
        </div>
        <div class="flex items-center flex-1">
          <input v-model="settings.maxKeyLength" type="range" class="flex-1" min="6" max="10">
          <div>{{ settings.maxKeyLength }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped></style>
SliderRoot
