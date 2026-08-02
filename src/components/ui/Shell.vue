<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import { Coffee, CupSoda, Heart, Moon, Rocket, Sun } from '@lucide/vue'
import AppLink from '@/components/ui/AppLink.vue'

const theme = ref<'light' | 'dark'>('light')

onMounted(() => {
  theme.value = document.documentElement.classList.contains('dark') ? 'dark' : 'light'
})

watch(theme, (mode) => {
  document.documentElement.classList.toggle('dark', mode === 'dark')
  localStorage.setItem('theme', mode)
})

function toggleTheme() {
  theme.value = theme.value === 'dark' ? 'light' : 'dark'
}
</script>

<template>
  <div
    class="min-h-screen bg-gradient-to-b from-slate-50 via-white to-slate-100 text-slate-900 dark:from-slate-950 dark:via-slate-950 dark:to-slate-900 dark:text-slate-100"
  >
    <header
      class="sticky top-0 z-30 border-b border-slate-200/80 bg-white/80 shadow-sm backdrop-blur-xl dark:border-slate-800/80 dark:bg-slate-900/70"
    >
      <div class="mx-auto flex w-full max-w-6xl items-center justify-between gap-4 px-4 py-4 sm:px-6 lg:px-8">
        <AppLink to="/" class="flex items-center gap-2">
          <span
            class="grid h-8 w-8 shrink-0 place-items-center rounded-xl bg-gradient-to-br from-cyan-500 to-blue-600 text-white shadow-sm"
          >
            <Rocket :size="16" />
          </span>
          <p class="text-sm font-bold text-slate-900 dark:text-slate-100">INTENIQUETIC</p>
        </AppLink>

        <nav class="flex items-center gap-1 text-sm font-semibold text-slate-600 dark:text-slate-300">
          <AppLink to="/port" class="hidden rounded-lg px-3 py-1.5 transition hover:bg-slate-100 dark:hover:bg-slate-800 sm:inline-block"
            >Developer Hub</AppLink
          >
          <AppLink to="/privacy" class="hidden rounded-lg px-3 py-1.5 transition hover:bg-slate-100 dark:hover:bg-slate-800 sm:inline-block"
            >Privacy</AppLink
          >

          <a
            href="https://ko-fi.com/prongbang"
            target="_blank"
            rel="noopener noreferrer"
            class="ml-1 inline-flex h-9 items-center gap-1.5 rounded-xl border border-slate-300/80 bg-white px-3 text-slate-700 shadow-sm transition hover:-translate-y-0.5 hover:bg-slate-50 dark:border-slate-700 dark:bg-slate-900 dark:text-slate-200 dark:hover:bg-slate-800"
            title="Support on Ko-fi"
          >
            <Coffee :size="16" />
            <span class="hidden sm:inline">Ko-fi</span>
          </a>
          <a
            href="https://buymeacoffee.com/prongbang"
            target="_blank"
            rel="noopener noreferrer"
            class="inline-flex h-9 items-center gap-1.5 rounded-xl border border-slate-300/80 bg-white px-3 text-slate-700 shadow-sm transition hover:-translate-y-0.5 hover:bg-slate-50 dark:border-slate-700 dark:bg-slate-900 dark:text-slate-200 dark:hover:bg-slate-800"
            title="Buy Me a Coffee"
          >
            <CupSoda :size="16" />
            <span class="hidden sm:inline">Coffee</span>
          </a>
          <a
            href="https://github.com/sponsors/prongbang"
            target="_blank"
            rel="noopener noreferrer"
            class="inline-flex h-9 items-center gap-1.5 rounded-xl border border-slate-300/80 bg-white px-3 text-slate-700 shadow-sm transition hover:-translate-y-0.5 hover:bg-slate-50 dark:border-slate-700 dark:bg-slate-900 dark:text-slate-200 dark:hover:bg-slate-800"
            title="Sponsor on GitHub"
          >
            <Heart :size="16" />
            <span class="hidden sm:inline">Sponsor</span>
          </a>

          <button
            type="button"
            class="ml-1 inline-flex h-9 w-9 items-center justify-center rounded-xl border border-slate-300/80 bg-white text-slate-700 shadow-sm transition hover:-translate-y-0.5 hover:bg-slate-50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-500/60 dark:border-slate-700 dark:bg-slate-900 dark:text-slate-200 dark:hover:bg-slate-800"
            :aria-label="theme === 'dark' ? 'Light mode' : 'Dark mode'"
            :title="theme === 'dark' ? 'Light mode' : 'Dark mode'"
            @click="toggleTheme"
          >
            <Sun v-if="theme === 'dark'" :size="18" />
            <Moon v-else :size="18" />
          </button>
        </nav>
      </div>
    </header>

    <slot />

    <footer
      class="border-t border-slate-200/80 bg-white/70 backdrop-blur dark:border-slate-800/80 dark:bg-slate-900/70"
    >
      <div
        class="mx-auto flex w-full max-w-6xl flex-col gap-3 px-4 py-6 text-xs text-slate-500 dark:text-slate-400 sm:flex-row sm:items-center sm:justify-between sm:px-6 lg:px-8"
      >
        <p>
          &copy; 2026 INTENIQUETIC ·
          <a href="mailto:dev.prongbang@gmail.com" class="hover:text-slate-700 dark:hover:text-slate-300"
            >dev.prongbang@gmail.com</a
          >
        </p>
        <div class="flex items-center gap-4 font-medium">
          <AppLink to="/privacy" class="hover:text-slate-700 dark:hover:text-slate-300">Privacy Policy</AppLink>
          <AppLink to="/port" class="hover:text-slate-700 dark:hover:text-slate-300">Developer Hub</AppLink>
          <a href="https://ko-fi.com/prongbang" target="_blank" rel="noopener noreferrer" class="hover:text-slate-700 dark:hover:text-slate-300">Ko-fi</a>
          <a href="https://buymeacoffee.com/prongbang" target="_blank" rel="noopener noreferrer" class="hover:text-slate-700 dark:hover:text-slate-300">Buy Me a Coffee</a>
          <a href="https://github.com/sponsors/prongbang" target="_blank" rel="noopener noreferrer" class="hover:text-slate-700 dark:hover:text-slate-300">Sponsor</a>
        </div>
      </div>
    </footer>
  </div>
</template>
