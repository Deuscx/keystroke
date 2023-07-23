import fs from 'node:fs'
import path from 'node:path'
import {
  defineConfig,
  presetAttributify,
  presetIcons,
  presetUno,
  presetWebFonts,
  transformerDirectives,
  transformerVariantGroup,
} from 'unocss'

const baseSvgPath = path.resolve(__dirname, './src/assets/svgs')
const icons = fs.readdirSync(baseSvgPath)

const iconsMap = icons.reduce<Record<string, string>>((map, filename) => {
  const iconName = path.basename(filename, '.svg')
  map[iconName] = fs.readFileSync(path.resolve(baseSvgPath, filename), 'utf-8')
  return map
}, {})
console.log(icons)
export default defineConfig({
  theme: {
    colors: {
      primary: '#6979F8',
    },
  },
  shortcuts: [
    ['btn', 'px-4 py-1 rounded inline-block bg-teal-600 text-white cursor-pointer hover:bg-teal-700 disabled:cursor-default disabled:bg-gray-600 disabled:opacity-50'],
    ['icon-btn', 'text-[0.9em] inline-block cursor-pointer select-none opacity-75 transition duration-200 ease-in-out hover:opacity-100 hover:text-teal-600 !outline-none'],
  ],
  presets: [
    presetUno(),
    presetAttributify(),
    presetIcons({
      scale: 1.2,
      warn: true,
      collections: {
        custom: iconsMap,
      },
    }),
    presetWebFonts({
      fonts: {
        sans: 'Roboto',
        mono: ['Fira Code', 'Fira Mono:400,700'],
      },
    }),
  ],
  transformers: [
    transformerDirectives(),
    transformerVariantGroup(),
  ],
})
