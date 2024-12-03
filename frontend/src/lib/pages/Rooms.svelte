<script lang="ts">
  import { type Game } from '$lib/types'
  import { client, user } from '$lib/store.svelte'
  import { flip } from 'svelte/animate'
  import { fly } from 'svelte/transition'
  import { link } from 'svelte-spa-router'

  let rooms = $state<[string, Game][]>([])

  $effect(() => {
    client.get('/room').then(({ data }) => {
      rooms = data
    })
  })
</script>

<div class="flex h-full w-full flex-col items-center py-4">
  {#each rooms as [room, game] (room)}
    <a
      use:link
      href={`/play/${room}/${user.user}`}
      animate:flip
      transition:fly
      class=" rounded border bg-[#F5F0CD] px-6 py-4"
    >
      {room}
    </a>
  {/each}
</div>
