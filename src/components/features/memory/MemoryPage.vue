<script setup lang="ts">
import { computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';

import MemoryModulesCard from '@/components/features/memory/MemoryModulesCard.vue';
import MemoryOverviewCard from '@/components/features/memory/MemoryOverviewCard.vue';
import MemoryUsageCard from '@/components/features/memory/MemoryUsageCard.vue';
import AppButton from '@/components/ui/AppButton.vue';
import { useHardwareProbe } from '@/composables/useHardwareProbe';
import { useMemoryLiveStream } from '@/composables/useMemoryLiveStream';
import type { MemoryInfo } from '@/shared/generated/MemoryInfo';

const { t } = useI18n();
const { data, loading, error, load } = useHardwareProbe<MemoryInfo>('memory_info');
const { live } = useMemoryLiveStream(1000);

onMounted(() => load());

const liveValue = computed(() => live.value);
</script>

<template>
  <div class="space-y-4 p-4">
    <header class="flex items-center justify-between gap-2">
      <h1 class="text-base font-semibold">{{ t('memory.title') }}</h1>
      <AppButton
        :label="t('memory.actions.refresh')"
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
      <USkeleton v-for="i in 3" :key="i" class="h-48 w-full" :class="i === 1 ? 'lg:col-span-2' : ''" />
    </div>

    <div v-else-if="data" class="grid gap-4 lg:grid-cols-2">
      <MemoryUsageCard :live="liveValue" class="lg:col-span-2" />

      <MemoryOverviewCard :data="data" />

      <MemoryModulesCard :modules="data.modules" />
    </div>
  </div>
</template>
