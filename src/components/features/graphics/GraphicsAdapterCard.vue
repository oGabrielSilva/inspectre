<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';

import AppCard from '@/components/ui/AppCard.vue';
import AppValue from '@/components/ui/AppValue.vue';
import { formatBytes, formatHex } from '@/shared/format';
import type { GraphicsAdapter } from '@/shared/generated/GraphicsAdapter';

const props = defineProps<{
  adapter: GraphicsAdapter;
}>();

const { t } = useI18n();

const vendorLabel = computed(() => t(`graphics.vendors.${props.adapter.vendor}`));
const kindLabel = computed(() => t(`graphics.kinds.${props.adapter.kind}`));

const kindColor = computed(() => {
  switch (props.adapter.kind) {
    case 'dedicated':
      return 'primary';
    case 'integrated':
      return 'info';
    case 'software':
      return 'warning';
    default:
      return 'neutral';
  }
});

const resolutionLabel = computed(() => {
  const r = props.adapter.currentResolution;
  return r ? `${r.width} × ${r.height}` : null;
});

const refreshLabel = computed(() =>
  props.adapter.refreshRateHz ? `${props.adapter.refreshRateHz} Hz` : null,
);
</script>

<template>
  <AppCard>
    <template #header>
      <div class="flex flex-wrap items-center justify-between gap-2">
        <div class="min-w-0 flex-1">
          <h2 class="truncate text-sm font-semibold">{{ adapter.name }}</h2>
          <div class="text-dimmed text-xs">{{ vendorLabel }}</div>
        </div>
        <UBadge :color="kindColor" variant="subtle" size="xs">{{ kindLabel }}</UBadge>
      </div>
    </template>

    <dl class="grid grid-cols-2 gap-x-6 gap-y-2 text-xs sm:grid-cols-3">
      <div>
        <dt class="text-dimmed">{{ t('graphics.fields.dedicatedMemory') }}</dt>
        <dd class="text-default font-mono tabular-nums">
          {{ formatBytes(adapter.dedicatedMemoryBytes) }}
        </dd>
      </div>
      <div>
        <dt class="text-dimmed">{{ t('graphics.fields.sharedMemory') }}</dt>
        <dd class="text-default font-mono tabular-nums">
          {{ formatBytes(adapter.sharedMemoryBytes) }}
        </dd>
      </div>
      <div>
        <dt class="text-dimmed">{{ t('graphics.fields.resolution') }}</dt>
        <dd class="text-default font-mono tabular-nums">
          <AppValue :value="resolutionLabel" />
        </dd>
      </div>
      <div>
        <dt class="text-dimmed">{{ t('graphics.fields.refreshRate') }}</dt>
        <dd class="text-default font-mono tabular-nums">
          <AppValue :value="refreshLabel" />
        </dd>
      </div>
      <div>
        <dt class="text-dimmed">{{ t('graphics.fields.driverVersion') }}</dt>
        <dd class="text-default font-mono">
          <AppValue :value="adapter.driverVersion" />
        </dd>
      </div>
      <div>
        <dt class="text-dimmed">{{ t('graphics.fields.driverDate') }}</dt>
        <dd class="text-default font-mono tabular-nums">
          <AppValue :value="adapter.driverDate" />
        </dd>
      </div>
      <div>
        <dt class="text-dimmed">{{ t('graphics.fields.videoProcessor') }}</dt>
        <dd class="text-default">
          <AppValue :value="adapter.videoProcessor" />
        </dd>
      </div>
      <div>
        <dt class="text-dimmed">{{ t('graphics.fields.vendorId') }}</dt>
        <dd class="text-default font-mono tabular-nums">{{ formatHex(adapter.vendorId) }}</dd>
      </div>
      <div>
        <dt class="text-dimmed">{{ t('graphics.fields.deviceId') }}</dt>
        <dd class="text-default font-mono tabular-nums">{{ formatHex(adapter.deviceId) }}</dd>
      </div>
    </dl>
  </AppCard>
</template>
