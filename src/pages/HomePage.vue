<script setup lang="ts">
import { Lock, ShieldCheck, Sparkles, Zap } from '@lucide/vue'
import ProductSection from '@/components/launchpad/ProductSection.vue'

const features = [
  {
    icon: Sparkles,
    title: 'AI enhancement',
    description: 'Upscale images and video 2x, 3x, or 4x with a model tuned for clean, natural detail.',
  },
  {
    icon: Zap,
    title: 'Fast by default',
    description: 'Runs in the browser with no install and no long render queue for everyday sizes.',
  },
  {
    icon: Lock,
    title: 'Privacy-first',
    description: 'No account required. Your files are processed for the job at hand, not stored for training.',
  },
]
</script>

<template>
  <main class="hide-scrollbar mx-auto flex w-full max-w-6xl flex-col gap-10 overflow-y-auto px-4 py-8 sm:px-6 lg:px-8">
    <header
      class="animate-in fade-in slide-in-from-bottom-3 relative overflow-hidden rounded-[2rem] border border-white/20 bg-gradient-to-br from-slate-950 via-blue-950 to-cyan-900 p-8 text-white shadow-[0_24px_64px_-28px_rgba(2,6,23,0.85)] duration-500 sm:p-12"
    >
      <div class="pointer-events-none absolute inset-0 opacity-20">
        <div class="absolute -top-16 left-1/3 h-40 w-40 rounded-full bg-cyan-300 blur-3xl"></div>
        <div class="absolute -bottom-20 right-0 h-48 w-48 rounded-full bg-blue-400 blur-3xl"></div>
      </div>
      <div class="relative mx-auto flex max-w-2xl flex-col items-center text-center">
        <p class="text-xs font-semibold uppercase tracking-[0.24em] text-cyan-200/80">Products by INTENIQUETIC</p>
        <h1 class="mt-2 text-3xl font-extrabold tracking-tight sm:text-4xl lg:text-5xl">
          Practical software, built with privacy in mind
        </h1>
        <p class="mt-3 max-w-xl text-sm text-slate-100/90 sm:text-base">
          From AI-powered media upscaling to on-device PDF and file tools — fast software that keeps your data
          yours.
        </p>
        <div class="mt-6 flex flex-wrap items-center justify-center gap-3">
          <a
            href="#products"
            class="inline-flex items-center gap-2 rounded-xl bg-white px-4 py-2 text-sm font-semibold text-slate-900 shadow-sm transition hover:-translate-y-0.5 hover:bg-slate-100"
          >
            Explore products
          </a>
        </div>
        <p class="mt-6 flex flex-wrap items-center justify-center gap-x-4 gap-y-1 text-xs font-medium text-slate-200/80">
          <span class="inline-flex items-center gap-1.5"><ShieldCheck :size="14" /> No account required</span>
        </p>
      </div>
    </header>

    <section class="animate-in fade-in slide-in-from-bottom-3 grid gap-4 duration-500 sm:grid-cols-3">
      <div
        v-for="feature in features"
        :key="feature.title"
        class="rounded-3xl border border-slate-200/80 bg-white/85 p-5 shadow-[0_12px_40px_-20px_rgba(15,23,42,0.35)] backdrop-blur transition hover:-translate-y-0.5 hover:shadow-[0_18px_46px_-30px_rgba(15,23,42,0.55)] dark:border-slate-800/80 dark:bg-slate-900/75"
      >
        <div
          class="grid h-10 w-10 place-items-center rounded-xl border border-cyan-200 bg-cyan-50 text-cyan-700 dark:border-cyan-900/80 dark:bg-cyan-950/70 dark:text-cyan-300"
        >
          <component :is="feature.icon" :size="20" />
        </div>
        <h3 class="mt-4 text-base font-bold text-slate-900 dark:text-slate-100">{{ feature.title }}</h3>
        <p class="mt-1 text-sm text-slate-600 dark:text-slate-400">{{ feature.description }}</p>
      </div>
    </section>

    <ProductSection id="products" class="scroll-mt-20" />
  </main>
</template>
