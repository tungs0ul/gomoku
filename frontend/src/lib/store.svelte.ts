import axios from 'axios'

export const HOST = 'localhost'
export const PORT = 11211

export let api = {
  get_user: 'user',
  create_game: '/games',
  get_rooms: '/rooms'
}

export let client = axios.create({
  baseURL: `http://${HOST}:${PORT}`
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
            user = data
            localStorage.setItem('user_id', user)
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
