<script setup lang="ts">
defineProps<{
  title?: string;
}>();
</script>

<template>
  <UCard>
    <template v-if="$slots.header || title" #header>
      <slot name="header">
        <h2 class="text-sm font-semibold">{{ title }}</h2>
      </slot>
    </template>

    <slot />

    <template v-if="$slots.footer" #footer>
      <slot name="footer" />
    </template>
  </UCard>
</template>
