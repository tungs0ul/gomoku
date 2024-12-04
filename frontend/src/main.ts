import { mount } from 'svelte'
import { getLocaleFromNavigator, init, register } from 'svelte-i18n'
import './app.css'
import App from './App.svelte'

register('en', () => import('$lib/languages/en.json'))
register('vn', () => import('$lib/languages/vn.json'))

init({
  fallbackLocale: 'en',
  initialLocale: getLocaleFromNavigator()
})

const app = mount(App, {
  target: document.getElementById('app')!
})

export default app
