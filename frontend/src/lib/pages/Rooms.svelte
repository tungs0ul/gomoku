<script lang="ts">
  import { type Game, type RoomType } from '$lib/types'
  import { api, client, user } from '$lib/store.svelte'
  import { flip } from 'svelte/animate'
  import { fly } from 'svelte/transition'
  import { link } from 'svelte-spa-router'
  import Bot from 'lucide-svelte/icons/bot'
  import Zap from 'lucide-svelte/icons/zap'
  import { createQuery } from '@tanstack/svelte-query'
  import { _ } from 'svelte-i18n'

  const getRooms = async () => {
    const { data } = await client.get(api.get_rooms)
    return data
  }

  const rooms = createQuery<[string, Game, RoomType][]>({
    queryKey: ['rooms'],
    queryFn: getRooms
  })
</script>

{#if $rooms.error}
  <div>{$_('error-getting-rooms')}</div>
{/if}
{#if $rooms.isSuccess}
  <div class="flex h-full w-full flex-col items-center py-4">
    {#each $rooms.data as [room, game, room_type] (room)}
      <a
        use:link
        href={`/ws/rooms/${room}/users/${user.user}`}
        animate:flip
        transition:fly
        class=" flex gap-4 rounded border bg-[#F5F0CD] px-6 py-4">
        <div>
          {#if room_type == 'bot'}
            <Bot />
          {:else if room_type == 'normal'}
            <Zap />
          {/if}
        </div>
        {room}
      </a>
    {/each}
  </div>
{/if}
