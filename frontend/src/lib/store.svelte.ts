import axios from 'axios'

export let client = axios.create({
  baseURL: 'http://localhost:11211'
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
      console.log('user', user)
      if (user === null) {
        client
          .post('/user')
          .then(({ data }: { data: string }) => {
            console.log('user', data)
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


