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

function effectiveSpeed(m: MemoryModule): string {
  const value = m.configuredSpeedMhz ?? m.speedMhz;
  return value ? formatMhz(value) : t('common.states.unavailable');
}
</script>

<template>
  <AppCard :title="t('memory.sections.modules')">
    <div v-if="modules.length === 0">
      <AppValue :value="null" />
    </div>

    <div v-else class="overflow-x-auto">
      <table class="w-full text-xs">
        <thead>
          <tr class="text-dimmed border-default border-b text-left">
            <th class="py-2 pr-3 font-medium">{{ t('memory.fields.module.slot') }}</th>
            <th class="py-2 pr-3 font-medium">{{ t('memory.fields.module.type') }}</th>
            <th class="py-2 pr-3 font-medium">{{ t('memory.fields.module.size') }}</th>
            <th class="py-2 pr-3 font-medium">
              {{ t('memory.fields.module.configuredSpeed') }}
            </th>
            <th class="py-2 pr-3 font-medium">
              {{ t('memory.fields.module.manufacturer') }}
            </th>
            <th class="py-2 font-medium">{{ t('memory.fields.module.partNumber') }}</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="(module, index) in modules"
            :key="`${module.slot}-${index}`"
            class="border-default border-b last:border-0"
          >
            <td class="py-2 pr-3 font-mono">{{ module.slot }}</td>
            <td class="py-2 pr-3">
              <AppValue :value="module.memoryType" />
            </td>
            <td class="py-2 pr-3 font-mono tabular-nums">
              {{ module.sizeBytes ? formatBytes(module.sizeBytes) : '—' }}
            </td>
            <td class="py-2 pr-3 font-mono tabular-nums">{{ effectiveSpeed(module) }}</td>
            <td class="py-2 pr-3">
              <AppValue :value="module.manufacturer" />
            </td>
            <td class="py-2 font-mono">
              <AppValue :value="module.partNumber" />
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </AppCard>
</template>
