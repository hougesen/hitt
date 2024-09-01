<script setup lang="ts">
const props = defineProps({
  href: {
    type: String,
    default: "",
  },
  blank: {
    type: Boolean,
    default: false,
  },
});

const external = computed(() => {
  try {
    const url = new URL(props?.href);

    return url?.hostname !== "hitt.mhouge.dk";
  } catch {
    return false;
  }
});
</script>

<template>
  <NuxtLink :to="href" :target="external ? '_blank' : undefined">
    <slot />
  </NuxtLink>
</template>
