<script setup lang="ts">
import { settings } from '~/composables/settings'
</script>

<template>
  <div class="flex gap-4 flex-col h-100vh bg-#ECEDF9">
    <div class="bg-white p-4 flex flex-col gap-4">
      <h1 class="text-xl font-bold">
        Settings
      </h1>
      <div id="fontSize">
        <div class=" leading-tight">
          Font Size
        </div>
        <div class="flex items-center flex-1 gap-4">
          <n-slider v-model:value="settings.fontSize" :step="1" :min="14" :max="32" />
          <n-input-number v-model:value="settings.fontSize" size="small" />
        </div>
      </div>
      <div id="fontColor">
        <div class=" leading-tight">
          Font Color
        </div>
        <div class="flex items-center flex-1 gap-4">
          <n-color-picker v-model="settings.fontColor" />
        </div>
      </div>
      <div id="KeyLen">
        <div class=" leading-tight">
          Key Length
        </div>
        <div class="flex items-center flex-1 gap-4">
          <n-slider v-model:value="settings.maxKeyLength" :step="1" :min="6" :max="10" />
          <n-input-number v-model:value="settings.maxKeyLength" size="small" />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
</style>
