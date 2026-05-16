<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';

import AppCard from '@/components/ui/AppCard.vue';
import AppDataRow from '@/components/ui/AppDataRow.vue';
import AppValue from '@/components/ui/AppValue.vue';
import { formatBytes } from '@/shared/format';
import type { MemoryLiveTick } from '@/shared/generated/MemoryLiveTick';

const props = defineProps<{
  live: MemoryLiveTick | null;
}>();

const { t } = useI18n();

const usagePercent = computed(() => {
  if (!props.live || props.live.totalBytes === 0) return 0;
  return (props.live.usedBytes / props.live.totalBytes) * 100;
});

const swapPercent = computed(() => {
  if (!props.live || props.live.swapTotalBytes === 0) return 0;
  return (props.live.swapUsedBytes / props.live.swapTotalBytes) * 100;
});

const usageColor = computed(() => {
  const p = usagePercent.value;
  if (p >= 90) return 'error';
  if (p >= 75) return 'warning';
  return 'primary';
});
</script>

<template>
  <AppCard :title="t('memory.sections.usage')">
    <div v-if="live" class="space-y-4">
      <div>
        <div class="mb-1 flex items-baseline justify-between">
          <span class="text-dimmed text-xs">{{ t('memory.fields.used') }}</span>
          <span class="font-mono text-2xl font-medium tabular-nums">
            {{ formatBytes(live.usedBytes) }} / {{ formatBytes(live.totalBytes) }}
          </span>
        </div>
        <UProgress :model-value="usagePercent" :max="100" size="md" :color="usageColor" />
        <div class="text-dimmed mt-1 flex justify-between text-xs">
          <span>{{ usagePercent.toFixed(1) }}%</span>
          <span>{{ t('memory.fields.available') }}: {{ formatBytes(live.availableBytes) }}</span>
        </div>
      </div>

      <div v-if="live.swapTotalBytes > 0">
        <div class="mb-1 flex items-baseline justify-between">
          <span class="text-dimmed text-xs">{{ t('memory.fields.swap') }}</span>
          <span class="font-mono text-sm tabular-nums">
            {{ formatBytes(live.swapUsedBytes) }} / {{ formatBytes(live.swapTotalBytes) }}
          </span>
        </div>
        <UProgress :model-value="swapPercent" :max="100" size="xs" color="neutral" />
      </div>

      <div v-else class="space-y-1">
        <AppDataRow :label="t('memory.fields.swapTotal')" :value="null" />
      </div>
    </div>

    <AppValue v-else :value="null" />
  </AppCard>
</template>
