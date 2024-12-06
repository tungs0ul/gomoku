<script lang="ts">
  import { _, locale } from 'svelte-i18n'
  import Play from 'lucide-svelte/icons/play'
  import { link } from 'svelte-spa-router'
  import { Button } from '$lib/components/ui/button'
  import Settings from 'lucide-svelte/icons/settings'
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu'
</script>

<div class="flex h-full min-w-32 flex-col bg-[#0B192C] py-4 text-white">
  <div class="px-4 py-4 text-xl font-bold">
    <a href="/" use:link>Gomoku</a>
  </div>
  <div class="grow">
    <!-- <div
      class="flex cursor-pointer items-center gap-2 px-4 py-4 text-lg hover:bg-[#000000]"
    >
      <div><Play /></div>
      <div>{$_('play')}</div>
    </div> -->
  </div>

  <DropdownMenu.Root>
    <DropdownMenu.Trigger>
      <div class="text-6xl">
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
            class="flex items-center text-4xl"
            on:click={() => {
              locale.set(lang.value)
            }}>{lang.icon} {lang.value}</DropdownMenu.Item
          >
        {/each}
      </DropdownMenu.Group>
    </DropdownMenu.Content>
  </DropdownMenu.Root>

  <!--  <div class="flex grow flex-col gap-2">-->
  <!--    <a href="/sign-up" use:link class="flex justify-center px-2">-->
  <!--      <Button variant="secondary" class="w-full">{$_('sign-up')}</Button>-->
  <!--    </a>-->
  <!--    <a href="/sign-in" use:link class="flex justify-center px-2">-->
  <!--      <Button variant="destructive" class="w-full">{$_('login')}</Button>-->
  <!--    </a>-->
  <!--  </div>-->

  <!-- <a class="flex items-center gap-2 px-4 py-8" href="/settings" use:link>
    <div><Settings /></div>
    <div>{$_('settings')}</div>
  </a> -->
</div>
