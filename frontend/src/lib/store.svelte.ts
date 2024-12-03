import axios from 'axios'

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
        axios
          .get('http://localhost:11211/api/users')
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

export let client = axios.create({
  baseURL: 'http://localhost:11211'
})
