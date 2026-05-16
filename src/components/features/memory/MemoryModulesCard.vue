<script setup lang="ts">
import { useI18n } from 'vue-i18n';

import AppCard from '@/components/ui/AppCard.vue';
import AppValue from '@/components/ui/AppValue.vue';
import { formatBytes, formatMhz } from '@/shared/format';
import type { MemoryModule } from '@/shared/generated/MemoryModule';

defineProps<{
  modules: MemoryModule[];
}>();

const { t } = useI18n();

function voltageLabel(v: number | null): string | null {
  if (v === null || v === undefined) return null;
  return `${v.toFixed(2)} V`;
}

function widthLabel(data: number | null, total: number | null): string | null {
  if (data === null) return null;
  if (total === null || total === data) return `${data} bits`;
  return `${data}/${total} bits`;
}
</script>

<template>
  <AppCard :title="t('memory.sections.modules')">
    <div v-if="modules.length === 0">
      <AppValue :value="null" />
    </div>

    <div v-else class="grid gap-3 md:grid-cols-2">
      <div
        v-for="(module, index) in modules"
        :key="`${module.slot}-${index}`"
        class="border-default rounded-md border p-3"
      >
        <div class="mb-2 flex items-center justify-between">
          <span class="font-mono text-xs font-semibold">{{ module.slot }}</span>
          <UBadge v-if="module.memoryType" color="primary" variant="subtle" size="xs">
            {{ module.memoryType }}
          </UBadge>
        </div>

        <div class="space-y-1 text-xs">
          <div class="flex justify-between">
            <span class="text-dimmed">{{ t('memory.fields.module.size') }}</span>
            <span class="text-default font-mono tabular-nums">
              {{ module.sizeBytes ? formatBytes(module.sizeBytes) : '—' }}
            </span>
          </div>
          <div v-if="module.configuredSpeedMhz || module.speedMhz" class="flex justify-between">
            <span class="text-dimmed">{{ t('memory.fields.module.configuredSpeed') }}</span>
            <span class="text-default font-mono tabular-nums">
              {{ formatMhz((module.configuredSpeedMhz ?? module.speedMhz) as number) }}
            </span>
          </div>
          <div
            v-if="module.speedMhz && module.configuredSpeedMhz && module.speedMhz !== module.configuredSpeedMhz"
            class="flex justify-between"
          >
            <span class="text-dimmed">{{ t('memory.fields.module.speed') }}</span>
            <span class="text-default font-mono tabular-nums">
              {{ formatMhz(module.speedMhz) }}
            </span>
          </div>
          <div v-if="module.manufacturer" class="flex justify-between">
            <span class="text-dimmed">{{ t('memory.fields.module.manufacturer') }}</span>
            <span class="text-default">{{ module.manufacturer }}</span>
          </div>
          <div v-if="module.partNumber" class="flex justify-between">
            <span class="text-dimmed">{{ t('memory.fields.module.partNumber') }}</span>
            <span class="text-default font-mono">{{ module.partNumber }}</span>
          </div>
          <div v-if="module.formFactor" class="flex justify-between">
            <span class="text-dimmed">{{ t('memory.fields.module.formFactor') }}</span>
            <span class="text-default">{{ module.formFactor }}</span>
          </div>
          <div v-if="widthLabel(module.dataWidth, module.totalWidth)" class="flex justify-between">
            <span class="text-dimmed">{{ t('memory.fields.module.width') }}</span>
            <span class="text-default font-mono">
              {{ widthLabel(module.dataWidth, module.totalWidth) }}
            </span>
          </div>
          <div v-if="voltageLabel(module.voltageV)" class="flex justify-between">
            <span class="text-dimmed">{{ t('memory.fields.module.voltage') }}</span>
            <span class="text-default font-mono">{{ voltageLabel(module.voltageV) }}</span>
          </div>
        </div>
      </div>
    </div>
  </AppCard>
</template>
