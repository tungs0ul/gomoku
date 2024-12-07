<script lang="ts">
  import { auth } from '$lib/store.svelte'
  import Button from '$lib/components/ui/button/button.svelte'
  import { fly } from 'svelte/transition'
  import { Input } from '$lib/components/ui/input'
  import SendHorizontal from 'lucide-svelte/icons/send-horizontal'
  import { _ } from 'svelte-i18n'
  import type { Message } from '$lib/types'

  let {
    messages,
    socket,
    chatBody
  }: {
    messages: Message[]
    socket: WebSocket | null
    chatBody: HTMLDivElement | null
  } = $props()
</script>

<div
  class="flex h-full w-full flex-col gap-4 overflow-hidden rounded border bg-[#FFF5D7] p-4">
  <div bind:this={chatBody} class="flex grow flex-col overflow-auto">
    {#each messages as msg, i (msg.id)}
      <div
        in:fly={{ x: auth.auth?.user.id === msg.user ? 50 : -50 }}
        class="flex flex-col">
        {#if i === 0 || msg.user !== messages[i - 1].user}
          <span
            class:self-start={msg.user !== auth.auth?.user.id}
            class:self-end={msg.user === auth.auth?.user.id}
            class="mb-1 mt-4 text-sm text-gray-500">
            {msg.user?.slice(0, 2) ?? ''}
          </span>
        {/if}
        {#if msg.user !== null}
          <div
            class={`mb-2 whitespace-pre-wrap rounded-2xl border px-4 py-2 shadow ${
              msg.user === auth.auth?.user.id
                ? 'rounded-rb-0 self-end border-green-200 bg-green-100'
                : 'rounded-lb-0 self-start border-gray-200 bg-white'
            }`}>
            {msg.msg}
          </div>
        {:else}
          <span class="mb-1 mt-4 self-center text-sm text-gray-500">
            {msg.msg}
          </span>
        {/if}
      </div>
    {/each}
  </div>
  <form
    class="flex items-center"
    onsubmit={(e) => {
      e.preventDefault()
      const formData = new FormData(e.currentTarget)
      const msg = formData.get('message')
      if (!msg) {
        return
      }
      socket?.send(
        JSON.stringify({
          event: 'Message',
          msg,
          user: auth.auth?.user?.user_metadata?.name ?? $_('anonymous'),
          id: auth.auth?.user.id
        })
      )
      e.currentTarget.reset()
    }}>
    <Input name="message" placeholder={$_('enter-to-trash-talk')} />
    <Button size="sm" type="submit" variant="ghost">
      <SendHorizontal class="h-12 w-12" />
    </Button>
  </form>
</div>
