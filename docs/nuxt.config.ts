export default defineNuxtConfig({
  app: {
    head: {
      htmlAttrs: {
        lang: 'en',
      },
      link: [
        {
          href: 'https://mhouge.dk/apple-touch-icon.png',
          rel: 'apple-touch-icon',
          sizes: '180x180',
        },
        {
          href: 'https://mhouge.dk/favicon-32x32.png',
          rel: 'icon',
          sizes: '32x32',
          type: 'image/png',
        },
        {
          href: 'https://mhouge.dk/favicon-16x16.png',
          rel: 'icon',
          sizes: '16x16',
          type: 'image/png',
        },
        {
          href: 'https://mhouge.dk/site.webmanifest',
          rel: 'manifest',
        },
        {
          color: '#5bbad5',
          href: 'https://mhouge.dk/safari-pinned-tab.svg',
          rel: 'mask-icon',
        },
        {
          href: 'https://mhouge.dk/favicon.ico',
          rel: 'shortcut icon',
          type: 'image/x-icon',
        },
      ],
      meta: [
        {
          content: '#da532c',
          name: 'msapplication-TileColor',
        },
        {
          content: '#ffffff',
          name: 'theme-color',
        },
      ],
    },
  },

  devtools: { enabled: true },

  modules: ['@nuxt/image', '@nuxt/content', '@nuxt/eslint'],

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
      langs: ['http', 'sh', 'lua', 'bash', 'powershell', 'zsh', 'fish'],
    },
    experimental: {
      cacheContents: true,
    },
  },
  eslint: {
    checker: true,
    config: {
      stylistic: false,
      typescript: {
        strict: true,
      },
    },
  },
});
