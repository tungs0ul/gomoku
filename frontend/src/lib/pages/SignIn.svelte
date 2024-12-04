<script lang="ts">
  import { _ } from 'svelte-i18n'
  import { Input } from '$lib/components/ui/input'
  import { Button } from '$lib/components/ui/button'
  import { link } from 'svelte-spa-router'
  import Eye from 'lucide-svelte/icons/eye'
  import EyeOff from 'lucide-svelte/icons/eye-off'

  let showPassword = $state(false)
</script>

<div class="fixed inset-0 grid place-items-center bg-[#121212]">
  <div class="flex items-center justify-center rounded-xl bg-zinc-800 p-4">
    <div class="w-full max-w-md">
      <div class="mb-8 text-center">
        <div class="flex items-center justify-center">
          <div class="mr-2 h-8 w-8 rounded-full bg-[#769656]"></div>
          <a href="/" use:link class="text-3xl font-bold text-white">Gomoku</a>
        </div>
      </div>
      <div class="rounded-lg bg-zinc-700 p-8 shadow-lg">
        <form class="space-y-6">
          <div>
            <div class="relative">
              <Input
                type="email"
                placeholder={$_('email')}
                class="w-full border-zinc-500 bg-zinc-600 pl-10 text-white placeholder-zinc-400" />
              <span
                class="absolute left-3 top-1/2 -translate-y-1/2 transform text-zinc-400">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  class="h-5 w-5"
                  viewBox="0 0 20 20"
                  fill="currentColor">
                  <path
                    fill-opacity="evenodd"
                    d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z"
                    clip-rule="evenodd" />
                </svg>
              </span>
            </div>
          </div>
          <div>
            <div class="relative">
              <Input
                type={showPassword ? 'text' : 'password'}
                placeholder={$_('password')}
                class="w-full border-zinc-500 bg-zinc-600 pl-10 pr-10 text-white placeholder-zinc-400" />
              <span
                class="absolute left-3 top-1/2 -translate-y-1/2 transform text-zinc-400">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  class="h-5 w-5"
                  viewBox="0 0 20 20"
                  fill="currentColor">
                  <path
                    fill-rule="evenodd"
                    d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z"
                    clip-rule="evenodd" />
                </svg>
              </span>
              <button
                type="button"
                class="absolute right-3 top-1/2 -translate-y-1/2 transform text-zinc-400"
                onclick={() => (showPassword = !showPassword)}>
                {#if showPassword}
                  <EyeOff />
                {:else}
                  <Eye />
                {/if}
              </button>
            </div>
          </div>
          <div class="flex items-center justify-end">
            <a
              href="/forgot-password"
              use:link
              class="text-sm text-zinc-300 hover:underline">
              {$_('forgot-password')}
            </a>
          </div>
          <Button
            class="w-full rounded bg-[#769656] px-4 py-2 font-bold text-white hover:bg-[#658046]">
            {$_('log-in')}
          </Button>
        </form>
        <div class="mt-6">
          <div class="relative">
            <div class="absolute inset-0 flex items-center">
              <div class="w-full border-t border-zinc-600"></div>
            </div>
            <div class="relative flex justify-center text-sm">
              <span class="bg-zinc-700 px-2 text-zinc-400">OR</span>
            </div>
          </div>
        </div>
        <div class="mt-6 space-y-4">
          <Button variant="secondary" class="grid w-full grid-cols-[auto_1fr]">
            <svg
              class="mr-2 h-5 w-5"
              viewBox="0 0 21 21"
              fill="none"
              xmlns="http://www.w3.org/2000/svg">
              <path
                d="M17.3 10.9C17.3 10.4 17.2 9.90005 17.1 9.40005H10.5V12.1H14.3C14.1 13 13.5 13.7 12.6 14.2V16.1H15.1C16.5 14.8 17.3 13 17.3 10.9Z"
                fill="#4285F4" />
              <path
                d="M10.5 18C12.6 18 14.4 17.3 15.1 16.1L12.6 14.2C11.9 14.7 11.1 15 10.5 15C8.80001 15 7.30001 13.8 6.80001 12.2H4.20001V14.1C5.30001 16.4 7.70001 18 10.5 18Z"
                fill="#34A853" />
              <path
                d="M6.8 12.2C6.6 11.6 6.5 11 6.5 10.4C6.5 9.8 6.6 9.2 6.8 8.6V6.7H4.2C3.7 7.8 3.5 9.1 3.5 10.4C3.5 11.7 3.8 13 4.2 14.1L6.8 12.2Z"
                fill="#FBBC05" />
              <path
                d="M10.5 5.8C11.7 5.8 12.8 6.2 13.6 7L15.8 4.8C14.4 3.5 12.6 2.8 10.5 2.8C7.7 2.8 5.3 4.4 4.2 6.7L6.8 8.6C7.3 7 8.8 5.8 10.5 5.8Z"
                fill="#EA4335" />
            </svg>
            {$_('log-in-with-google')}
          </Button>
          <Button
            variant="default"
            class="grid w-full grid-cols-[auto_1fr] border-[#1877F2] bg-[#1877F2] text-white hover:bg-[#0a60d6]">
            <svg
              class="mr-2 h-5 w-5"
              viewBox="0 0 24 24"
              fill="currentColor"
              xmlns="http://www.w3.org/2000/svg">
              <path
                d="M24 12.073c0-6.627-5.373-12-12-12s-12 5.373-12 12c0 5.99 4.388 10.954 10.125 11.854v-8.385H7.078v-3.47h3.047V9.43c0-3.007 1.792-4.669 4.533-4.669 1.312 0 2.686.235 2.686.235v2.953H15.83c-1.491 0-1.956.925-1.956 1.874v2.25h3.328l-.532 3.47h-2.796v8.385C19.612 23.027 24 18.062 24 12.073z" />
            </svg>
            {$_('log-in-with-facebook')}
          </Button>
        </div>
      </div>
      <div class="mt-8 text-center">
        <p class="text-zinc-400">
          {$_('new')}?{' '}
          <a href="/sign-up" use:link class="text-[#769656] hover:underline">
            {$_('sign-in-and-play')}
          </a>
        </p>
      </div>
    </div>
  </div>
</div>
