export default defineNuxtConfig({
  app: {
    head: {
      htmlAttrs: {
        lang: "en",
      },
      link: [
        {
          href: "https://mhouge.dk/apple-touch-icon.png",
          rel: "apple-touch-icon",
          sizes: "180x180",
        },
        {
          href: "https://mhouge.dk/favicon-32x32.png",
          rel: "icon",
          sizes: "32x32",
          type: "image/png",
        },
        {
          href: "https://mhouge.dk/favicon-16x16.png",
          rel: "icon",
          sizes: "16x16",
          type: "image/png",
        },
        {
          href: "https://mhouge.dk/site.webmanifest",
          rel: "manifest",
        },
        {
          color: "#5bbad5",
          href: "https://mhouge.dk/safari-pinned-tab.svg",
          rel: "mask-icon",
        },
        {
          href: "https://mhouge.dk/favicon.ico",
          rel: "shortcut icon",
          type: "image/x-icon",
        },
      ],
      meta: [
        {
          content: "#da532c",
          name: "msapplication-TileColor",
        },
        {
          content: "#ffffff",
          name: "theme-color",
        },
      ],
    },
  },

  content: {
    documentDriven: true,
    experimental: {
      cacheContents: true,
    },
    highlight: {
      langs: ["http", "sh", "lua", "bash", "powershell", "zsh", "fish"],
      theme: {
        default: "one-dark-pro",
      },
    },
  },

  css: ["~/assets/css/main.css"],

  devtools: {
    enabled: true,
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

  image: {
    domains: ["mhouge.dk", "hitt.mhouge.dk"],
    provider: "ipxStatic",
  },

  modules: ["@nuxt/image", "@nuxtjs/sitemap", "@nuxt/content", "@nuxt/eslint"],

  nitro: {
    minify: true,
    prerender: {
      crawlLinks: true,
      routes: ["/", "/sitemap.xml"],
    },
  },

  postcss: {
    plugins: {
      autoprefixer: {},
      tailwindcss: {},
    },
  },

  routeRules: {
    "/": {
      prerender: true,
    },
  },

  site: {
    indexable: true,
    url: "https://hitt.mhouge.dk",
  },

  sitemap: {
    cacheMaxAgeSeconds: 3600,
    credits: false,
    discoverImages: true,
    enabled: true,
  },

  telemetry: false,
});
