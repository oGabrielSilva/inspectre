<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';

import AppCard from '@/components/ui/AppCard.vue';
import AppDataRow from '@/components/ui/AppDataRow.vue';
import AppValue from '@/components/ui/AppValue.vue';
import { formatMhz } from '@/shared/format';
import type { CpuLiveTick } from '@/shared/generated/CpuLiveTick';

const props = defineProps<{
  baseMhz: number | null;
  maxMhz: number | null;
  busMhz: number | null;
  snapshotMhz: number | null;
  live: CpuLiveTick | null;
}>();

const { t } = useI18n();

const currentMhz = computed(() => props.live?.mhzAvg ?? props.snapshotMhz);

const currentMultiplier = computed(() => {
  if (!currentMhz.value || !props.busMhz || props.busMhz === 0) return null;
  return currentMhz.value / props.busMhz;
});
</script>

<template>
  <AppCard :title="t('cpu.sections.clock')">
    <div class="flex flex-col items-center justify-center gap-1 py-2">
      <span class="font-mono text-3xl font-medium tabular-nums">
        <AppValue :value="currentMhz" :format="formatMhz" />
      </span>
      <span class="text-dimmed text-xs">{{ t('cpu.fields.clock.current') }}</span>
    </div>

    <div class="mt-2 space-y-1">
      <AppDataRow :label="t('cpu.fields.clock.base')" :value="baseMhz" :format="formatMhz" />
      <AppDataRow :label="t('cpu.fields.clock.max')" :value="maxMhz" :format="formatMhz" />
      <AppDataRow :label="t('cpu.fields.clock.bus')" :value="busMhz" :format="formatMhz" />
      <AppDataRow
        :label="t('cpu.fields.clock.multiplier')"
        :value="currentMultiplier"
        :format="(v) => `${(v as number).toFixed(1)}x`"
      />
    </div>
  </AppCard>
</template>
