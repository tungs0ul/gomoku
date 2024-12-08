<script lang="ts">
  import * as Card from '$lib/components/ui/card/index.js'
  import { auth, supabase } from '$lib/store.svelte'
  import User from 'lucide-svelte/icons/user'
  import { Button } from '$lib/components/ui/button'
  import { _ } from 'svelte-i18n'
  import { replace } from 'svelte-spa-router'
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
            alt="avatar" />
        {:else}
          <User class="h-32 w-32 rounded-full" />
        {/if}
      </Card.Title>
    </Card.Header>
    <Card.Content>
      <div class="flex flex-col gap-4">
        <div>
          {auth.auth?.user.user_metadata.name ?? 'Anonymous'}
        </div>
        <div>
          {auth.auth?.user.email ?? 'Email'}
        </div>
      </div>
    </Card.Content>
    <Card.Footer>
      <Button
        variant="destructive"
        class="w-full"
        on:click={() => {
          supabase.auth.signOut().then(() => {
            replace('/')
          })
        }}>{$_('sign-out')}</Button>
    </Card.Footer>
  </Card.Root>
</div>
