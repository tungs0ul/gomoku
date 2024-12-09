<script lang="ts">
  import { _ } from 'svelte-i18n'
  import Board from '$lib/assets/board.jpeg'
  import { api, auth } from '$lib/store.svelte'
  import { push } from 'svelte-spa-router'
  import { Button } from '$lib/components/ui/button'
  import { toast } from 'svelte-sonner'
  import * as Dialog from '$lib/components/ui/dialog'

  import type { GameResponse, GameType } from '$lib/types'
  import SignIn from '$lib/components/SignIn.svelte'

  let signInCommand = $state<'PlayBot' | 'PlayNormal' | 'WatchGame' | null>(
    null
  )

  let games = 345834985734
  let players = 1234567890

  const signInCallback = () => {
    switch (signInCommand) {
      case null:
        break
      case 'WatchGame':
        push('/rooms')
        break
      case 'PlayBot':
      case 'PlayNormal':
        createGame(signInCommand === 'PlayBot' ? 'bot' : 'normal')
        break
      default:
        break
    }
  }

  const createGame = async (gameType: GameType) => {
    if (auth.auth === null) {
      signInCommand = (() => {
        switch (gameType) {
          case 'bot':
            return 'PlayBot'
          case 'normal':
            return 'PlayNormal'
          default:
            return null
        }
      })()
      return
    }
    if (gameType) {
      try {
        const { data } = await auth.apiClient.post<GameResponse>(api.play, {
          user_id: auth.auth.user.id,
          game_type: gameType
        })
        push(`/rooms/${data.room}`)
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
        <Button
          class="justify-start bg-blue-400 py-8 text-left text-xl"
          on:click={() => createGame('bot')}>
          <!-- <Bot class="mr-2 h-4 w-4" /> -->
          {$_('play-with-bot')}
        </Button>

        <Button
          class="justify-start bg-green-400 py-8 text-xl"
          on:click={() => createGame('normal')}>
          <!-- <User class="mr-2 h-4 w-4" /> -->
          {$_('play-online')}
        </Button>

        <Button
          on:click={() => {
            if (auth.auth === null) {
              signInCommand = 'WatchGame'
              return
            }
            push('/rooms')
          }}
          class="w-full justify-start py-8 text-xl"
          variant="secondary">
          {$_('watch')}
        </Button>
      </div>
    </div>
  </div>
</div>

{#if signInCommand !== null}
  <Dialog.Root
    open={signInCommand !== null}
    onOpenChange={() => {
      signInCommand = null
    }}>
    <Dialog.Content class="bg-zinc-700">
      <Dialog.Header>
        <Dialog.Title class="text-green-400">{$_('Gomoku')}</Dialog.Title>
        <Dialog.Description class="text-[#769656]">
          {$_('login-to-play')}
        </Dialog.Description>
      </Dialog.Header>
      <SignIn callback={signInCallback} enableSignInAnonymously={true} />
    </Dialog.Content>
  </Dialog.Root>
{/if}
