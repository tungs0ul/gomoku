<script lang="ts">
  import { type Game, type Move, type Player } from '$lib/types'
  import { scale } from 'svelte/transition'

  let {
    game = $bindable(),
    play,
    player,
    predicts
  }: {
    game: Game
    play: (row: number, col: number) => Promise<void>
    player: Player | null
    predicts: { row: number; col: number; score: number }[]
  } = $props()

  // const findIndex = (x: number, y: number) => {
  //   let idx = game.moves.findIndex(
  //     (move) => move.position.row === y && move.position.col === x
  //   )
  //   return idx === -1 ? null : idx
  // }

  const findPredict = (x: number, y: number) => {
    let predict = predicts.find((p) => p.row === y && p.col === x)
    if (predict === undefined) {
      return { score: '', text: '' }
    }
    let text = 'bg-red-400/75 text-white'
    if (predict.score < -10) {
      text = 'bg-red-400/75 text-white'
    } else if (predict.score < 0) {
      text = 'bg-yellow-400/75 text-white'
    } else if (predict.score < 10) {
      text = 'bg-blue-400/75 text-white'
    } else {
      text = 'bg-green-400/75 text-white'
    }
    return {
      score: predict.score,
      text
    }
  }

  const getStyle = (col: number, row: number) => {
    let border =
      game.board[row][col] === 'x' ? 'border-gray-700 ' : 'border-gray-200 '
    let base =
      'grid aspect-square place-items-center rounded-full border-2 shadow-xl ' +
      border
    if (
      game.winner?.find(
        (m) => m.position.row === row && m.position.col === col
      ) !== undefined
    ) {
      return (
        base +
        'bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500 animate-bounce'
      )
    }

    let lastMove = game.moves[game.moves.length - 1]
    if (lastMove.position.row === row && lastMove.position.col === col) {
      return (
        base +
        (game.board[row][col] === player && player !== null
          ? 'bg-green-400'
          : 'bg-red-400') +
        ' border-4 border-indigo-500/100'
      )
    }
    if (player === null && game.board[row][col] === 'x') {
      return base + 'border-gray-700 bg-black'
    }
    if (player === null && game.board[row][col] === 'o') {
      return base + 'border-gray-200 bg-white'
    }

    if (game.board[row][col] === player) {
      return base + 'border-black bg-green-400'
    }
    if (game.board[row][col] !== player) {
      return base + 'border-black bg-red-400'
    }
    return base
  }
</script>

<div
  class="-z-9 shadow-xk absolute grid aspect-square animate-bounce place-items-center rounded-full border border-4 border-black border-indigo-500/100 bg-white bg-yellow-400 bg-gradient-to-r bg-gradient-to-r from-indigo-500 from-indigo-500 via-purple-500 via-purple-500 to-pink-500 to-pink-500 text-black text-black opacity-0"
></div>

<div class="grid h-full grow">
  {#each game.board as row, y (y)}
    <div
      class="flex w-full bg-[#B17457]"
      class:cursor-progress={player !== game.next_player}
      class:cursor-pointer={player === game.next_player}
    >
      {#each row as col, x (x)}
        {#if col === null}
          <button
            class:cursor-progress={player !== game.next_player}
            class:cursor-pointer={player === game.next_player}
            class={`aspect-square h-12 border p-0 text-xs ${findPredict(x, y).text} ${game.next_player === player ? 'hover:bg-[#F3C623]' : 'hover:bg-[#AB886D]'}`}
            onclick={() => {
              if (player === null) return
              play(y, x)
            }}
          >
            {findPredict(x, y).score}
          </button>
        {:else}
          <div class="aspect-square h-12 border p-2">
            <div class={getStyle(x, y)} in:scale>
              <!-- {col} -->
            </div>
          </div>
          <!-- <div class="">
            <div
              class="text-md grid aspect-square place-items-center border p-3 sm:text-xl"
            >
              <div
                in:scale
                class:animate-bounce={game.winner?.find(
                  (m) => m.position.row === y && m.position.col === x
                )}
                class:border-black={findIndex(x, y) !== null &&
                  findIndex(x, y)! % 2 == 0}
                class:border-white={findIndex(x, y) !== null &&
                  findIndex(x, y)! & 1}
                class={`grid aspect-square w-full animate-bounce place-items-center rounded-full border-2 shadow-xl ${
                  game.winner?.find(
                    (m) => m.position.row === y && m.position.col === x
                  )
                    ? 'bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500 text-black'
                    : findIndex(x, y) !== null
                      ? findIndex(x, y)! & 1
                        ? ' bg-white'
                        : 'bg-gray-900 text-white'
                      : ''
                }`}
              >
                {findIndex(x, y) !== null ? findIndex(x, y)! + 1 : ''}
              </div>
            </div>
          </div> -->
        {/if}
      {/each}
    </div>
  {/each}
</div>
