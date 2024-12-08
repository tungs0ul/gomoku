<script lang="ts">
  import { Button } from '$lib/components/ui/button'
  import { supabase } from '$lib/store.svelte'
  import { _ } from 'svelte-i18n'

  let { callback, enableSignInAnonymously } = $props()
</script>

<div class="flex flex-col gap-4">
  <Button
    variant="default"
    class="bg-black"
    on:click={() => {
      supabase.auth.signInWithOAuth({ provider: 'github' }).then(callback)
    }}>{$_('login-with-github')}</Button
  >
  <Button
    on:click={() => {
      supabase.auth.signInWithOAuth({ provider: 'google' }).then(callback)
    }}
    variant="secondary">{$_('login-with-google')}</Button
  >

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
      }}>{$_('play-as-guest')}</Button
    >
  {/if}
</div>
