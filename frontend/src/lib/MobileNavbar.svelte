<script lang="ts">
  import { _ } from 'svelte-i18n'
  import Play from 'lucide-svelte/icons/play'
  import { link } from 'svelte-spa-router'
  import { Button } from '$lib/components/ui/button'
  import Settings from 'lucide-svelte/icons/settings'
</script>

<div class="flex items-center bg-[#0B192C] text-white">
  <div class="grow px-4 py-4 text-xl">
    <a href="/" use:link>Gomoku</a>
  </div>

  <div class="flex gap-2">
    <a href="/sign-up" use:link class="flex justify-center px-2">
      <Button variant="secondary" size="sm" class="w-full"
        >{$_('sign-up')}</Button>
    </a>
    <a href="/sign-in" use:link class="flex justify-center px-2">
      <Button variant="destructive" size="sm" class="w-full"
        >{$_('login')}</Button>
    </a>
  </div>
</div>
