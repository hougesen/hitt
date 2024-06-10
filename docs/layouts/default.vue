<script setup>
const { page, next, prev } = useContent();

useContentHead(page);
</script>

<template>
  <article class="prose lg:prose-xl flex-col">
    <h1>
      {{ page.title }}
    </h1>

    <TableOfContents />

    <ContentRenderer
      :key="page._id"
      :value="page"
      class="flex-col"
      style="flex-direction: column !important"
    />

    <div class="flex w-full gap-4">
      <NuxtLink v-if="prev?._path" :to="prev._path"
        >&lt; {{ prev.title }}</NuxtLink
      >

      <NuxtLink v-if="next?._path" :to="next._path" class="ml-auto"
        >{{ next.title }} &gt;</NuxtLink
      >
    </div>
  </article>
</template>
