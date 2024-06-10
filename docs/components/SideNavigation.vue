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

        <li class="text-lg font-bold mt-2">
          <NuxtLink target="_blank" to="https://crates.io/crates/hitt">
            <img src="https://img.shields.io/crates/v/hitt.svg" />
          </NuxtLink>
        </li>

        <li class="text-lg font-bold mt-2">
          <NuxtLink
            target="_blank"
            to="https://github.com/hougesen/hitt/actions/workflows/validate.yml"
          >
            <img
              src="https://github.com/hougesen/hitt/actions/workflows/validate.yml/badge.svg"
            />
          </NuxtLink>
        </li>

        <li class="text-lg font-bold mt-2">
          <NuxtLink target="_blank" to="https://codecov.io/gh/hougesen/hitt">
            <img
              src="https://codecov.io/gh/hougesen/hitt/branch/main/graph/badge.svg"
            />
          </NuxtLink>
        </li>
      </ul>
    </ContentNavigation>
  </nav>
</template>
