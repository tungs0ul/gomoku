<script lang="ts">
  import { _ } from 'svelte-i18n'
  import Bot from 'lucide-svelte/icons/bot'
  import Zap from 'lucide-svelte/icons/zap'
  import User from 'lucide-svelte/icons/user'
  import Board from '$lib/assets/board.jpeg'
  import { client, user } from '$lib/store.svelte'
  import { link, push } from 'svelte-spa-router'
  import * as Drawer from '$lib/components/ui/drawer'
  import { Button } from '$lib/components/ui/button'

  // import * as Drawer from '$lib/components/ui/drawer'
  // import { Button } from '$lib/components/ui/button'

  let games = 345834985734
  let players = 1234567890

  const createGame = async (single: boolean) => {
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
        <Drawer.Root>
          <Drawer.Trigger>
            <Button class="w-full py-12 text-4xl" variant="default" size="lg"
              >{$_('play')}</Button
            >
          </Drawer.Trigger>
          <Drawer.Content>
            <Drawer.Header>
              <Drawer.Title>{$_('create-game')}</Drawer.Title>
              <Drawer.Description>
                {$_('create-an-online-room-any-one-can-join')}
              </Drawer.Description>
            </Drawer.Header>
            <Drawer.Footer>
              <button
                onclick={() => createGame(true)}
                class="grid grid-cols-2 items-center gap-4 rounded-md bg-blue-400 py-6"
              >
                <div class="flex justify-end">
                  <Bot class="h-6 w-6" />
                </div>
                <div class="flex">{$_('play-bot')}</div>
              </button>

              <button
                onclick={() => alert($_('feature-in-progress'))}
                class="grid grid-cols-2 items-center gap-4 rounded-md bg-green-400 py-6"
              >
                <div class="flex justify-end">
                  <User class="h-6 w-6" />
                </div>
                <div class="flex">{$_('play-friend')}</div>
              </button>
              <button
                onclick={() => alert($_('feature-in-progress'))}
                class="grid grid-cols-2 items-center gap-4 rounded-md bg-red-400 py-6"
              >
                <div class="flex justify-end">
                  <Zap class="h-6 w-6" />
                </div>
                <div class="flex">{$_('play-random')}</div>
              </button>
              <Drawer.Close>{$_('cancel')}</Drawer.Close>
            </Drawer.Footer>
          </Drawer.Content>
        </Drawer.Root>

        <a href="/rooms" use:link class="w-full">
          <Button size="lg" class="w-full py-8 text-4xl" variant="secondary">
            {$_('watch')}
          </Button>
        </a>
      </div>
    </div>
  </div>
</div>
