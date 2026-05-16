<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';

import AppCard from '@/components/ui/AppCard.vue';
import AppDataRow from '@/components/ui/AppDataRow.vue';
import { formatBytes } from '@/shared/format';
import type { MemoryInfo } from '@/shared/generated/MemoryInfo';

const props = defineProps<{
  data: MemoryInfo;
}>();

const { t } = useI18n();

const channelLabel = computed(() => t(`memory.states.${props.data.channels.configuration}`));

const predominantType = computed(() => {
  const counts = new Map<string, number>();
  for (const m of props.data.modules) {
    if (m.memoryType) counts.set(m.memoryType, (counts.get(m.memoryType) ?? 0) + 1);
  }
  let best: string | null = null;
  let bestCount = 0;
  for (const [type, count] of counts) {
    if (count > bestCount) {
      best = type;
      bestCount = count;
    }
  }
  return best;
});
</script>

<template>
  <AppCard :title="t('memory.sections.overview')">
    <div class="space-y-1">
      <AppDataRow
        :label="t('memory.fields.total')"
        :value="data.totalBytes"
        :format="(v) => formatBytes(v as number)"
      />
      <AppDataRow
        :label="t('memory.fields.maxSupported')"
        :value="data.maxSupportedBytes"
        :format="(v) => formatBytes(v as number)"
      />
      <AppDataRow :label="t('memory.fields.type')" :value="predominantType" />
      <AppDataRow :label="t('memory.fields.channels')" :value="channelLabel" />
      <AppDataRow
        :label="t('memory.fields.slots')"
        :value="`${data.slotsPopulated} / ${data.slotsTotal}`"
      />
      <AppDataRow
        :label="t('memory.fields.ecc')"
        :value="data.eccSupported ? t('memory.states.supported') : t('memory.states.unsupported')"
      />
    </div>
  </AppCard>
</template>
