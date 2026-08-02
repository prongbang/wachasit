<script setup lang="ts">
import Shell from '@/components/ui/Shell.vue'
import HomePage from '@/pages/HomePage.vue'
import PortPage from '@/pages/PortPage.vue'
import PrivacyPage from '@/pages/PrivacyPage.vue'
import { useRoute } from '@/lib/router'

const path = useRoute()
</script>

<template>
  <Shell>
    <PrivacyPage v-if="path === '/privacy'" />
    <PortPage v-else-if="path === '/port'" />
    <HomePage v-else />
  </Shell>
</template>
