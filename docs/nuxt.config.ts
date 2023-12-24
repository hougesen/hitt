export default defineNuxtConfig({
  devtools: { enabled: true },

  modules: ['@nuxt/image', '@nuxt/content'],

  image: {
    domains: ['mhouge.dk', 'hitt.mhouge.dk'],
  },

  postcss: {
    plugins: {
      tailwindcss: {},
      autoprefixer: {},
    },
  },

  css: ['~/assets/css/main.css'],

  content: {
    documentDriven: true,

    highlight: {
      theme: {
        default: 'github-dark',
      },
      preload: ['sh', 'lua'],
    },
  },
});
