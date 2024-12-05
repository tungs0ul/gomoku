<script lang="ts">
  import { _ } from 'svelte-i18n'
  import Bot from 'lucide-svelte/icons/bot'
  import User from 'lucide-svelte/icons/user'
  import Board from '$lib/assets/board.jpeg'
  import { api, client, user } from '$lib/store.svelte'
  import { link, push } from 'svelte-spa-router'
  import { Button } from '$lib/components/ui/button'
  import { toast } from 'svelte-sonner'

  import * as Drawer from '$lib/components/ui/drawer'
  import type { GameType } from '$lib/types'

  let games = 345834985734
  let players = 1234567890

  const createGame = async (gameType: GameType) => {
    if (user.user === null) return
    if (gameType) {
      try {
        const { data } = await client.post(api.play, {
          user_id: user.user,
          game_type: gameType
        })
        push(data)
      } catch (e) {
        console.error(e)
        toast.error($_('there-is-something-wrong-please-try-again-later'), {
          position: 'top-center'
        })
      }
    }
  }
</script>

<div class="flex h-full items-center justify-center">
  <div class="flex flex-col gap-24 py-16 text-white sm:flex-row">
    <div class="hidden max-w-md place-items-center rounded-xl sm:grid">
      <img alt="gomoku board" class="min-w-sm rounded-xl" src={Board} />
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
            <Button class="w-full py-12 text-4xl" size="lg" variant="default"
              >{$_('play')}</Button>
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
                class="grid grid-cols-2 items-center gap-4 rounded-md bg-blue-400 py-6"
                onclick={() => createGame('bot')}>
                <div class="flex justify-end">
                  <Bot class="h-6 w-6" />
                </div>
                <div class="flex">{$_('play-with-bot')}</div>
              </button>

              <button
                class="grid grid-cols-2 items-center gap-4 rounded-md bg-green-400 py-6"
                onclick={() => createGame('normal')}>
                <div class="flex justify-end">
                  <User class="h-6 w-6" />
                </div>
                <div class="flex">{$_('play-online')}</div>
              </button>
              <Drawer.Close>{$_('cancel')}</Drawer.Close>
            </Drawer.Footer>
          </Drawer.Content>
        </Drawer.Root>

        <a class="w-full" href="/rooms" use:link>
          <Button class="w-full py-8 text-4xl" size="lg" variant="secondary">
            {$_('watch')}
          </Button>
        </a>
      </div>
    </div>
  </div>
</div>
