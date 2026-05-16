<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';

import AppButton from '@/components/ui/AppButton.vue';
import AppCard from '@/components/ui/AppCard.vue';
import { useFeedback } from '@/composables/useFeedback';
import { logger } from '@/utils/logger';

const { t } = useI18n();
const feedback = useFeedback();

const SIZE = 1024;
const PREVIEW_SIZES = [16, 32, 64, 128] as const;

const mainCanvas = ref<HTMLCanvasElement | null>(null);
const preview16 = ref<HTMLCanvasElement | null>(null);
const preview32 = ref<HTMLCanvasElement | null>(null);
const preview64 = ref<HTMLCanvasElement | null>(null);
const preview128 = ref<HTMLCanvasElement | null>(null);
const primaryProbe = ref<HTMLDivElement | null>(null);

const backgroundColor = ref('#10b981');
const iconColor = ref('#ffffff');
const cornerRadiusPercent = ref(22.4);
const iconScale = ref(55);
const saving = ref(false);

const MICROSCOPE_SVG = (color: string) =>
  `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="${color}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 18h8"/><path d="M3 22h18"/><path d="M14 22a7 7 0 1 0 0-14h-1"/><path d="M9 14h2"/><path d="M9 12a2 2 0 0 1-2-2V6h6v4a2 2 0 0 1-2 2Z"/><path d="M12 6V3a1 1 0 0 0-1-1H9a1 1 0 0 0-1 1v3"/></svg>`;

async function loadIconImage(color: string): Promise<HTMLImageElement> {
  const url = `data:image/svg+xml;charset=utf-8,${encodeURIComponent(MICROSCOPE_SVG(color))}`;
  const img = new Image();
  img.src = url;
  await new Promise<void>((resolve, reject) => {
    img.onload = () => resolve();
    img.onerror = () => reject(new Error('falha ao carregar SVG'));
  });
  return img;
}

function drawRoundedRect(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  w: number,
  h: number,
  r: number,
) {
  const radius = Math.min(r, w / 2, h / 2);
  ctx.beginPath();
  ctx.moveTo(x + radius, y);
  ctx.arcTo(x + w, y, x + w, y + h, radius);
  ctx.arcTo(x + w, y + h, x, y + h, radius);
  ctx.arcTo(x, y + h, x, y, radius);
  ctx.arcTo(x, y, x + w, y, radius);
  ctx.closePath();
}

async function renderTo(canvas: HTMLCanvasElement, size: number, icon: HTMLImageElement) {
  canvas.width = size;
  canvas.height = size;
  const ctx = canvas.getContext('2d');
  if (!ctx) return;
  ctx.clearRect(0, 0, size, size);

  const radius = (size * cornerRadiusPercent.value) / 100;
  ctx.fillStyle = backgroundColor.value;
  drawRoundedRect(ctx, 0, 0, size, size, radius);
  ctx.fill();

  const iconPx = (size * iconScale.value) / 100;
  const offset = (size - iconPx) / 2;
  ctx.drawImage(icon, offset, offset, iconPx, iconPx);
}

async function render() {
  const icon = await loadIconImage(iconColor.value);
  const targets: Array<[HTMLCanvasElement | null, number]> = [
    [mainCanvas.value, SIZE],
    [preview16.value, 16],
    [preview32.value, 32],
    [preview64.value, 64],
    [preview128.value, 128],
  ];
  for (const [canvas, size] of targets) {
    if (canvas) await renderTo(canvas, size, icon);
  }
}

async function save() {
  if (!mainCanvas.value || saving.value) return;
  saving.value = true;
  try {
    const blob: Blob = await new Promise((resolve, reject) => {
      mainCanvas.value!.toBlob(
        (b) => (b ? resolve(b) : reject(new Error('toBlob retornou null'))),
        'image/png',
      );
    });
    const bytes = Array.from(new Uint8Array(await blob.arrayBuffer()));
    const path = await invoke<string>('save_icon_source', { bytes });
    feedback.success({
      title: t('lab.icons.saved.title'),
      description: t('lab.icons.saved.description', { path }),
    });
  } catch (err) {
    logger.warn('falha ao salvar source', err);
    feedback.error({ title: t('lab.icons.errors.save') });
  } finally {
    saving.value = false;
  }
}

function rgbToHex(rgb: string): string | null {
  const match = rgb.match(/\d+/g);
  if (!match || match.length < 3) return null;
  const [r, g, b] = match.slice(0, 3).map(Number);
  return `#${[r, g, b].map((n) => n.toString(16).padStart(2, '0')).join('')}`;
}

onMounted(async () => {
  if (primaryProbe.value) {
    const bg = getComputedStyle(primaryProbe.value).backgroundColor;
    const hex = rgbToHex(bg);
    if (hex) backgroundColor.value = hex;
  }
  await render();
});

watch([backgroundColor, iconColor, cornerRadiusPercent, iconScale], () => {
  render();
});
</script>

<template>
  <div class="space-y-4 p-4">
    <header class="flex items-start justify-between gap-4">
      <div class="min-w-0">
        <h1 class="text-base font-semibold">{{ t('lab.icons.title') }}</h1>
        <p class="text-dimmed text-xs">{{ t('lab.icons.description') }}</p>
      </div>
      <AppButton
        :label="saving ? t('lab.icons.actions.saving') : t('lab.icons.actions.save')"
        icon="i-lucide-download"
        color="primary"
        size="sm"
        :loading="saving"
        :disabled="saving"
        @click="save"
      />
    </header>

    <div class="grid grid-cols-1 gap-4 lg:grid-cols-[1fr_320px]">
      <AppCard :title="t('lab.icons.sections.preview')">
        <div class="flex flex-col items-center gap-4">
          <canvas
            ref="mainCanvas"
            class="border-default rounded-md border"
            style="width: 256px; height: 256px"
          />
          <div class="flex items-end gap-6">
            <div
              v-for="px in PREVIEW_SIZES"
              :key="px"
              class="flex flex-col items-center gap-1"
            >
              <canvas
                :ref="
                  (el) => {
                    if (px === 16) preview16 = el as HTMLCanvasElement | null;
                    else if (px === 32) preview32 = el as HTMLCanvasElement | null;
                    else if (px === 64) preview64 = el as HTMLCanvasElement | null;
                    else if (px === 128) preview128 = el as HTMLCanvasElement | null;
                  }
                "
                :style="{ width: `${px}px`, height: `${px}px` }"
              />
              <span class="text-dimmed font-mono text-[10px]">{{ px }}px</span>
            </div>
          </div>
        </div>
      </AppCard>

      <AppCard :title="t('lab.icons.sections.controls')">
        <dl class="space-y-3 text-xs">
          <div class="space-y-1">
            <dt class="text-dimmed">{{ t('lab.icons.fields.background') }}</dt>
            <dd class="flex items-center gap-2">
              <input
                v-model="backgroundColor"
                type="color"
                class="border-default h-8 w-12 cursor-pointer rounded border"
              />
              <span class="text-default font-mono">{{ backgroundColor }}</span>
            </dd>
          </div>

          <div class="space-y-1">
            <dt class="text-dimmed">{{ t('lab.icons.fields.iconColor') }}</dt>
            <dd class="flex items-center gap-2">
              <input
                v-model="iconColor"
                type="color"
                class="border-default h-8 w-12 cursor-pointer rounded border"
              />
              <span class="text-default font-mono">{{ iconColor }}</span>
            </dd>
          </div>

          <div class="space-y-1">
            <dt class="text-dimmed flex items-center justify-between">
              <span>{{ t('lab.icons.fields.cornerRadius') }}</span>
              <span class="font-mono">{{ cornerRadiusPercent.toFixed(1) }}%</span>
            </dt>
            <dd>
              <input
                v-model.number="cornerRadiusPercent"
                type="range"
                min="0"
                max="50"
                step="0.5"
                class="w-full"
              />
            </dd>
          </div>

          <div class="space-y-1">
            <dt class="text-dimmed flex items-center justify-between">
              <span>{{ t('lab.icons.fields.iconScale') }}</span>
              <span class="font-mono">{{ iconScale }}%</span>
            </dt>
            <dd>
              <input
                v-model.number="iconScale"
                type="range"
                min="20"
                max="80"
                step="1"
                class="w-full"
              />
            </dd>
          </div>
        </dl>
      </AppCard>
    </div>

    <div ref="primaryProbe" class="bg-primary pointer-events-none absolute h-0 w-0 opacity-0" />
  </div>
</template>
