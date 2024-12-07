<script lang="ts">
  import { _ } from 'svelte-i18n'
  import Bot from 'lucide-svelte/icons/bot'
  import User from 'lucide-svelte/icons/user'
  import Board from '$lib/assets/board.jpeg'
  import { api, auth, supabase } from '$lib/store.svelte'
  import { push } from 'svelte-spa-router'
  import { Button } from '$lib/components/ui/button'
  import { toast } from 'svelte-sonner'
  import * as Dialog from '$lib/components/ui/dialog'

  import * as Drawer from '$lib/components/ui/drawer'
  import type { GameType } from '$lib/types'

  let commandAfterLogin = $state<'PlayBot' | 'PlayNormal' | 'WatchGame' | null>(
    null
  )

  let games = 345834985734
  let players = 1234567890

  const signInAndResumeLastCommand = () => {
    switch (commandAfterLogin) {
      case null:
        break
      case 'WatchGame':
        push('/rooms')
        break
      case 'PlayBot':
      case 'PlayNormal':
        createGame(commandAfterLogin === 'PlayBot' ? 'bot' : 'normal')
        break
      default:
        break
    }
  }

  const createGame = async (gameType: GameType) => {
    if (auth.auth === null) {
      commandAfterLogin = (() => {
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
        const { data } = await auth.apiClient.post(api.play, {
          user_id: auth.auth.user.id,
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
        <Button
          size="lg"
          class="justify-start bg-blue-400 px-16 py-8 text-left text-4xl"
          on:click={() => createGame('bot')}>
          <!-- <Bot class="mr-2 h-4 w-4" /> -->
          {$_('play-with-bot')}
        </Button>

        <Button
          size="lg"
          class="justify-start bg-green-400 px-16 py-8 text-4xl"
          on:click={() => createGame('normal')}>
          <!-- <User class="mr-2 h-4 w-4" /> -->
          {$_('play-online')}
        </Button>

        <Button
          on:click={() => {
            if (auth.auth === null) {
              commandAfterLogin = 'WatchGame'
              return
            }
            push('/rooms')
          }}
          class="w-full justify-start px-16 py-8 text-4xl"
          size="lg"
          variant="secondary">
          {$_('watch')}
        </Button>
      </div>
    </div>
  </div>
</div>

{#if commandAfterLogin !== null}
  <Dialog.Root
    open={commandAfterLogin !== null}
    onOpenChange={(open) => {
      if (!open) {
        commandAfterLogin = null
      }
    }}>
    <Dialog.Content>
      <Dialog.Header>
        <Dialog.Title>{$_('play-gomoku-online')}</Dialog.Title>
        <Dialog.Description>
          {$_('please-login-to-join-gomoku')}
        </Dialog.Description>
      </Dialog.Header>
      <div class="flex flex-col gap-4">
        <Button
          variant="default"
          class="bg-black"
          on:click={() => {
            supabase.auth
              .signInWithOAuth({ provider: 'github' })
              .then(signInAndResumeLastCommand)
          }}>{$_('login-with-github')}</Button>
        <Button
          on:click={() => {
            supabase.auth
              .signInWithOAuth({ provider: 'google' })
              .then(signInAndResumeLastCommand)
          }}
          variant="secondary">{$_('login-with-google')}</Button>

        <Button
          variant="link"
          onclick={() => {
            supabase.auth.signInAnonymously().then(signInAndResumeLastCommand)
          }}>{$_('play-as-guest')}</Button>
      </div>
    </Dialog.Content>
  </Dialog.Root>
{/if}
