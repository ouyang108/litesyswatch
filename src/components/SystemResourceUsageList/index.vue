<script setup lang="ts">
import type { SysStatus } from '../../types'
import { inject } from 'vue'

const sysStatusRef = inject<SysStatus>('sysStatusRef')
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
      <div id="cpuUsage" class="stat-value">
        {{ sysStatusRef?.cpu || '0' }}%
      </div>
    </div>
    <!-- 内存占用 -->
    <div class="stat-card">
      <div class="stat-label">
        当前内存总占用
      </div>
      <div id="memUsage" class="stat-value">
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
</style>
