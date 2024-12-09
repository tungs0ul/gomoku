<script lang="ts">
  import { Button } from '$lib/components/ui/button'
  import { supabase } from '$lib/store.svelte'
  import { _ } from 'svelte-i18n'

  let { callback, enableSignInAnonymously } = $props()

  const signIn = (provider: 'facebook' | 'github' | 'google') => {
    supabase.auth.signInWithOAuth({ provider }).then(callback)
  }
</script>

<div class="flex flex-col gap-4">
  <button
    onclick={() => {
      signIn('google')
    }}
    class="flex items-center gap-2 rounded-lg border border-gray-300 bg-white px-4 py-2 text-gray-800 shadow hover:bg-gray-200">
    <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48">
      <path
        fill="#fbc02d"
        d="M44.5 20H24v8.5h11.8c-1.5 4.7-5.7 8-11.8 8-6.6 0-12-5.4-12-12s5.4-12 12-12c2.9 0 5.5 1 7.6 2.6l6.4-6.4C33.3 5.3 28.9 4 24 4 12.9 4 4 12.9 4 24s8.9 20 20 20c10.2 0 19-7.4 19-20 0-1.3-.2-2.7-.5-4z" />
      <path
        fill="#e53935"
        d="M6.3 14.9l6.7 4.9C15 15 19.2 12 24 12c2.9 0 5.5 1 7.6 2.6l6.4-6.4C33.3 5.3 28.9 4 24 4c-6.7 0-12.5 3.2-16.3 8.1z" />
      <path
        fill="#4caf50"
        d="M24 44c5.7 0 10.9-2.2 14.7-5.8l-7-5.7c-2.1 1.4-4.8 2.5-7.7 2.5-4.8 0-8.9-3-10.6-7.1l-7 5.4c3.6 5.1 9.5 8.7 16.6 8.7z" />
      <path
        fill="#1565c0"
        d="M44.5 20H24v8.5h11.8c-.7 2.4-2.2 4.4-4.1 5.9v.1l7 5.7c-.5.5 7.8-5.5 7.8-16.2 0-1.3-.2-2.7-.5-4z" />
    </svg>
    <span>{$_('login-with-google')}</span>
  </button>

  <button
    onclick={() => {
      signIn('github')
    }}
    class="flex items-center gap-2 rounded-lg bg-gray-800 px-4 py-2 text-white shadow hover:bg-gray-900">
    <svg
      class="h-6 w-6"
      xmlns="http://www.w3.org/2000/svg"
      fill="currentColor"
      viewBox="0 0 24 24">
      <path
        d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.387.6.113.82-.258.82-.577v-2.234c-3.338.726-4.042-1.416-4.042-1.416-.546-1.387-1.333-1.757-1.333-1.757-1.091-.746.083-.73.083-.73 1.205.084 1.838 1.237 1.838 1.237 1.07 1.835 2.809 1.305 3.495.997.107-.775.418-1.305.762-1.605-2.665-.3-5.466-1.332-5.466-5.931 0-1.31.467-2.382 1.236-3.221-.124-.303-.536-1.523.116-3.176 0 0 1.008-.322 3.3 1.23.957-.266 1.983-.398 3.003-.403 1.02.005 2.047.137 3.006.403 2.291-1.553 3.297-1.23 3.297-1.23.653 1.653.241 2.873.118 3.176.771.839 1.233 1.911 1.233 3.221 0 4.609-2.804 5.625-5.475 5.921.43.372.814 1.102.814 2.222v3.293c0 .322.216.694.824.576C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12" />
    </svg>
    <span>{$_('login-with-github')}</span>
  </button>

  <!-- <button
    onclick={() => {
      signIn('facebook')
    }}
    class="flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-white shadow hover:bg-blue-700">
    <svg
      class="h-6 w-6"
      xmlns="http://www.w3.org/2000/svg"
      fill="currentColor"
      viewBox="0 0 24 24">
      <path
        d="M22.675 0h-21.35C.6 0 0 .6 0 1.325v21.351C0 23.4.6 24 1.325 24h11.486v-9.294H9.691v-3.622h3.12V8.41c0-3.1 1.894-4.788 4.66-4.788 1.325 0 2.463.098 2.794.143v3.24l-1.918.001c-1.504 0-1.794.715-1.794 1.763v2.31h3.587l-.467 3.622h-3.12V24h6.116c.725 0 1.325-.6 1.325-1.324V1.325C24 .6 23.4 0 22.675 0z" />
    </svg>
    <span>{$_('login-with-facebook')}</span>
  </button> -->

  {#if enableSignInAnonymously}
    <div class="relative">
      <div class="absolute inset-0 flex items-center">
        <div class="w-full border-t border-zinc-600"></div>
      </div>
      <div class="relative flex justify-center text-sm">
        <span class="bg-zinc-700 px-2 uppercase text-zinc-400">{$_('or')}</span>
      </div>
    </div>

    <Button
      class="text-white"
      variant="link"
      onclick={() => {
        supabase.auth.signInAnonymously().then(callback)
      }}>{$_('join-as-guest')}</Button>
  {/if}
</div>
