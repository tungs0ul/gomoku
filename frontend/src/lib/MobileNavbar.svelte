<script lang="ts">
  import { _ } from 'svelte-i18n'
  import { link, replace } from 'svelte-spa-router'
  import { Button } from '$lib/components/ui/button'
  import { auth } from './store.svelte'
  import * as Dialog from '$lib/components/ui/dialog'
  import SignIn from '$lib/components/SignIn.svelte'
  import Settings from 'lucide-svelte/icons/settings'

  let openSignInModal = $state(false)
</script>

<div class="flex items-center bg-[#0B192C] text-white">
  <div class="grow px-4 py-4 text-xl">
    <a href="/" use:link>Gomoku</a>
  </div>

  <div class="flex flex-col gap-2 px-4">
    {#if auth.auth === null || auth.auth.user.is_anonymous}
      <Button
        on:click={() => {
          openSignInModal = true
        }}
        class="w-full bg-green-600 hover:bg-green-500">{$_('sign-in')}</Button>
    {:else}
      <div class="grid place-items-center">
        <a href="/settings" use:link>
          <Settings />
        </a>
      </div>
    {/if}
  </div>
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
      enableSignInAnonymously={false} />
  </Dialog.Content>
</Dialog.Root>
