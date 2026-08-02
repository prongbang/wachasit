<script setup lang="ts">
const sections = [
  {
    title: '1. Information we collect',
    body: [
      'This website does not require an account. We do not run analytics or advertising trackers on it.',
      'Theme preference (light or dark) is stored only in your browser’s localStorage and never sent to our servers.',
      'If you email us, we receive your email address and whatever you include in your message.',
      'If you subscribe to a paid plan, payment is handled entirely by Stripe. We receive your name, email, and payment status from Stripe — we never see or store your full card number.',
    ],
  },
  {
    title: '2. Cookies & tracking',
    body: [
      'We do not use tracking or advertising cookies, and we do not run third-party analytics on this website.',
    ],
  },
  {
    title: '3. How we use information',
    body: [
      'To respond to your questions and support requests.',
      'To provide, maintain, and improve AI Upscaler and our other tools.',
      'To process payments, send receipts, and prevent fraud.',
      'To comply with legal obligations.',
    ],
  },
  {
    title: '4. Third-party services',
    body: [
      'Payments: Stripe (stripe.com/privacy) processes all payments on our behalf.',
      'Demo tools are hosted on GitHub Pages; Tools and PDF Pro X are distributed via Google Play. Each is governed by its own provider’s privacy policy.',
    ],
  },
  {
    title: '5. Data retention & security',
    body: [
      'We keep information only as long as needed for the purposes above, and use reasonable technical measures to protect it. No method of transmission or storage is 100% secure.',
    ],
  },
  {
    title: '6. Your rights',
    body: [
      'You can ask us to access, correct, or delete personal information we hold about you by emailing us at the address below.',
    ],
  },
  {
    title: '7. Children’s privacy',
    body: ['Our services are not directed at children under 13, and we do not knowingly collect their information.'],
  },
  {
    title: '8. Changes to this policy',
    body: ['We may update this policy from time to time. The date below reflects the latest revision.'],
  },
]
</script>

<template>
  <main class="hide-scrollbar mx-auto flex w-full max-w-3xl flex-col gap-6 overflow-y-auto px-4 py-8 sm:px-6 lg:px-8">
    <header
      class="rounded-3xl border border-slate-200/80 bg-white/85 p-6 shadow-[0_12px_40px_-20px_rgba(15,23,42,0.35)] backdrop-blur dark:border-slate-800/80 dark:bg-slate-900/75 sm:p-8"
    >
      <p class="text-xs font-semibold uppercase tracking-[0.24em] text-cyan-700 dark:text-cyan-300">Legal</p>
      <h1 class="mt-1 text-3xl font-extrabold tracking-tight text-slate-950 dark:text-white">Privacy Policy</h1>
      <p class="mt-2 text-sm text-slate-500 dark:text-slate-400">Last updated: August 2, 2026</p>
      <p class="mt-4 text-sm leading-6 text-slate-600 dark:text-slate-300">
        This policy explains what information INTENIQUETIC ("we", "us") collects through this website and our
        products (including AI Upscaler, Tools, and PDF Pro X), and how we use it.
      </p>
    </header>

    <section
      v-for="section in sections"
      :key="section.title"
      class="rounded-3xl border border-slate-200/80 bg-white/85 p-6 shadow-[0_12px_40px_-20px_rgba(15,23,42,0.35)] backdrop-blur dark:border-slate-800/80 dark:bg-slate-900/75"
    >
      <h2 class="text-lg font-bold text-slate-900 dark:text-slate-100">{{ section.title }}</h2>
      <ul class="mt-3 space-y-2">
        <li v-for="line in section.body" :key="line" class="text-sm leading-6 text-slate-600 dark:text-slate-300">
          {{ line }}
        </li>
      </ul>
    </section>

    <section
      class="rounded-3xl border border-cyan-200/70 bg-gradient-to-br from-cyan-50 via-white to-slate-50 p-6 shadow-[0_12px_40px_-20px_rgba(8,145,178,0.35)] dark:border-cyan-900/70 dark:from-slate-950 dark:via-slate-900 dark:to-cyan-950/40"
    >
      <h2 class="text-lg font-bold text-slate-900 dark:text-slate-100">9. Contact us</h2>
      <p class="mt-2 text-sm leading-6 text-slate-600 dark:text-slate-300">
        Questions about this policy or your data? Email
        <a href="mailto:dev.prongbang@gmail.com" class="font-semibold text-cyan-700 underline dark:text-cyan-300"
          >dev.prongbang@gmail.com</a
        >.
      </p>
    </section>
  </main>
</template>
