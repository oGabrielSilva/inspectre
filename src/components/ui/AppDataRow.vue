<script setup lang="ts">
import AppValue from '@/components/ui/AppValue.vue';

defineProps<{
  label: string;
  value: unknown;
  format?: (value: NonNullable<unknown>) => string;
  unit?: string;
}>();
</script>

<template>
  <div class="flex items-baseline justify-between gap-4 py-1 text-sm">
    <span class="text-dimmed">{{ label }}</span>
    <AppValue :value="value" :format="format" :unit="unit" />
  </div>
</template>
