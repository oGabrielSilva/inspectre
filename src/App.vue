<script setup lang="ts">
import type { NavigationMenuItem } from '@nuxt/ui';
import { invoke } from '@tauri-apps/api/core';
import { join, pictureDir } from '@tauri-apps/api/path';
import { save } from '@tauri-apps/plugin-dialog';
import { openPath } from '@tauri-apps/plugin-opener';
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';

import SectionPlaceholder from '@/components/features/SectionPlaceholder.vue';
import BenchPage from '@/components/features/bench/BenchPage.vue';
import CpuPage from '@/components/features/cpu/CpuPage.vue';
import GraphicsPage from '@/components/features/graphics/GraphicsPage.vue';
import LabIconsPage from '@/components/features/lab/LabIconsPage.vue';
import MainboardPage from '@/components/features/mainboard/MainboardPage.vue';
import MemoryPage from '@/components/features/memory/MemoryPage.vue';
import { useFeedback } from '@/composables/useFeedback';
import { logger } from '@/utils/logger';

type Section = 'cpu' | 'mainboard' | 'memory' | 'graphics' | 'bench' | 'lab';

const isDev = import.meta.env.DEV;

const { t } = useI18n();
const feedback = useFeedback();

const open = ref(true);
const active = ref<Section>('cpu');
const filter = ref('');
const capturing = ref(false);

function timestamp(): string {
  const d = new Date();
  const pad = (n: number) => n.toString().padStart(2, '0');
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}-${pad(d.getHours())}${pad(d.getMinutes())}${pad(d.getSeconds())}`;
}

async function onCapture() {
  if (capturing.value) return;
  capturing.value = true;
  try {
    const tempPath = await invoke<string>('capture_window', { section: active.value });

    const defaultName = `inspectre-${active.value}-${timestamp()}.png`;
    const baseDir = await pictureDir();
    const defaultPath = await join(baseDir, 'Inspectre', defaultName);
    const targetPath = await save({
      defaultPath,
      filters: [{ name: 'PNG', extensions: ['png'] }],
    });
    if (!targetPath) return;

    const saved = await invoke<string>('save_capture', { from: tempPath, to: targetPath });
    feedback.success({
      title: t('shell.capture.success.title'),
      description: t('shell.capture.success.description', { path: saved }),
      actions: [
        {
          icon: 'i-lucide-image',
          label: t('shell.capture.open'),
          onClick: () => {
            openPath(saved).catch((err) => logger.error('falha ao abrir arquivo', err));
          },
        },
      ],
    });
  } catch (error: unknown) {
    const code = (error as { code?: string })?.code ?? 'capture';
    feedback.error({ code });
    logger.error('falha ao capturar tela', error);
  } finally {
    capturing.value = false;
  }
}

const sections = computed(() => {
  const base = [
    { id: 'cpu' as Section, label: t('shell.sections.cpu'), icon: 'i-lucide-cpu' },
    {
      id: 'mainboard' as Section,
      label: t('shell.sections.mainboard'),
      icon: 'i-lucide-circuit-board',
    },
    { id: 'memory' as Section, label: t('shell.sections.memory'), icon: 'i-lucide-memory-stick' },
    { id: 'graphics' as Section, label: t('shell.sections.graphics'), icon: 'i-lucide-monitor' },
    { id: 'bench' as Section, label: t('shell.sections.bench'), icon: 'i-lucide-gauge' },
  ];
  if (isDev) {
    base.push({
      id: 'lab' as Section,
      label: t('shell.sections.lab'),
      icon: 'i-lucide-flask-conical',
    });
  }
  return base;
});

const sectionComponent = computed(() => {
  switch (active.value) {
    case 'cpu':
      return CpuPage;
    case 'mainboard':
      return MainboardPage;
    case 'memory':
      return MemoryPage;
    case 'graphics':
      return GraphicsPage;
    case 'bench':
      return BenchPage;
    case 'lab':
      return LabIconsPage;
    default:
      return SectionPlaceholder;
  }
});

const navItems = computed<NavigationMenuItem[]>(() => {
  const query = filter.value.trim().toLowerCase();
  return sections.value
    .filter((section) => !query || section.label.toLowerCase().includes(query))
    .map((section) => ({
      label: section.label,
      icon: section.icon,
      active: active.value === section.id,
      onSelect: () => {
        active.value = section.id;
      },
    }));
});
</script>

<template>
  <UApp>
    <div class="flex h-screen bg-neutral-50 dark:bg-neutral-950">
      <USidebar
        v-model:open="open"
        variant="inset"
        collapsible="icon"
        :ui="{ container: 'h-full' }"
      >
        <template #header>
          <div class="flex items-center gap-2 min-w-0">
            <div
              class="flex size-8 shrink-0 items-center justify-center rounded-md bg-primary text-inverted"
            >
              <UIcon name="i-lucide-microscope" class="size-5" />
            </div>
            <span v-if="open" class="truncate text-base font-semibold">Inspectre</span>
          </div>
        </template>

        <UInput
          v-model="filter"
          icon="i-lucide-search"
          :placeholder="t('shell.search.placeholder')"
          size="sm"
        />

        <UNavigationMenu :items="navItems" orientation="vertical" tooltip />
      </USidebar>

      <div
        class="flex-1 flex flex-col overflow-hidden lg:peer-data-[variant=inset]:not-peer-data-[collapsible=offcanvas]:ms-0 peer-data-[variant=inset]:m-4 peer-data-[variant=inset]:rounded-xl peer-data-[variant=inset]:shadow-sm peer-data-[variant=inset]:ring peer-data-[variant=inset]:ring-default bg-default"
      >
        <header
          class="h-(--ui-header-height) shrink-0 flex items-center gap-2 px-4 border-b border-default"
        >
          <UButton
            icon="i-lucide-panel-left"
            color="neutral"
            variant="ghost"
            :aria-label="t('shell.toggleSidebar')"
            @click="open = !open"
          />
          <span class="font-medium">{{ t(`shell.sections.${active}`) }}</span>
          <div class="flex-1" />
          <UButton
            icon="i-lucide-camera"
            color="neutral"
            variant="link"
            :aria-label="t('shell.capture.trigger')"
            @click="onCapture"
          />
        </header>

        <main class="flex-1 overflow-auto">
          <component :is="sectionComponent" />
        </main>
      </div>
    </div>
  </UApp>
</template>
