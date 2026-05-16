<script setup lang="ts">
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
</script>

<template>
  <div class="flex h-full items-center justify-center p-8">
    <UAlert
      color="neutral"
      variant="subtle"
      icon="i-lucide-construction"
      :title="t('common.states.placeholder')"
      class="max-w-md"
    />
  </div>
</template>
