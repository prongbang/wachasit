<script setup lang="ts">
import { navigate } from '@/lib/router'

const props = defineProps<{ to: string }>()

function onClick(event: MouseEvent) {
  if (event.metaKey || event.ctrlKey || event.shiftKey || event.button !== 0) return
  event.preventDefault()
  navigate(props.to)
}
</script>

<template>
  <a :href="to" @click="onClick">
    <slot />
  </a>
</template>
