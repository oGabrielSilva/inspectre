<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';

import AppButton from '@/components/ui/AppButton.vue';
import AppCard from '@/components/ui/AppCard.vue';
import { useBenchRunner } from '@/composables/useBenchRunner';
import { formatThroughput } from '@/shared/format';
import type { BenchResult } from '@/shared/generated/BenchResult';

const { t, te } = useI18n();
const { running, report, progress, start } = useBenchRunner();

const phaseLabel = computed(() => {
  if (!progress.value) return '';
  const key = `bench.phases.${progress.value.phase}`;
  return te(key) ? t(key) : progress.value.phase;
});

const progressPercent = computed(() => {
  if (!progress.value) return 0;
  return (progress.value.phaseIndex / progress.value.totalPhases) * 100;
});

const workloads = computed<Array<{ key: string; result: BenchResult } | null>>(() => {
  if (!report.value) return [];
  return [
    { key: 'sha256', result: report.value.sha256 },
    { key: 'aes256', result: report.value.aes256 },
    { key: 'prime', result: report.value.prime },
  ];
});

function totalDurationLabel(): string {
  if (!report.value) return '';
  const seconds = report.value.totalDurationMs / 1000;
  return `${seconds.toFixed(1)} s`;
}
</script>

<template>
  <div class="space-y-4 p-4">
    <header class="flex items-center justify-between gap-2">
      <div class="min-w-0">
        <h1 class="text-base font-semibold">{{ t('bench.title') }}</h1>
        <p class="text-dimmed text-xs">{{ t('bench.description') }}</p>
      </div>
      <AppButton
        :label="running ? t('bench.actions.running') : t('bench.actions.run')"
        icon="i-lucide-play"
        color="primary"
        size="sm"
        :loading="running"
        :disabled="running"
        @click="start(3)"
      />
    </header>

    <AppCard v-if="running" :title="t('bench.sections.running')">
      <div class="space-y-2">
        <div class="flex items-baseline justify-between text-xs">
          <span class="text-default">{{ phaseLabel }}</span>
          <span class="text-dimmed font-mono tabular-nums">
            {{ progress?.phaseIndex ?? 0 }} / {{ progress?.totalPhases ?? 6 }}
          </span>
        </div>
        <UProgress :model-value="progressPercent" :max="100" size="md" color="primary" />
      </div>
    </AppCard>

    <AppCard v-else-if="!report" :title="t('bench.sections.results')">
      <p class="text-dimmed text-sm">{{ t('bench.sections.empty') }}</p>
    </AppCard>

    <AppCard v-else :title="t('bench.sections.results')">
      <div class="overflow-x-auto">
        <table class="w-full text-xs">
          <thead>
            <tr class="text-dimmed border-default border-b text-left">
              <th class="py-2 pr-3 font-medium">{{ t('bench.title') }}</th>
              <th class="py-2 pr-3 text-right font-medium">
                {{ t('bench.fields.singleThread') }}
              </th>
              <th class="py-2 pr-3 text-right font-medium">
                {{ t('bench.fields.multiThread') }}
              </th>
              <th class="py-2 text-right font-medium">{{ t('bench.fields.scaling') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="entry in workloads"
              :key="entry?.key"
              class="border-default border-b last:border-0"
            >
              <td class="py-2 pr-3">
                <div class="font-medium">{{ t(`bench.workloads.${entry?.key}`) }}</div>
                <div class="text-dimmed font-mono text-[10px]">
                  {{ entry?.result.multiThread.threadCount }} threads
                </div>
              </td>
              <td class="py-2 pr-3 text-right font-mono tabular-nums">
                {{
                  entry
                    ? formatThroughput(entry.result.singleThread.throughput, entry.result.unit)
                    : ''
                }}
              </td>
              <td class="py-2 pr-3 text-right font-mono tabular-nums">
                {{
                  entry
                    ? formatThroughput(entry.result.multiThread.throughput, entry.result.unit)
                    : ''
                }}
              </td>
              <td class="py-2 text-right font-mono tabular-nums">
                {{ entry ? `${entry.result.scaling.toFixed(2)}x` : '' }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="text-dimmed mt-3 text-xs">
        {{ t('bench.fields.totalDuration') }}: {{ totalDurationLabel() }}
      </div>
    </AppCard>
  </div>
</template>
