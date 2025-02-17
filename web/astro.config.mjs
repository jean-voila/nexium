import { defineConfig } from 'astro/config';

import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
  site: 'https://jean.herail.gitlab.cri.epita.fr',
  base: '/web',
  outDir: 'public',
  publicDir: 'static',
  integrations: [
    starlight({
      title: "Nexium",
    }),
  ],
});
