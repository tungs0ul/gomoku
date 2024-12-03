<script lang="ts">
  import { _ } from 'svelte-i18n'
  import Bot from 'lucide-svelte/icons/bot'
  import User from 'lucide-svelte/icons/user'
  import Board from '$lib/assets/board.jpeg'
  import { client, user } from '$lib/store.svelte'
  import { push } from 'svelte-spa-router'

  let games = 345834985734
  let players = 1234567890

  const createGame = async (single: boolean, random: boolean) => {
    if (user.user === null) return
    if (single) {
      const { data } = await client.get(`/game/bot/${user.user}`)
      push(`/play/${data}/${user.user}`)
    }
  }
</script>

<div class="flex h-full items-center justify-center">
  <div class="flex flex-col gap-24 py-16 text-white sm:flex-row">
    <div class="hidden max-w-md place-items-center rounded-xl sm:grid">
      <img src={Board} alt="gomoku board" class="min-w-sm rounded-xl" />
    </div>
    <div class="flex max-w-md grow flex-col gap-12">
      <div class="flex flex-col gap-8">
        <div class="w-full text-center text-6xl font-bold sm:text-8xl">
          {$_('Gomoku')}
        </div>
        <div class="flex justify-between gap-4">
          <div class="flex items-center gap-2">
            <div class="text-lg font-semibold">
              {games}
            </div>
            <div class="font-thin">
              {$_('games')}
            </div>
          </div>
          <div class="flex items-center gap-2">
            <div class="text-lg font-semibold">
              {players}
            </div>
            <div class="font-thin">
              {$_('players')}
            </div>
          </div>
        </div>
      </div>

      <div class="flex flex-col gap-4">
        <button
          onclick={() => createGame(true, false)}
          class="flex w-full justify-between gap-2 rounded-xl bg-[#7c945d] px-4 py-6 text-xl shadow-xl hover:bg-[#6a8049]"
        >
          <User class="h-16 w-16" />
          <div class="grid grow gap-2">
            <div class="text-4xl font-bold">{$_('play-online')}</div>
            <div class="text-sm font-thin">
              {$_('find-room-to-play-with')}
            </div>
          </div>
        </button>
        <button
          onclick={() => createGame(true, false)}
          class="flex w-full justify-between gap-2 rounded-xl border-gray-600 bg-[#3d3d3d] px-4 py-6 text-xl shadow-xl"
        >
          <Bot class="h-16 w-16" />
          <div class="grid grow gap-2">
            <div class="text-4xl font-bold">{$_('play-bots')}</div>
            <div class="text-sm font-thin">
              <!-- {$_('play-with-customizable-training-bots')} -->
              {$_('feature-in-progress')}
            </div>
          </div>
        </button>
      </div>
    </div>
  </div>
</div>
