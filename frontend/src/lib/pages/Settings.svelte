<script lang="ts">
  import * as Card from '$lib/components/ui/card/index.js'
  import { auth } from '$lib/store.svelte'
  import User from 'lucide-svelte/icons/user'
</script>

<div class="grid place-items-center">
  <Card.Root>
    <Card.Header>
      <Card.Title>
        {#if auth.auth?.user.user_metadata.avatar_url}
          <img
            class="h-32 w-32 rounded-full"
            src={auth.auth?.user.user_metadata.avatar_url ??
              'https://www.flaticon.com/free-icons/user'}
            alt="avatar"
          />
        {:else}
          <User class="h-32 w-32 rounded-full" />
        {/if}
      </Card.Title>
    </Card.Header>
    <Card.Content>
      {auth.auth?.user.user_metadata.name ?? 'Anonymous'}
    </Card.Content>
    <Card.Footer>
      {auth.auth?.user.email ?? 'Email'}
    </Card.Footer>
  </Card.Root>
</div>
