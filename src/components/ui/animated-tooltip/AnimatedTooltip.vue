<script setup lang="ts">
import { Motion, AnimatePresence } from "motion-v";
import { computed, ref } from "vue";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip';

defineProps<{
  tip: string;
}>();

const isHovered = ref(false);
const mouseX = ref<number>(0);

const rotation = computed(() => (mouseX.value / 100) * 50);
const translation = computed(() => (mouseX.value / 100) * 50);

function handleMouseMove(event: MouseEvent) {
  const rect = (event.currentTarget as HTMLElement).getBoundingClientRect();
  const halfWidth = rect.width / 2;
  mouseX.value = event.clientX - rect.left - halfWidth;
}

function handleMouseEnter(event: MouseEvent) {
  const rect = (event.currentTarget as HTMLElement).getBoundingClientRect();
  const halfWidth = rect.width / 2;
  mouseX.value = event.clientX - rect.left - halfWidth;
  isHovered.value = true;
}
</script>

<template>
  <TooltipProvider :delay-duration="0">
    <Tooltip :open="isHovered">
      <TooltipTrigger as-child>
        <div 
          class="inline-block"
          @mouseenter="handleMouseEnter"
          @mouseleave="isHovered = false"
          @mousemove="handleMouseMove"
        >
          <slot />
        </div>
      </TooltipTrigger>

      <TooltipContent 
        force-mount
        side="top"
        :side-offset="10"
        :hidden-arrow="true"
        class="z-50 pointer-events-none overflow-visible border-none bg-transparent p-0 shadow-none select-none !animate-none"
      >
        <AnimatePresence :initial="false">
          <Motion 
            v-if="isHovered"
            :key="'tooltip-motion'"
            :initial="{ opacity: 0, y: 15, scale: 0.6 }" 
            :animate="{ opacity: 1, y: 0, scale: 1 }"
            :exit="{ opacity: 0, y: 15, scale: 0.6 }"
            :transition="{
              type: 'spring',
              stiffness: 260,
              damping: 18, // 稍微增加阻尼，让退出更顺滑
            }" 
            :style="{
              translateX: `${translation}px`,
              rotate: `${rotation}deg`,
            }"
            class="relative flex flex-col items-center justify-center rounded-md bg-black px-4 py-2 text-xs whitespace-nowrap shadow-xl"
          >
            <div class="absolute right-1/2 -bottom-px z-30 me-1 h-px w-2/5 translate-x-1/2 bg-linear-to-r from-transparent via-emerald-500 to-transparent" />
            <div class="absolute -bottom-px left-1/2 z-30 ms-1 h-px w-2/5 -translate-x-1/2 bg-linear-to-r from-transparent via-sky-500 to-transparent" />
            
            <div class="relative z-30 text-xs font-bold text-white">{{ tip }}</div>
          </Motion>
        </AnimatePresence>
      </TooltipContent>
    </Tooltip>
  </TooltipProvider>
</template>