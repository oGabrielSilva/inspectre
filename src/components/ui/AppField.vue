<script setup lang="ts">
defineProps<{
  label?: string;
  description?: string;
  help?: string;
  hint?: string;
  error?: string;
  required?: boolean;
}>();
</script>

<template>
  <UFormField
    :label="label"
    :description="description"
    :help="help"
    :hint="hint"
    :error="error"
    :required="required"
  >
    <slot />
  </UFormField>
</template>
