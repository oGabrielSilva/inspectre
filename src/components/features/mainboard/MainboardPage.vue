<script setup lang="ts">
import { onMounted } from 'vue';
import { useI18n } from 'vue-i18n';

import AppButton from '@/components/ui/AppButton.vue';
import AppCard from '@/components/ui/AppCard.vue';
import AppDataRow from '@/components/ui/AppDataRow.vue';
import { useHardwareProbe } from '@/composables/useHardwareProbe';
import type { FirmwareType } from '@/shared/generated/FirmwareType';
import type { MainboardInfo } from '@/shared/generated/MainboardInfo';

const { t } = useI18n();
const { data, loading, error, load } = useHardwareProbe<MainboardInfo>('mainboard_info');

onMounted(() => load());

function firmwareLabel(type: FirmwareType): string {
  return t(`mainboard.states.${type}`);
}

function boolLabel(value: boolean | null | undefined): string | null {
  if (value === null || value === undefined) return null;
  return value ? t('mainboard.states.enabled') : t('mainboard.states.disabled');
}

function presenceLabel(value: boolean): string {
  return value ? t('mainboard.states.present') : t('mainboard.states.absent');
}
</script>

<template>
  <div class="space-y-4 p-4">
    <header class="flex items-center justify-between gap-2">
      <h1 class="text-base font-semibold">{{ t('mainboard.title') }}</h1>
      <AppButton
        :label="t('mainboard.actions.refresh')"
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
      <USkeleton v-for="i in 4" :key="i" class="h-48 w-full" />
    </div>

    <div v-else-if="data" class="grid gap-4 lg:grid-cols-2">
      <AppCard :title="t('mainboard.sections.board')">
        <div class="space-y-1">
          <AppDataRow :label="t('mainboard.fields.manufacturer')" :value="data.manufacturer" />
          <AppDataRow :label="t('mainboard.fields.product')" :value="data.product" />
          <AppDataRow :label="t('mainboard.fields.version')" :value="data.version" />
          <AppDataRow :label="t('mainboard.fields.serial')" :value="data.serial" />
        </div>
      </AppCard>

      <AppCard :title="t('mainboard.sections.system')">
        <div class="space-y-1">
          <AppDataRow
            :label="t('mainboard.fields.chassisManufacturer')"
            :value="data.chassisManufacturer"
          />
          <AppDataRow :label="t('mainboard.fields.chassisModel')" :value="data.chassisModel" />
        </div>
      </AppCard>

      <AppCard :title="t('mainboard.sections.bios')">
        <div class="space-y-1">
          <AppDataRow :label="t('mainboard.fields.bios.vendor')" :value="data.bios.vendor" />
          <AppDataRow :label="t('mainboard.fields.bios.version')" :value="data.bios.version" />
          <AppDataRow
            :label="t('mainboard.fields.bios.smbiosVersion')"
            :value="data.bios.smbiosVersion"
          />
          <AppDataRow
            :label="t('mainboard.fields.bios.releaseDate')"
            :value="data.bios.releaseDate"
          />
          <AppDataRow
            :label="t('mainboard.fields.bios.firmwareType')"
            :value="firmwareLabel(data.bios.firmwareType)"
          />
        </div>
      </AppCard>

      <AppCard :title="t('mainboard.sections.security')">
        <div class="space-y-1">
          <AppDataRow
            :label="t('mainboard.fields.security.secureBoot')"
            :value="boolLabel(data.security.secureBoot)"
          />
          <AppDataRow
            :label="t('mainboard.fields.security.tpmPresent')"
            :value="presenceLabel(data.security.tpmPresent)"
          />
          <AppDataRow
            :label="t('mainboard.fields.security.tpmEnabled')"
            :value="boolLabel(data.security.tpmEnabled)"
          />
          <AppDataRow
            :label="t('mainboard.fields.security.tpmSpecVersion')"
            :value="data.security.tpmSpecVersion"
          />
        </div>
      </AppCard>
    </div>
  </div>
</template>
