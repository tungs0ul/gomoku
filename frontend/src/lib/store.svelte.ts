import { createClient, type AuthSession } from '@supabase/supabase-js'
import axios from 'axios'

export const SUPABASE_URL = 'https://kong.sansantech.de'
export const BACKEND_URL = 'http://localhost:11211/api'

export const ANON_KEY =
  'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ewogICJyb2xlIjogImFub24iLAogICJpc3MiOiAic3VwYWJhc2UiLAogICJpYXQiOiAxNzMzNDM5NjAwLAogICJleHAiOiAxODkxMjA2MDAwCn0.mSdKDeE3nA0Ae4rPcTSGZv8KfOIf0ZJLHj3jG5D6RJo'

export const supabase = createClient(SUPABASE_URL, ANON_KEY)

const createAuth = () => {
  let state = $state<AuthSession | null>(null)
  let api = $derived.by(() => {
    return axios.create({
      baseURL: BACKEND_URL,
      headers: {
        apiKey: ANON_KEY,
        Authorization: `Bearer ${state?.access_token}`
      }
    })
  })

  return {
    get auth() {
      return state
    },
    set(newState: AuthSession | null) {
      state = newState
    },
    get apiClient() {
      return api
    }
  }
}

export const auth = createAuth()
supabase.auth.onAuthStateChange((_event, session) => {
  auth.set(session)
})

// export const API_URL = 'https://api.sansantech.de/api'
// export const WS_URL = 'wss://api.sansantech.de'
export const WS_URL = 'ws://localhost:11211'
// export const WS_URL = 'wss://kong.sansantech.de/backend/v1'

export let api = {
  get_user: '/users',
  play: '/games',
  get_rooms: '/rooms'
}
