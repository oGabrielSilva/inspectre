<script setup lang="ts">
import { useI18n } from 'vue-i18n';

import AppCard from '@/components/ui/AppCard.vue';
import AppValue from '@/components/ui/AppValue.vue';
import { formatMhz } from '@/shared/format';
import type { CpuLiveTick } from '@/shared/generated/CpuLiveTick';

defineProps<{
  live: CpuLiveTick | null;
}>();

const { t } = useI18n();
</script>

<template>
  <AppCard :title="t('cpu.sections.usage')">
    <div v-if="live" class="space-y-3">
      <div class="flex items-baseline justify-between">
        <span class="text-dimmed text-xs">{{ t('cpu.fields.usage.average') }}</span>
        <span class="font-mono text-2xl font-medium tabular-nums">
          {{ live.usageAvg.toFixed(1) }}%
        </span>
      </div>

      <div class="grid grid-cols-1 gap-x-4 gap-y-1 sm:grid-cols-2">
        <div
          v-for="(usage, index) in live.usagePerCore"
          :key="index"
          class="grid grid-cols-[3rem_1fr_4rem] items-center gap-2 text-xs"
        >
          <span class="text-dimmed">{{ t('cpu.fields.usage.core', { n: index }) }}</span>
          <UProgress :model-value="usage" :max="100" size="xs" color="primary" />
          <span class="text-default text-right font-mono tabular-nums">
            {{ usage.toFixed(0) }}%
          </span>
        </div>
      </div>

      <div class="border-default mt-2 border-t pt-2">
        <span class="text-dimmed text-xs">{{ t('cpu.fields.clock.current') }}</span>
        <div class="grid grid-cols-2 gap-x-4 gap-y-1 text-xs sm:grid-cols-4">
          <div
            v-for="(mhz, index) in live.mhzPerCore"
            :key="index"
            class="flex items-baseline justify-between"
          >
            <span class="text-dimmed">{{ t('cpu.fields.usage.core', { n: index }) }}</span>
            <span class="text-default font-mono tabular-nums">{{ formatMhz(mhz) }}</span>
          </div>
        </div>
      </div>
    </div>

    <AppValue v-else :value="null" />
  </AppCard>
</template>
