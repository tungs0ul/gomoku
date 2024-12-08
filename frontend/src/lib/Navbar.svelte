<script lang="ts">
  import { _, locale } from 'svelte-i18n'
  import Play from 'lucide-svelte/icons/play'
  import { link, replace } from 'svelte-spa-router'
  import { Button } from '$lib/components/ui/button'
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu'
  import { auth } from './store.svelte'
  import * as Dialog from '$lib/components/ui/dialog'
  import Settings from 'lucide-svelte/icons/settings'

  import SignIn from '$lib/components/SignIn.svelte'

  let openSignInModal = $state(false)
</script>

<div class="flex h-full w-fit flex-col bg-[#0B192C] py-4 text-white">
  <div class="px-4 py-4 text-xl font-bold">
    <a href="/" use:link>Gomoku</a>
  </div>
  <div class="grow">
    <a
      href="/"
      use:link
      class="flex cursor-pointer items-center gap-2 px-4 py-4 text-lg hover:bg-[#000000]"
    >
      <div><Play /></div>
      <div>{$_('play')}</div>
    </a>
  </div>

  <div class="flex grow flex-col gap-2 px-4">
    {#if auth.auth === null || auth.auth.user.is_anonymous}
      <Button
        on:click={() => {
          openSignInModal = true
        }}
        class="w-full bg-green-600 hover:bg-green-500">{$_('sign-in')}</Button
      >
    {/if}
  </div>

  {#if auth.auth === null || auth.auth.user.is_anonymous}
    <DropdownMenu.Root>
      <DropdownMenu.Trigger>
        <div class="text-4xl">
          {#if $locale?.includes('en')}
            🇺🇸
          {:else if $locale?.includes('vn')}
            🇻🇳
          {/if}
        </div>
      </DropdownMenu.Trigger>
      <DropdownMenu.Content>
        <DropdownMenu.Group>
          {#each [{ icon: '🇺🇸', value: 'en' }, { icon: '🇻🇳', value: 'vn' }].filter((e) => !$locale?.startsWith(e.value)) as lang (lang.value)}
            <DropdownMenu.Item
              class="flex items-center text-2xl"
              on:click={() => {
                locale.set(lang.value)
              }}>{lang.icon} {$_(lang.value)}</DropdownMenu.Item
            >
          {/each}
        </DropdownMenu.Group>
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  {:else}
    <div class="grid place-items-center">
      <a href="/settings" use:link class="flex items-center gap-1">
        <Settings />
        {$_('settings')}
      </a>
    </div>
  {/if}

  <!-- <a class="flex items-center gap-2 px-4 py-8" href="/settings" use:link>
    <div><Settings /></div>
    <div>{$_('settings')}</div>
  </a> -->
</div>

<Dialog.Root bind:open={openSignInModal}>
  <Dialog.Content class="bg-zinc-700">
    <Dialog.Header>
      <Dialog.Title class="text-green-400">{$_('Gomoku')}</Dialog.Title>
      <Dialog.Description class="text-[#769656]">
        {$_('login-to-play')}
      </Dialog.Description>
    </Dialog.Header>
    <SignIn
      callback={() => {
        replace('/')
      }}
      enableSignInAnonymously={false}
    />
  </Dialog.Content>
</Dialog.Root>
