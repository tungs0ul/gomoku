import { mount } from 'svelte'
import { getLocaleFromNavigator, init, register } from 'svelte-i18n'
import './app.css'
import App from './App.svelte'

register('en-US', () => import('$lib/languages/en.json'))
register('vi-VN', () => import('$lib/languages/vn.json'))

init({
  fallbackLocale: 'en-US',
  initialLocale: localStorage.getItem('language') ?? getLocaleFromNavigator()
})

const app = mount(App, {
  target: document.getElementById('app')!
})

export default app
