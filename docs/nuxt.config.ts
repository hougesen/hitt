export default defineNuxtConfig({
  devtools: { enabled: true },

  modules: ['@nuxt/image', '@nuxt/content'],

  image: {
    domains: ['mhouge.dk', 'hitt.mhouge.dk'],
  },
});
