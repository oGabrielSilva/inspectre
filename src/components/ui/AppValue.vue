<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';

const props = defineProps<{
  value: unknown;
  format?: (value: NonNullable<unknown>) => string;
  unit?: string;
}>();

const { t } = useI18n();

const display = computed(() => {
  if (props.value === null || props.value === undefined) {
    return t('common.states.unavailable');
  }
  const formatted = props.format
    ? props.format(props.value as NonNullable<unknown>)
    : String(props.value);
  return props.unit ? `${formatted} ${props.unit}` : formatted;
});

const isUnavailable = computed(() => props.value === null || props.value === undefined);
</script>

<template>
  <span :class="isUnavailable ? 'text-muted' : 'text-default'">{{ display }}</span>
</template>
