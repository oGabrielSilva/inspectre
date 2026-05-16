import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { onMounted, onUnmounted, ref } from 'vue';

import type { CpuLiveTick } from '@/shared/generated/CpuLiveTick';
import { logger } from '@/utils/logger';

export function useCpuLiveStream(intervalMs = 1000) {
  const live = ref<CpuLiveTick | null>(null);
  let unlisten: UnlistenFn | undefined;

  onMounted(async () => {
    try {
      unlisten = await listen<CpuLiveTick>('cpu:live', (event) => {
        live.value = event.payload;
      });
      await invoke('start_cpu_live_stream', { intervalMs });
    } catch (err) {
      logger.warn('falha ao iniciar stream cpu:live', err);
    }
  });

  onUnmounted(async () => {
    unlisten?.();
    try {
      await invoke('stop_cpu_live_stream');
    } catch (err) {
      logger.warn('falha ao parar stream cpu:live', err);
    }
  });

  return { live };
}
