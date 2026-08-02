<script setup lang="ts">
import { computed, ref } from 'vue'
import { ArrowRight, Check, Lock, ShieldCheck, Sparkles, Zap } from '@lucide/vue'
import Badge from '@/components/ui/Badge.vue'
import Button from '@/components/ui/Button.vue'
import AppLink from '@/components/ui/AppLink.vue'
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

function open(url: string) {
  window.open(url, '_blank')
}

interface Plan {
  name: string
  price: string
  period: string
  description: string
  features: string[]
  cta: string
  href: string
  variant: 'primary' | 'secondary'
  featured?: boolean
}

const productPricing: { key: string; name: string; tagline: string; plans: Plan[] }[] = [
  {
    key: 'ai-upscaler',
    name: 'AI Upscaler',
    tagline: 'AI image & video upscaling, 2x to 4x.',
    plans: [
      {
        name: 'Free',
        price: '$0',
        period: 'forever',
        description: 'Try AI Upscaler in your browser.',
        features: ['2x upscale', 'Image only', 'Standard queue', 'Community support'],
        cta: 'Start for free',
        href: 'https://prongbang.github.io/ai-upscaler/',
        variant: 'secondary' as const,
      },
      {
        name: 'Pro',
        price: '$9',
        period: '/month',
        description: 'For creators who upscale often.',
        features: ['2x, 3x, 4x upscale', 'Image & video', 'Priority queue', 'Email support'],
        cta: 'Contact sales',
        href: 'mailto:dev.prongbang@gmail.com?subject=AI%20Upscaler%20Pro',
        variant: 'primary' as const,
        featured: true,
      },
      {
        name: 'Business',
        price: '$29',
        period: '/month',
        description: 'For teams and studios.',
        features: ['Everything in Pro', 'Bulk processing', 'Dedicated support', 'Usage invoicing'],
        cta: 'Contact sales',
        href: 'mailto:dev.prongbang@gmail.com?subject=AI%20Upscaler%20Business',
        variant: 'secondary' as const,
      },
    ],
  },
  {
    key: 'tools',
    name: 'Tools',
    tagline: 'Local-first PDF, QR, image & zip utilities for Android.',
    plans: [
      {
        name: 'Free',
        price: '$0',
        period: 'forever',
        description: 'Available now on Google Play.',
        features: ['PDF / QR / Image / Zip', 'Works offline', 'No account'],
        cta: 'Get on Google Play',
        href: 'https://play.google.com/store/apps/details?id=com.inteniquetic.tools&hl=en',
        variant: 'primary' as const,
      },
    ],
  },
  {
    key: 'pdf-pro-x',
    name: 'PDF Pro X',
    tagline: 'Manage PDFs on-device, built for privacy and security.',
    plans: [
      {
        name: 'Free',
        price: '$0',
        period: 'forever',
        description: 'Available now on Google Play.',
        features: ['On-device PDF tools', 'Privacy & security', 'Fast & lightweight'],
        cta: 'Get on Google Play',
        href: 'https://play.google.com/store/apps/details?id=tech.otel.pdfprox&hl=en',
        variant: 'primary' as const,
      },
    ],
  },
]

const activeProductKey = ref(productPricing[0].key)
const activeProduct = computed(
  () => productPricing.find((product) => product.key === activeProductKey.value) ?? productPricing[0],
)
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
          <a
            href="#pricing"
            class="inline-flex items-center gap-2 rounded-xl px-4 py-2 text-sm font-semibold text-white/90 transition hover:text-white"
          >
            See pricing
            <ArrowRight :size="16" />
          </a>
        </div>
        <p class="mt-6 flex flex-wrap items-center justify-center gap-x-4 gap-y-1 text-xs font-medium text-slate-200/80">
          <span class="inline-flex items-center gap-1.5"><ShieldCheck :size="14" /> No account required</span>
          <span class="inline-flex items-center gap-1.5"><Lock :size="14" /> Secure payments via Stripe</span>
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

    <section id="pricing" class="scroll-mt-20 space-y-8">
      <header class="animate-in fade-in slide-in-from-bottom-3 text-center duration-500">
        <p class="text-xs font-semibold uppercase tracking-[0.24em] text-cyan-700 dark:text-cyan-300">Pricing</p>
        <h2 class="mt-1 text-2xl font-extrabold tracking-tight text-slate-950 dark:text-white">
          Simple plans, cancel anytime
        </h2>
        <p class="mx-auto mt-2 max-w-xl text-sm text-slate-600 dark:text-slate-300">
          Pick a product to see its plans. Prices in USD.
        </p>
      </header>

      <div class="flex flex-wrap items-center justify-center gap-2">
        <button
          v-for="product in productPricing"
          :key="product.key"
          type="button"
          :class="[
            'rounded-full border px-4 py-2 text-sm font-semibold transition hover:-translate-y-0.5',
            activeProductKey === product.key
              ? 'border-slate-900 bg-slate-900 text-white shadow-sm dark:border-white dark:bg-white dark:text-slate-900'
              : 'border-slate-300/80 bg-white text-slate-600 hover:bg-slate-50 dark:border-slate-700 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800',
          ]"
          @click="activeProductKey = product.key"
        >
          {{ product.name }}
        </button>
      </div>

      <div :key="activeProduct.key" class="animate-in fade-in slide-in-from-bottom-2 space-y-4 duration-300">
        <p class="text-center text-sm text-slate-500 dark:text-slate-400">{{ activeProduct.tagline }}</p>

        <div
          :class="[
            'grid gap-4',
            activeProduct.plans.length === 1 ? 'mx-auto max-w-sm' : 'lg:grid-cols-3',
          ]"
        >
          <article
            v-for="plan in activeProduct.plans"
            :key="plan.name"
            :class="[
              'flex flex-col gap-5 rounded-3xl border p-6 shadow-[0_18px_46px_-30px_rgba(15,23,42,0.55)] transition hover:-translate-y-0.5',
              plan.featured
                ? 'border-cyan-300 bg-white ring-2 ring-cyan-500/60 dark:border-cyan-800 dark:bg-slate-900'
                : 'border-slate-200/80 bg-white/90 dark:border-slate-800/80 dark:bg-slate-900/80',
            ]"
          >
            <div class="flex items-center justify-between gap-2">
              <h3 class="text-lg font-extrabold tracking-tight text-slate-950 dark:text-white">{{ plan.name }}</h3>
              <Badge v-if="plan.featured" label="Popular" />
            </div>
            <p class="text-sm text-slate-600 dark:text-slate-300">{{ plan.description }}</p>
            <p class="text-3xl font-extrabold tracking-tight text-slate-950 dark:text-white">
              {{ plan.price }}
              <span class="text-sm font-semibold text-slate-500 dark:text-slate-400">{{ plan.period }}</span>
            </p>
            <ul class="flex-1 space-y-2">
              <li
                v-for="item in plan.features"
                :key="item"
                class="flex items-center gap-2 text-sm text-slate-600 dark:text-slate-300"
              >
                <Check :size="16" class="shrink-0 text-emerald-600 dark:text-emerald-400" />
                {{ item }}
              </li>
            </ul>
            <Button :label="plan.cta" :variant="plan.variant" @click="open(plan.href)" />
          </article>
        </div>
      </div>

      <p class="flex items-center justify-center gap-1.5 text-center text-xs text-slate-500 dark:text-slate-400">
        <Lock :size="12" /> Paid plans are processed securely by Stripe. See our
        <AppLink to="/privacy" class="font-semibold text-slate-700 underline hover:text-slate-900 dark:text-slate-300 dark:hover:text-white"
          >Privacy Policy</AppLink
        >.
      </p>
    </section>
  </main>
</template>
