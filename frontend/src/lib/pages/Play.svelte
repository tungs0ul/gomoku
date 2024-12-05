<script lang="ts">
  import { HOST, PORT, user } from '$lib/store.svelte'
  import type { Game, GameEvent, Message, Player } from '$lib/types'
  import GameRender from '$lib/Game.svelte'
  import { link, location } from 'svelte-spa-router'
  import { onDestroy, onMount } from 'svelte'
  import PlayerXSound from '$lib/assets/sound/player-action-x.mp3'
  import PlayerOSound from '$lib/assets/sound/player-action-o.mp3'
  import VictorySound from '$lib/assets/sound/victory-2.mp3'
  import DefeatedSound from '$lib/assets/sound/defeat.mp3'
  import GameStartSound from '$lib/assets/sound/game-start.mp3'
  import Button from '$lib/components/ui/button/button.svelte'
  import { fly } from 'svelte/transition'
  import { _ } from 'svelte-i18n'
  import Chat from '$lib/components/Chat.svelte'
  import * as Drawer from '$lib/components/ui/drawer'
  import MessageSquareMore from 'lucide-svelte/icons/message-square-more'

  let game = $state<Game | null>(null)
  let predicts = $state<{ row: number; col: number; score: number }[]>([])
  let socket = $state<WebSocket | null>(null)
  let player = $state<Player | null>(null)
  let chatBody = $state<HTMLDivElement | null>(null)

  let wsError = $state<boolean>(false)

  let xAudio = new Audio(PlayerXSound)
  let oAudio = new Audio(PlayerOSound)
  let victoryAudio = new Audio(VictorySound)
  let defeatedAudio = new Audio(DefeatedSound)
  let gameStartAudio = new Audio(GameStartSound)

  let messages = $state<Message[]>([])
  let unReadMessages = $state<number>(0)

  onDestroy(() => {
    socket?.close()
  })

  onMount(() => {
    // axios
    //   .post(`http://localhost:11211/api/play/bot/${user.user}`)
    //   .then(({ data }) => {
    //     console.log(data)
    socket = new WebSocket(`ws://${HOST}:${PORT}${$location}`)
    socket.onopen = () => {
      socket!.send('Hello world')
    }
    socket.onclose = (ev) => {
      console.log('CLOSING WS')
      wsError = true
      // console.log('close', ev)
    }
    socket.onmessage = ({ data }) => {
      let msg: GameEvent = JSON.parse(data)
      console.log(msg)
      switch (msg.event) {
        case 'Game':
          gameStartAudio.play()
          game = msg.game
          player =
            msg.game.x === user.user
              ? 'x'
              : msg.game.o === user.user
                ? 'o'
                : null
          break
        case 'MoveEvent':
          if (game === null) break
          game.moves.push(msg.mv)
          game.board[msg.mv.position.row][msg.mv.position.col] = msg.mv.player
          if (player !== msg.mv.player) {
            if (msg.mv.player === 'o') {
              xAudio.play()
            } else {
              oAudio.play()
            }
          }
          game.next_player = msg.mv.player === 'x' ? 'o' : 'x'
          predicts = []
          break
        case 'Winner':
          if (game === null) break
          if (game === null) return
          if (msg.last_move.player === player) {
            victoryAudio.play()
          } else {
            defeatedAudio.play()
          }
          game.board[msg.last_move.position.row][msg.last_move.position.col] =
            msg.last_move.player
          game.moves.push(msg.last_move)
          game.status = 'ready'
          game.x_ready = false
          game.o_ready = false
          predicts = []
          game.winner = msg.moves
          break

        case 'MiniMax':
          if (game === null) return
          // predicts.push({ ...msg.position, score: msg.score })
          let idx = predicts.findIndex(
            (p) => p.row === msg.position.row && p.col === msg.position.col
          )
          if (idx === -1) {
            predicts.push({ ...msg.position, score: msg.score })
          } else {
            predicts[idx].score = msg.score
          }
          predicts = [...predicts]
          break

        case 'Status':
          if (game === null) break
          game.status = msg.status
          console.log(game)
          break
        case 'Message':
          messages.push(msg)
          if (msg.user !== user.user) unReadMessages += 1
          setTimeout(() => {
            chatBody?.scrollTo({
              top: chatBody.scrollHeight,
              behavior: 'smooth'
            })
          }, 0)
          break
        default:
          break
      }
    }
  })

  const play = async (row: number, col: number) => {
    if (game === null) return
    if (player === null) return
    if (game.next_player !== player) return
    if (game.winner !== null) return
    if (game.status !== 'playing') return
    xAudio.play()
    game.board[row][col] = 'x'
    game.moves.push({ position: { row, col }, player })
    game.next_player = 'o'

    game = game

    // game = game
    socket!.send(
      JSON.stringify({
        event: 'MoveEvent',
        mv: { position: { row, col }, player }
      })
    )
  }
</script>

<div
  class="sm:lex-row flex h-full max-h-screen w-full flex-col overflow-auto p-2 sm:p-4"
>
  <div
    class="flex h-full w-full flex-col items-center gap-4 overflow-auto sm:flex-row"
  >
    <div class="relative grid h-fit w-fit place-items-center">
      {#if game !== null}
        <GameRender {game} {play} {player} {predicts} />
        {#if game.winner && player !== null}
          <div class="absolute inset-0 flex justify-center bg-gray-900/60 p-24">
            <div
              in:fly={{ y: -100 }}
              class="grid h-fit w-96 place-items-center gap-4 rounded bg-white p-8"
            >
              <div
                class="inline-block bg-gradient-to-r from-blue-600 via-green-500 to-indigo-400 bg-clip-text text-6xl font-bold text-transparent"
              >
                {#if player === game.winner[0].player}
                  <div>{$_('won')}</div>
                {:else}
                  <div>{$_('lost')}</div>
                {/if}
              </div>
              <Button
                variant="destructive"
                on:click={() => {
                  socket!.send(JSON.stringify({ event: 'PlayAgain' }))
                }}>Play Again</Button
              >
              <a href="/" use:link>Exit</a>
            </div>
          </div>
        {/if}
      {/if}
    </div>
    <div class="block sm:hidden">
      <Drawer.Root>
        <Drawer.Trigger>
          <Button class="absolute bottom-4 left-4" variant="outline">
            <MessageSquareMore />
            {#if unReadMessages > 0}
              <div
                class="absolute right-0 top-0 h-4 w-4 rounded-full bg-red-400 text-xs"
              >
                {unReadMessages}
              </div>
            {/if}
          </Button>
        </Drawer.Trigger>
        <Drawer.Content class="h-full">
          <Drawer.Header>
            <Drawer.Title>{$_('trash-talk')}</Drawer.Title>
          </Drawer.Header>
          <div class="h-full grow">
            <Chat {messages} {socket} {chatBody} />
          </div>
        </Drawer.Content>
      </Drawer.Root>
    </div>

    <div class="hidden h-full grow sm:block">
      <Chat {messages} {socket} {chatBody} />
    </div>
  </div>
</div>

{#if wsError}
  <div class="absolute inset-0 flex justify-center bg-gray-900/90 p-24">
    <div
      in:fly={{ y: -100 }}
      class="grid h-fit w-96 place-items-center gap-4 rounded bg-white p-8"
    >
      <div class="">
        <div class="text-2xl font-bold text-red-400">
          {$_('there-is-something-wrong')}
        </div>
        <a href="/" use:link>
          <Button variant="link">
            {$_('to-main-menu')}
          </Button>
        </a>
      </div>
    </div>
  </div>
{/if}
