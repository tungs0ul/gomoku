<script>
  import './app.css'
  import { Toaster } from '$lib/components/ui/sonner'
  import { user } from '$lib/store.svelte'
  import { isLoading } from 'svelte-i18n'
  import Navbar from '$lib/Navbar.svelte'
  import { _ } from 'svelte-i18n'
  import Router from 'svelte-spa-router'
  import { wrap } from 'svelte-spa-router/wrap'
  import Home from '$lib/pages/Home.svelte'
  import NotFound from '$lib/pages/NotFound.svelte'
  import MobileNavbar from '$lib/MobileNavbar.svelte'
  import { QueryClientProvider, QueryClient } from '@tanstack/svelte-query'
  import { SvelteQueryDevtools } from '@tanstack/svelte-query-devtools'

  const queryClient = new QueryClient()

  const routes = {
    '/': Home,

    '/sign-in': wrap({
      // @ts-ignore
      asyncComponent: () => import('$lib/pages/SignIn.svelte')
    }),

    '/rooms': wrap({
      // @ts-ignore
      asyncComponent: () => import('$lib/pages/Rooms.svelte')
    }),

    '/ws/*': wrap({
      // @ts-ignore
      asyncComponent: () => import('$lib/pages/Play.svelte')
    }),

    // Catch-all route last
    '*': NotFound
  }

  user.init()
</script>

<!-- <button onclick={createRoom}>Create room</button> -->

{#if $isLoading}
  Please wait...
{:else}
  <Toaster />
  <div class="flex h-screen flex-col bg-[#1E3E62] sm:flex-row">
    <div class="block sm:hidden">
      <MobileNavbar />
    </div>
    <div class="hidden h-full sm:block">
      <Navbar />
    </div>
    <QueryClientProvider client={queryClient}>
      {#if user.user !== null}
        <main class="relative grid grow place-items-center p-2">
          <Router {routes} />
        </main>
      {/if}
      <SvelteQueryDevtools />
    </QueryClientProvider>
  </div>
{/if}
