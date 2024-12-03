<script lang="ts">
  import { _ } from 'svelte-i18n'
  import Play from 'lucide-svelte/icons/play'
  import { link } from 'svelte-spa-router'
  import { Button } from '$lib/components/ui/button'
  import Settings from 'lucide-svelte/icons/settings'
</script>

<div class="flex h-full min-w-32 flex-col bg-[#0B192C] py-4 text-white">
  <div class="px-4 py-4 text-xl font-bold">
    <a href="/" use:link>Gomoku</a>
  </div>
  <div class="grow">
    <div
      class="flex cursor-pointer items-center gap-2 px-4 py-4 text-lg hover:bg-[#000000]"
    >
      <div><Play /></div>
      <div>{$_('play')}</div>
    </div>
  </div>

  <div class="flex grow flex-col gap-2">
    <a href="/sign-up" use:link class="flex justify-center px-2">
      <Button variant="secondary" class="w-full">{$_('sign-up')}</Button>
    </a>
    <a href="/sign-in" use:link class="flex justify-center px-2">
      <Button variant="destructive" class="w-full">{$_('login')}</Button>
    </a>
  </div>

  <a use:link href="/settings" class="flex items-center gap-2 px-4 py-8">
    <div><Settings /></div>
    <div>{$_('settings')}</div>
  </a>
</div>
