<script lang="ts">
  import { user } from '$lib/store.svelte'
  import Button from '$lib/components/ui/button/button.svelte'
  import { fly } from 'svelte/transition'
  import { Input } from '$lib/components/ui/input'
  import SendHorizontal from 'lucide-svelte/icons/send-horizontal'
  import { _ } from 'svelte-i18n'

  let {
    messages,
    socket,
    chatBody
  }: {
    messages: { msg: string; user: string; id: string }[]
    socket: WebSocket | null
    chatBody: HTMLDivElement | null
  } = $props()
</script>

<div
  class="flex h-full w-full flex-col gap-4 overflow-hidden rounded border bg-[#FFF5D7] p-4">
  <div bind:this={chatBody} class="flex grow flex-col overflow-auto">
    {#each messages as msg, i (msg.id)}
      <div
        in:fly={{ x: user.user === msg.user ? 50 : -50 }}
        class="flex flex-col">
        {#if i === 0 || msg.user !== messages[i - 1].user}
          <span
            class:self-start={msg.user !== user.user}
            class:self-end={msg.user === user.user}
            class="mb-1 mt-4 text-sm text-gray-500">
            {msg.user.slice(0, 2)}
          </span>
        {/if}
        <div
          class={`mb-2 whitespace-pre-wrap rounded-2xl border px-4 py-2 shadow ${
            msg.user === user.user
              ? 'rounded-rb-0 self-end border-green-200 bg-green-100'
              : 'rounded-lb-0 self-start border-gray-200 bg-white'
          }`}>
          {msg.msg}
        </div>
      </div>
    {/each}
  </div>
  <form
    class="flex items-center"
    on:submit={(e) => {
      e.preventDefault()
      const formData = new FormData(e.currentTarget)
      const msg = formData.get('message')
      if (!msg) {
        return
      }
      socket?.send(
        JSON.stringify({ event: 'Chat', msg, user: '', id: user.user })
      )
      e.currentTarget.reset()
    }}>
    <Input name="message" placeholder={$_('enter-to-trash-talk')} />
    <Button size="sm" type="submit" variant="ghost">
      <SendHorizontal class="h-12 w-12" />
    </Button>
  </form>
</div>
