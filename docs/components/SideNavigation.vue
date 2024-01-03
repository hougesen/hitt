<script setup lang="ts">
const route = useRoute();
</script>

<template>
  <nav>
    <ContentNavigation v-slot="{ navigation }">
      <ul>
        <li
          v-for="link of navigation"
          :key="link._path"
          class="text-lg font-bold"
        >
          <NuxtLink
            :to="link._path"
            :class="{
              'text-blue-400': route?.path === link?._path,
            }"
          >
            {{ link.title }}
          </NuxtLink>
        </li>

        <li class="mt-4 text-lg font-bold">
          <NuxtLink to="https://github.com/hougesen/hitt" target="_blank">
            GitHub
          </NuxtLink>
        </li>
      </ul>
    </ContentNavigation>
  </nav>
</template>
