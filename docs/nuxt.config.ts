export default defineNuxtConfig({
  devtools: { enabled: true },

  modules: ['@nuxt/image', '@nuxt/content'],

  image: {
    domains: ['mhouge.dk', 'hitt.mhouge.dk'],
    provider: 'ipx',
    presets: {
      default: {
        modifiers: {
          format: 'webp',
          quality: '80',
        },
      },
      svg: {
        modifiers: {
          format: 'svg',
          quality: '100',
        },
      },
    },
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
        default: 'one-dark-pro',
      },
      preload: ['sh', 'lua'],
    },
    experimental: {
      cacheContents: true,
    },
  },
});
