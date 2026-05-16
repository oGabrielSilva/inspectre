<script setup lang="ts">
defineProps<{
  label?: string;
  icon?: string;
  iconRight?: string;
  variant?: 'solid' | 'outline' | 'soft' | 'subtle' | 'ghost' | 'link';
  color?: 'primary' | 'secondary' | 'success' | 'info' | 'warning' | 'error' | 'neutral';
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl';
  loading?: boolean;
  disabled?: boolean;
}>();
</script>

<template>
  <UButton
    :label="label"
    :icon="icon"
    :trailing-icon="iconRight"
    :variant="variant"
    :color="color"
    :size="size"
    :loading="loading"
    :disabled="disabled"
  >
    <slot />
  </UButton>
</template>
