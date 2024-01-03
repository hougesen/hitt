<script setup lang="ts">
const { toc } = useContent();
</script>

<template>
  <div v-if="toc?.links?.length" class="rounded-lg border-2">
    <TocList :links="toc?.links || []" />
  </div>
</template>
