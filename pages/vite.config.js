import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import mdx from '@mdx-js/rollup'
import rehypeShiki from '@shikijs/rehype'
import fs from 'fs'

const fuzzyGrammar = JSON.parse(fs.readFileSync('./fuzzy.tmLanguage.json', 'utf8'))

export default defineConfig({
  plugins: [
    mdx({
      rehypePlugins: [
        [
          rehypeShiki,
          {
            theme: 'slack-dark',
            langs: [fuzzyGrammar]
          }
        ]
      ]
    }),
    react()
  ],
  base: '/fuzzy/'
})
