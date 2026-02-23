<script setup lang="ts">
import type { Ref } from 'vue'
import type { Setting, SysStatus } from '../../types'
import { computed, inject, watch } from 'vue'
import { showNotification } from '../../utils/index'

const props = withDefaults(defineProps<Props>(), {
  setting: () => ({
    closeMonitor: false,
    cpuThreshold: 80,
    memThreshold: 80,
  }),
})
const CLASS_HIGH_USAGE = 'high-usage'
interface Props {
  setting?: Setting
}
const sysStatusRef = inject<Ref<SysStatus>>('sysStatusRef')
const cpuUsageClass = computed(() => {
  // 提取变量提高可读性，并处理可能的 Ref 取值问题
  const cpuUsed = Number(sysStatusRef?.value?.cpu || 0)
  const threshold = props.setting?.cpuThreshold // 假设默认阈值 80
  const result = Number(props.setting?.cpuThreshold) === 0 ? false : cpuUsed > threshold
  return {
    // 只有当布尔值为 true 时，'high-usage' 类才会被添加
    [CLASS_HIGH_USAGE]: result,
  }
})
let lastNotificationTime = 0
const COOLDOWN = 60000 // 冷却时间 1 分钟

let lastMemNotificationTime = 0

// memo
const memUsageClass = computed(() => {
  // 提取变量提高可读性，并处理可能的 Ref 取值问题
  const memUsed = Number(sysStatusRef?.value?.mem || 0)
  const threshold = props.setting?.memThreshold // 假设默认阈值 80
  const result = Number(props.setting?.memThreshold) === 0 ? false : memUsed > threshold
  return {
    // 只有当布尔值为 true 时，'high-usage' 类才会被添加
    [CLASS_HIGH_USAGE]: result,
  }
})
watch(() => cpuUsageClass.value, (newValue: Record<string, boolean>) => {
  if (newValue[CLASS_HIGH_USAGE]) {
    const now = Date.now()
    // 只有当超过阈值，且距离上次通知超过 1 分钟时才发送
    if (now - lastNotificationTime > COOLDOWN) {
      showNotification('⚠️ 系统性能预警', `当前 CPU 占用率已达 ${sysStatusRef?.value.cpu}%，请关注。`)
      lastNotificationTime = now
    }
  }
})
watch(() => memUsageClass.value, (newValue: Record<string, boolean>) => {
  if (newValue[CLASS_HIGH_USAGE]) {
    const now = Date.now()
    // 只有当超过阈值，且距离上次通知超过 1 分钟时才发送
    if (now - lastMemNotificationTime > COOLDOWN) {
      showNotification('⚠️ 系统性能预警', `当前内存占用率已达 ${sysStatusRef?.value.mem}%，请关注。`)
      lastMemNotificationTime = now
    }
  }
})
</script>

<template>
  <!-- 系统监控区域 -->
  <div class="monitor-section">
    <div class="section-title">
      系统资源占用
    </div>
    <!-- CPU占用 -->
    <div class="stat-card">
      <div class="stat-label">
        当前CPU总占用率
      </div>
      <div id="cpuUsage" class="stat-value" :class="cpuUsageClass">
        {{ sysStatusRef?.cpu || '0' }}%
      </div>
    </div>
    <!-- 内存占用 -->
    <div class="stat-card">
      <div class="stat-label">
        当前内存总占用
      </div>
      <div id="memUsage" class="stat-value" :class="memUsageClass">
        {{ sysStatusRef?.mem_used || '0' }} GB ({{ sysStatusRef?.mem || '0' }}%)
      </div>
    </div>
  </div>
</template>

<style lang='scss' scoped>
.stat-card {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 15px;
    background: #f8f9fa;
    border-radius: 6px;
    margin-bottom: 10px;
}

.stat-label {
    font-size: 14px;
    color: #606266;
}

.stat-value {
    font-size: 16px;
    font-weight: 600;
    color: #409eff;
}
.high-usage {
    color: #f56c6c;
}
</style>
