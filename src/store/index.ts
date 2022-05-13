import { createStore } from 'vuex'
import { tauri } from '@tauri-apps/api'

const state = {}
const actions = {
    get_isa_store ({ commit }) {
        tauri.invoke('get_isa_store')
            .then((res) => {
                console.log(res)
            })
            .catch((e: Error) => console.error(e))
    } 
}

export default createStore({
    state,
    actions,
})