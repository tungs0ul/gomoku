import axios from 'axios'

export const ANON_KEY = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ewogICJyb2xlIjogImFub24iLAogICJpc3MiOiAic3VwYWJhc2UiLAogICJpYXQiOiAxNzMzNDM5NjAwLAogICJleHAiOiAxODkxMjA2MDAwCn0.mSdKDeE3nA0Ae4rPcTSGZv8KfOIf0ZJLHj3jG5D6RJo'



// export const API_URL = 'https://api.sansantech.de/api'
export const API_URL = 'https://kong.sansantech.de/backend/v1/api'
// export const WS_URL = 'wss://api.sansantech.de'
export const WS_URL = 'wss://kong.sansantech.de/backend/v1'


export let api = {
  get_user: '/users',
  play: '/games',
  get_rooms: '/rooms'
}

export let client = axios.create({
  baseURL: `${API_URL}`,
  headers: {
    "apiKey": ANON_KEY
  }
})

const createUser = () => {
  let state = $state<string | null>(null)

  return {
    set(id: string) {
      state = id
    },
    get user() {
      return state
    },
    init() {
      if (state !== null) {
        return
      }
      let user = localStorage.getItem('user_id')
      if (user === null) {
        client
          .post(api.get_user)
          .then(({ data }: { data: string }) => {
            state = data
            localStorage.setItem('user_id', data)
          })
          .catch((err) => {
            console.error(err)
          })
      }
      if (user !== null) {
        state = user
      }
    }
  }
}

export const user = createUser()
