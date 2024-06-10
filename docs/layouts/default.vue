<script setup lang="ts">
const { page, next, prev } = useContent();

useContentHead(page);
</script>

<template>
  <article class="prose flex-col lg:prose-xl">
    <h1>
      {{ page.title }}
    </h1>

    <TableOfContents />

    <ContentRenderer :key="page._id" class="flex-col" :value="page" />

    <div class="flex w-full gap-4">
      <NuxtLink v-if="prev?._path" :to="prev._path"
        >&lt; {{ prev.title }}</NuxtLink
      >

      <NuxtLink v-if="next?._path" class="ml-auto" :to="next._path"
        >{{ next.title }} &gt;</NuxtLink
      >
    </div>
  </article>
</template>
