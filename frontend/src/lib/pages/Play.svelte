<script lang="ts">
  import { HOST, PORT, user } from '$lib/store.svelte'
  import type { Game, GameEvent, Move, Player } from '$lib/types'
  import { toast } from 'svelte-sonner'
  import { _ } from 'svelte-i18n'
  import GameRender from '$lib/Game.svelte'
  import { link, location } from 'svelte-spa-router'
  import { onDestroy, onMount } from 'svelte'
  import PlayerXSound from '$lib/assets/sound/player-action-x.mp3'
  import PlayerOSound from '$lib/assets/sound/player-action-o.mp3'
  import VictorySound from '$lib/assets/sound/victory-2.mp3'
  import DefeatedSound from '$lib/assets/sound/defeat.mp3'
  import GameStartSound from '$lib/assets/sound/game-start.mp3'
  import Button from '$lib/components/ui/button/button.svelte'
  import { fly, scale } from 'svelte/transition'

  let game = $state<Game | null>(null)
  let predicts = $state<{ row: number; col: number; score: number }[]>([])
  let socket = $state<WebSocket | null>(null)
  let player = $state<Player | null>(null)
  let playAgain = $state(false)

  let xAudio = new Audio(PlayerXSound)
  let oAudio = new Audio(PlayerOSound)
  let victoryAudio = new Audio(VictorySound)
  let defeatedAudio = new Audio(DefeatedSound)
  let gameStartAudio = new Audio(GameStartSound)

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
      // console.log('close', ev)
    }
    socket.onmessage = ({ data }) => {
      let msg: GameEvent = JSON.parse(data)
      if (msg.event === 'Game') {
        gameStartAudio.play()
        game = msg.game
        player =
          msg.game.x === user.user ? 'x' : msg.game.o === user.user ? 'o' : null
      }
      if (msg.event === 'MoveEvent') {
        if (game === null) return
        // console.log(
        //   `game.board[${msg.mv.position.row}][${msg.mv.position.col}] = Some(Player::${msg.mv.player})`
        // )
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
      }
      if (msg.event === 'Winner') {
        if (game === null) return
        if (msg.last_move.player === player) {
          victoryAudio.play()
        } else {
          defeatedAudio.play()
        }
        game.board[msg.last_move.position.row][msg.last_move.position.col] =
          msg.last_move.player
        game.moves.push(msg.last_move)
        predicts = []
        game.winner = msg.moves
      }
      if (msg.event === 'InvalidMove') {
        if (game === null) return
        game.moves.pop()
      }
      if (msg.event === 'MiniMax') {
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
      }
      // console.log(msg)
    }
  })

  const play = async (row: number, col: number) => {
    if (game === null) return
    if (player === null) return
    if (game.next_player !== player) return
    if (game.winner !== null) return
    xAudio.play()
    game.board[row][col] = 'x'
    game.moves.push({ position: { row, col }, player })
    game.next_player = 'o'

    game = game

    // game = game
    socket!.send(
      JSON.stringify({ event: 'PredictBot', position: { row, col } })
    )
  }
</script>

<div class="sm:lex-row flex h-full w-full flex-col p-2 sm:p-4">
  <div class="flex flex-col sm:flex-row">
    <div class="grid h-full w-full place-items-center">
      {#if game !== null}
        <GameRender {game} {play} {player} {predicts} />
        {#if game.winner && player !== null}
          <div class="absolute inset-0 flex justify-center bg-gray-400/50 p-24">
            <div
              in:fly={{ y: -100 }}
              class="grid h-fit place-items-center gap-4 rounded bg-white p-8">
              <div
                class="inline-block bg-gradient-to-r from-blue-600 via-green-500 to-indigo-400 bg-clip-text text-6xl font-bold text-transparent">
                {#if player === game.winner[0].player}
                  <div>You Won</div>
                {:else}
                  <div>You Lost</div>
                {/if}
              </div>
              <Button
                variant="destructive"
                on:click={() => {
                  socket!.send(JSON.stringify({ event: 'PlayAgain' }))
                }}>Play Again</Button>
              <a href="/" use:link>Exit</a>
            </div>
          </div>
        {/if}
      {/if}
    </div>
    <div class="grow">history</div>
  </div>
</div>
