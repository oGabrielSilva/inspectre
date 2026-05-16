<script setup lang="ts">
import { computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';

import CpuFrequencyCard from '@/components/features/cpu/CpuFrequencyCard.vue';
import CpuSensorsCard from '@/components/features/cpu/CpuSensorsCard.vue';
import CpuUsageCard from '@/components/features/cpu/CpuUsageCard.vue';
import AppButton from '@/components/ui/AppButton.vue';
import AppCard from '@/components/ui/AppCard.vue';
import AppDataRow from '@/components/ui/AppDataRow.vue';
import AppValue from '@/components/ui/AppValue.vue';
import { useCpuLiveStream } from '@/composables/useCpuLiveStream';
import { useHardwareProbe } from '@/composables/useHardwareProbe';
import { formatCacheSize, formatHex } from '@/shared/format';
import type { CacheLevel } from '@/shared/generated/CacheLevel';
import type { CpuInfo } from '@/shared/generated/CpuInfo';

const { t } = useI18n();
const { data, loading, error, load } = useHardwareProbe<CpuInfo>('cpu_info');
const { live } = useCpuLiveStream(1000);

onMounted(() => load());

const liveValue = computed(() => live.value);

function describeCache(level: CacheLevel | null | undefined): string | null {
  if (!level) return null;
  return `${formatCacheSize(level.sizeKb)} • ${level.ways} ${t('cpu.fields.cache.ways')} • ${level.lineSize} B ${t('cpu.fields.cache.lineSize')}`;
}
</script>

<template>
  <div class="space-y-4 p-4">
    <header class="flex items-center justify-between gap-2">
      <h1 class="text-base font-semibold">{{ t('cpu.title') }}</h1>
      <AppButton
        :label="t('cpu.actions.refresh')"
        icon="i-lucide-refresh-cw"
        variant="ghost"
        size="sm"
        :loading="loading"
        @click="load()"
      />
    </header>

    <UAlert
      v-if="error"
      color="error"
      icon="i-lucide-circle-alert"
      :title="t(`errors.${error.code}.title`)"
      :description="t(`errors.${error.code}.description`)"
    />

    <div v-if="loading && !data" class="grid gap-4 lg:grid-cols-2">
      <USkeleton v-for="i in 6" :key="i" class="h-48 w-full" />
    </div>

    <div v-else-if="data" class="grid gap-4 lg:grid-cols-2">
      <CpuUsageCard :live="liveValue" class="lg:col-span-2" />

      <CpuFrequencyCard
        :base-mhz="data.baseClockMhz"
        :max-mhz="data.maxClockMhz"
        :bus-mhz="data.busClockMhz"
        :snapshot-mhz="data.currentClockMhz"
        :live="liveValue"
      />

      <CpuSensorsCard :live="liveValue" :boot-time-sec="data.bootTimeSec" />

      <AppCard :title="t('cpu.sections.processor')">
        <div class="space-y-1">
          <AppDataRow :label="t('cpu.fields.name')" :value="data.name" />
          <AppDataRow :label="t('cpu.fields.vendor')" :value="data.vendor" />
          <AppDataRow :label="t('cpu.fields.codename')" :value="data.codename" />
          <AppDataRow :label="t('cpu.fields.socket')" :value="data.socket" />
          <AppDataRow :label="t('cpu.fields.process')" :value="data.processNm" unit="nm" />
          <AppDataRow :label="t('cpu.fields.tdp')" :value="data.tdpW" unit="W" />
          <AppDataRow :label="t('cpu.fields.revision')" :value="data.revision" />
          <AppDataRow
            :label="t('cpu.fields.family')"
            :value="data.family"
            :format="(v) => formatHex(v as number)"
          />
          <AppDataRow
            :label="t('cpu.fields.model')"
            :value="data.model"
            :format="(v) => formatHex(v as number)"
          />
          <AppDataRow
            :label="t('cpu.fields.stepping')"
            :value="data.stepping"
            :format="(v) => formatHex(v as number)"
          />
          <AppDataRow
            :label="t('cpu.fields.extFamily')"
            :value="data.extFamily"
            :format="(v) => formatHex(v as number)"
          />
          <AppDataRow
            :label="t('cpu.fields.extModel')"
            :value="data.extModel"
            :format="(v) => formatHex(v as number)"
          />
        </div>
      </AppCard>

      <AppCard :title="t('cpu.sections.cores')">
        <div class="space-y-1">
          <AppDataRow :label="t('cpu.fields.cores')" :value="data.cores" />
          <AppDataRow :label="t('cpu.fields.threads')" :value="data.threads" />
        </div>

        <div class="mt-3 flex flex-wrap items-center gap-2">
          <UBadge
            :color="data.hyperthreading ? 'success' : 'neutral'"
            variant="subtle"
            size="xs"
          >
            {{ t('cpu.fields.hyperthreading') }}:
            {{ data.hyperthreading ? t('cpu.states.active') : t('cpu.states.inactive') }}
          </UBadge>
          <UBadge
            :color="data.virtualizationSupported ? 'success' : 'neutral'"
            variant="subtle"
            size="xs"
          >
            {{ t('cpu.fields.virtualization') }}:
            {{
              data.virtualizationSupported
                ? t('cpu.states.supported')
                : t('cpu.states.unsupported')
            }}
          </UBadge>
        </div>

        <div class="mt-3">
          <span class="text-dimmed mb-1 block text-xs">{{ t('cpu.fields.instructions') }}</span>
          <div v-if="data.instructionSets.length" class="flex flex-wrap gap-1">
            <UBadge
              v-for="set in data.instructionSets"
              :key="set"
              color="neutral"
              variant="subtle"
              size="xs"
            >
              {{ set }}
            </UBadge>
          </div>
          <AppValue v-else :value="null" />
        </div>
      </AppCard>

      <AppCard :title="t('cpu.sections.cache')" class="lg:col-span-2">
        <div class="space-y-1">
          <AppDataRow :label="t('cpu.fields.cache.l1Data')" :value="describeCache(data.cache.l1Data)" />
          <AppDataRow :label="t('cpu.fields.cache.l1Inst')" :value="describeCache(data.cache.l1Inst)" />
          <AppDataRow :label="t('cpu.fields.cache.l2')" :value="describeCache(data.cache.l2)" />
          <AppDataRow :label="t('cpu.fields.cache.l3')" :value="describeCache(data.cache.l3)" />
        </div>
      </AppCard>
    </div>
  </div>
</template>
