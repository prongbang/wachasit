import { ref } from 'vue'

const path = ref(window.location.pathname)

window.addEventListener('popstate', () => {
  path.value = window.location.pathname
})

export function useRoute() {
  return path
}

export function navigate(to: string) {
  if (to === path.value) return
  window.history.pushState({}, '', to)
  path.value = to
}
