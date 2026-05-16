<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';

import AppCard from '@/components/ui/AppCard.vue';
import AppDataRow from '@/components/ui/AppDataRow.vue';
import AppValue from '@/components/ui/AppValue.vue';
import { formatUptime } from '@/shared/format';
import type { CpuLiveTick } from '@/shared/generated/CpuLiveTick';

const props = defineProps<{
  live: CpuLiveTick | null;
  bootTimeSec: number;
}>();

const { t } = useI18n();

const now = ref(Math.floor(Date.now() / 1000));
let timerId: ReturnType<typeof setInterval> | undefined;

onMounted(() => {
  timerId = setInterval(() => {
    now.value = Math.floor(Date.now() / 1000);
  }, 1000);
});

onUnmounted(() => {
  if (timerId) clearInterval(timerId);
});

const uptimeSec = computed(() => Math.max(0, now.value - props.bootTimeSec));

const temperatureC = computed(() => props.live?.temperatureC ?? null);

function formatTemperature(value: NonNullable<unknown>) {
  return `${(value as number).toFixed(1)} °C`;
}
</script>

<template>
  <AppCard :title="t('cpu.sections.sensors')">
    <div class="space-y-3">
      <div class="flex flex-col items-center gap-1 py-2">
        <span class="font-mono text-3xl font-medium tabular-nums">
          <AppValue :value="temperatureC" :format="formatTemperature" />
        </span>
        <span class="text-dimmed text-xs">{{ t('cpu.fields.sensors.temperature') }}</span>
      </div>

      <AppDataRow
        :label="t('cpu.fields.sensors.uptime')"
        :value="uptimeSec"
        :format="(v) => formatUptime(v as number)"
      />
    </div>
  </AppCard>
</template>
