<script lang="ts">
  import { type Game } from '$lib/types'
  import { api, auth } from '$lib/store.svelte'
  import { flip } from 'svelte/animate'
  import { fly } from 'svelte/transition'
  import { link, replace } from 'svelte-spa-router'
  import Bot from 'lucide-svelte/icons/bot'
  import Zap from 'lucide-svelte/icons/zap'
  import { createQuery } from '@tanstack/svelte-query'
  import { _ } from 'svelte-i18n'
  import { Button } from '$lib/components/ui/button'

  if (auth.auth === null) {
    replace('/')
  }

  const getRooms = async () => {
    if (auth.auth === null) return
    const { data } = await auth.apiClient.get(api.get_rooms)
    return data
  }

  const rooms = createQuery<Game[]>({
    queryKey: ['rooms'],
    queryFn: getRooms
  })
</script>

{#if $rooms.error}
  <div>{$_('error-getting-rooms')}</div>
{/if}
{#if $rooms.isSuccess}
  {#if $rooms.data.length === 0}
    <div class="grid h-full w-full place-items-center">
      <div class="flex flex-col items-center gap-4">
        <div class="text-4xl font-bold text-red-400">
          {$_('there-is-no-room-atm')}
        </div>

        <a use:link href="/">
          <Button variant="link" size="lg" class="text-xl text-white">
            {$_('back-to-main')}
          </Button>
        </a>
      </div>
    </div>
  {/if}
  <div class="flex h-full w-full flex-col items-center py-4">
    {#each $rooms.data as game (game.room_id)}
      <a
        use:link
        href={`/rooms/${game.room_id}`}
        animate:flip
        transition:fly
        class=" flex gap-4 rounded border bg-[#F5F0CD] px-6 py-4">
        <div>
          {#if game.game_type == 'bot'}
            <Bot />
          {:else if game.game_type == 'normal'}
            <Zap />
          {/if}
        </div>
        {game.room_id}
      </a>
    {/each}
  </div>
{/if}
