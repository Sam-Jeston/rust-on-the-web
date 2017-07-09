const axios = require('axios')

export function login(username: string, password: string) {
  return axios.post('/login', {username, password}).then((d: AxiosData) => d.data)
}

export function signup(username: string, password: string) {
  return axios.post('/signup', {username, password}).then((d: AxiosData) => d.data)
}
