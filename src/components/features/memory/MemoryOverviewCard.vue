<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';

import AppCard from '@/components/ui/AppCard.vue';
import AppValue from '@/components/ui/AppValue.vue';
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

const eccLabel = computed(() =>
  props.data.eccSupported ? t('memory.states.supported') : t('memory.states.unsupported'),
);

const slotsLabel = computed(() => `${props.data.slotsPopulated} / ${props.data.slotsTotal}`);
</script>

<template>
  <AppCard :title="t('memory.sections.overview')">
    <dl class="grid grid-cols-2 gap-x-6 gap-y-2 text-xs sm:grid-cols-3">
      <div>
        <dt class="text-dimmed">{{ t('memory.fields.total') }}</dt>
        <dd class="text-default font-mono tabular-nums">{{ formatBytes(data.totalBytes) }}</dd>
      </div>
      <div>
        <dt class="text-dimmed">{{ t('memory.fields.type') }}</dt>
        <dd class="text-default">
          <AppValue :value="predominantType" />
        </dd>
      </div>
      <div>
        <dt class="text-dimmed">{{ t('memory.fields.channels') }}</dt>
        <dd class="text-default">{{ channelLabel }}</dd>
      </div>
      <div>
        <dt class="text-dimmed">{{ t('memory.fields.slots') }}</dt>
        <dd class="text-default font-mono tabular-nums">{{ slotsLabel }}</dd>
      </div>
      <div>
        <dt class="text-dimmed">{{ t('memory.fields.maxSupported') }}</dt>
        <dd class="text-default font-mono tabular-nums">
          <AppValue
            :value="data.maxSupportedBytes"
            :format="(v) => formatBytes(v as number)"
          />
        </dd>
      </div>
      <div>
        <dt class="text-dimmed">{{ t('memory.fields.ecc') }}</dt>
        <dd class="text-default">{{ eccLabel }}</dd>
      </div>
    </dl>
  </AppCard>
</template>
