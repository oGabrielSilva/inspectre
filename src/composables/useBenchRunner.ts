import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { ref } from 'vue';

import type { BenchProgress } from '@/shared/generated/BenchProgress';
import type { BenchReport } from '@/shared/generated/BenchReport';
import { logger } from '@/utils/logger';

export function useBenchRunner() {
  const running = ref(false);
  const report = ref<BenchReport | null>(null);
  const progress = ref<BenchProgress | null>(null);

  async function start(durationSecs = 3) {
    if (running.value) return;
    running.value = true;
    progress.value = null;
    report.value = null;

    let unlisten: UnlistenFn | undefined;
    try {
      unlisten = await listen<BenchProgress>('bench:progress', (event) => {
        progress.value = event.payload;
      });
      report.value = await invoke<BenchReport>('run_bench', { durationSecs });
    } catch (err) {
      logger.warn('bench falhou', err);
    } finally {
      unlisten?.();
      running.value = false;
      progress.value = null;
    }
  }

  return { running, report, progress, start };
}
