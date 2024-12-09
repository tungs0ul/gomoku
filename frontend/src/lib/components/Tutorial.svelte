<script lang="ts">
  import * as Card from '$lib/components/ui/card'
  import { Checkbox } from '$lib/components/ui/checkbox'
  import { Label } from '$lib/components/ui/label'
  import { Button } from '$lib/components/ui/button'
  import { _ } from 'svelte-i18n'

  let { onclick }: { onclick: () => void } = $props()
  let checked = $state(false)
</script>

<Card.Root>
  <Card.Header>
    <Card.Title>{$_('tutorial-title')}</Card.Title>
    <Card.Description>{$_('tutorial-description')}</Card.Description>
  </Card.Header>
  <Card.Content>
    <div class="grid gap-4">
      <div>{$_('gomoku-objective')}</div>
      <div>
        <div class="mx-auto w-full max-w-md">
          <svg viewBox="0 0 400 400" class="h-auto w-full">
            <rect x="0" y="0" width="400" height="400" fill="#E6D2AA" />

            {#each Array.from({ length: 15 }) as arr, i}
              <line
                x1="20"
                y1={20 + i * 24}
                x2="380"
                y2={20 + i * 24}
                stroke="#8B4513"
                stroke-width="1" />
              <line
                x1={20 + i * 24}
                y1="20"
                x2={20 + i * 24}
                y2="380"
                stroke="#8B4513"
                stroke-width="1" />
            {/each}

            <line
              x1="380"
              y1="20"
              x2="380"
              y2="380"
              stroke="#8B4513"
              stroke-width="1" />
            <line
              x1="20"
              y1="380"
              x2="380"
              y2="380"
              stroke="#8B4513"
              stroke-width="1" />

            <circle cx="116" cy="116" r="10" fill="green" />
            <circle cx="164" cy="164" r="10" fill="red" />
            <circle cx="212" cy="212" r="10" fill="green" />
            <circle cx="260" cy="260" r="10" fill="red" />

            <circle cx="68" cy="68" r="10" fill="green" />
            <circle cx="92" cy="92" r="10" fill="green" />
            <circle cx="116" cy="116" r="10" fill="green" />
            <circle cx="140" cy="140" r="10" fill="green" />
            <circle cx="164" cy="164" r="10" fill="green" />

            <line
              x1="68"
              y1="68"
              x2="164"
              y2="164"
              stroke="rgba(255, 215, 0, 0.6)"
              stroke-width="8"
              stroke-linecap="round" />
          </svg>
        </div>
      </div>
    </div>
  </Card.Content>
  <Card.Footer>
    <div class="flex w-full justify-between gap-4">
      <div class="flex items-center space-x-2">
        <Checkbox id="terms" bind:checked aria-labelledby="terms-label" />
        <Label
          id="terms-label"
          for="terms"
          class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
          {$_('dont-show-this-anymore')}
        </Label>
      </div>

      <Button
        on:click={() => {
          if (checked) {
            localStorage.setItem('tutorial', 'false')
          }
          onclick()
        }}>{$_('play-now')}</Button>
    </div>
  </Card.Footer>
</Card.Root>
