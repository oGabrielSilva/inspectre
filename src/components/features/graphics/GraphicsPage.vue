<script setup lang="ts">
import { onMounted } from 'vue';
import { useI18n } from 'vue-i18n';

import GraphicsAdapterCard from '@/components/features/graphics/GraphicsAdapterCard.vue';
import AppButton from '@/components/ui/AppButton.vue';
import AppCard from '@/components/ui/AppCard.vue';
import AppValue from '@/components/ui/AppValue.vue';
import { useHardwareProbe } from '@/composables/useHardwareProbe';
import type { GraphicsInfo } from '@/shared/generated/GraphicsInfo';

const { t } = useI18n();
const { data, loading, error, load } = useHardwareProbe<GraphicsInfo>('graphics_info');

onMounted(() => load());
</script>

<template>
  <div class="space-y-4 p-4">
    <header class="flex items-center justify-between gap-2">
      <h1 class="text-base font-semibold">{{ t('graphics.title') }}</h1>
      <AppButton
        :label="t('graphics.actions.refresh')"
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

    <div v-if="loading && !data" class="space-y-4">
      <USkeleton v-for="i in 2" :key="i" class="h-48 w-full" />
    </div>

    <div v-else-if="data" class="space-y-4">
      <AppCard v-if="data.adapters.length === 0">
        <AppValue :value="null" />
        <p class="text-dimmed mt-2 text-sm">{{ t('graphics.sections.noAdapters') }}</p>
      </AppCard>

      <GraphicsAdapterCard
        v-for="(adapter, index) in data.adapters"
        :key="`${adapter.vendorId}-${adapter.deviceId}-${index}`"
        :adapter="adapter"
      />
    </div>
  </div>
</template>
