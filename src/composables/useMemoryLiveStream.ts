import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { onMounted, onUnmounted, ref } from 'vue';

import type { MemoryLiveTick } from '@/shared/generated/MemoryLiveTick';
import { logger } from '@/utils/logger';

export function useMemoryLiveStream(intervalMs = 1000) {
  const live = ref<MemoryLiveTick | null>(null);
  let unlisten: UnlistenFn | undefined;

  onMounted(async () => {
    try {
      unlisten = await listen<MemoryLiveTick>('memory:live', (event) => {
        live.value = event.payload;
      });
      await invoke('start_memory_live_stream', { intervalMs });
    } catch (err) {
      logger.warn('falha ao iniciar stream memory:live', err);
    }
  });

  onUnmounted(async () => {
    unlisten?.();
    try {
      await invoke('stop_memory_live_stream');
    } catch (err) {
      logger.warn('falha ao parar stream memory:live', err);
    }
  });

  return { live };
}
