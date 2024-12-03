<script>
  import './app.css'
  import { Button } from '$lib/components/ui/button'
  import { Toaster } from '$lib/components/ui/sonner'
  import axios from 'axios'
  import { user } from '$lib/store.svelte'
  import { isLoading } from 'svelte-i18n'
  import Navbar from '$lib/Navbar.svelte'
  import { _ } from 'svelte-i18n'
  import Router from 'svelte-spa-router'
  import { wrap } from 'svelte-spa-router/wrap'
  import Home from '$lib/pages/Home.svelte'
  import NotFound from '$lib/pages/NotFound.svelte'
  import MobileNavbar from '$lib/MobileNavbar.svelte'

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

    '/play/*': wrap({
      // @ts-ignore
      asyncComponent: () => import('$lib/pages/Play.svelte')
    }),

    // Catch-all route last
    '*': NotFound
  }

  user.init()
  const createRoom = async () => {
    if (user.user === null) {
      return
    }
    axios
      .post('http://localhost:11211/api/rooms', { user_id: user.user })
      .then(({ data }) => {
        const socket = new WebSocket(
          `ws://localhost:11211/ws/${data}/${user.user}`
        )
        socket.onopen = () => {
          console.log('open')
          socket.send('Hello world')
        }
        socket.onmessage = ({ data }) => {
          console.log('msg', data)
        }
      })
  }

  const getRooms = async () => {
    const { data } = await axios.get('http://localhost:11211/api/rooms')
  }

  // $effect(() => {
  //   getRooms()
  // })
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
    <main class="grid grow place-items-center p-2">
      <Router {routes} />
    </main>
  </div>
{/if}
