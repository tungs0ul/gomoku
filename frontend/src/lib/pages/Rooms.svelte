<script lang="ts">
  import {type Game, type RoomType} from '$lib/types'
  import {api, client, user} from '$lib/store.svelte'
  import {flip} from 'svelte/animate'
  import {fly} from 'svelte/transition'
  import {link} from 'svelte-spa-router'
  import Bot from 'lucide-svelte/icons/bot'
  import Zap from 'lucide-svelte/icons/zap'

  let rooms = $state<[string, Game, RoomType][]>([])
    $effect(() => {
        client.get(api.get_rooms).then(({data}) => {
            rooms = data
        })
    })
</script>

<div class="flex h-full w-full flex-col items-center py-4">
    {#each rooms as [room, game, room_type] (room)}
        <a
                use:link
                href={`/ws/rooms/${room}/users/${user.user}`}
                animate:flip
                transition:fly
                class=" rounded border bg-[#F5F0CD] px-6 py-4 flex gap-4"
        >
            <div>
                {#if room_type == 'bot'}
                    <Bot/>
                    {:else if room_type == 'normal'}
                    <Zap/>
                {/if}
            </div>
            {room}
        </a>
    {/each}
</div>
