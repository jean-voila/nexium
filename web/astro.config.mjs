import { defineConfig } from 'astro/config';

import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
  site: 'https://nexium-jean-herail-ff3bbec6f6f2d53c3b619fbb2bb7f3a70234886c751d.pages.epita.fr/',
  base: '/web',
  outDir: 'public',
  publicDir: 'static',
  integrations: [
    starlight({
      title: "Nexium",
    }),
  ],
});
