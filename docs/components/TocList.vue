<script setup lang="ts">
type Link = {
  id: string;
  text: string;
  children?: Array<Link>;
};

defineProps<{
  links?: Array<Link>;
}>();
</script>

<template>
  <ul v-if="links?.length">
    <li v-for="link in links" :key="link.text">
      <a :href="`#${link.id}`">{{ link.text }}</a>

      <LazyTocList v-if="link.children" :links="link?.children || []" />
    </li>
  </ul>
</template>
