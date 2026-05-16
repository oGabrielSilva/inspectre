import { invoke, type InvokeArgs } from '@tauri-apps/api/core';
import { readonly, ref } from 'vue';

import { useFeedback } from '@/composables/useFeedback';
import { logger } from '@/utils/logger';

export type InspectreErrorPayload = {
  code: string;
  details?: unknown;
};

function normalizeError(err: unknown): InspectreErrorPayload {
  if (err && typeof err === 'object' && 'code' in err) {
    const e = err as { code: unknown; details?: unknown };
    return {
      code: typeof e.code === 'string' ? e.code : 'internal',
      details: e.details,
    };
  }
  return {
    code: 'internal',
    details: err instanceof Error ? err.message : String(err),
  };
}

export function useHardwareProbe<T>(command: string) {
  const data = ref<T | null>(null);
  const loading = ref(false);
  const error = ref<InspectreErrorPayload | null>(null);
  const feedback = useFeedback();

  async function load(args?: InvokeArgs) {
    loading.value = true;
    error.value = null;
    try {
      data.value = await invoke<T>(command, args);
    } catch (e) {
      const normalized = normalizeError(e);
      error.value = normalized;
      logger.warn('probe falhou', command, normalized);
      feedback.error({ code: normalized.code });
    } finally {
      loading.value = false;
    }
  }

  return {
    data: readonly(data),
    loading: readonly(loading),
    error: readonly(error),
    load,
  };
}
