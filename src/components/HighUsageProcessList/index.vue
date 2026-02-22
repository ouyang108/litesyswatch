<script setup lang="ts">
import { ask } from '@tauri-apps/plugin-dialog'

// 创建 Yes/No 对话框

async function close() {
  const answer = await ask('This action cannot be reverted. Are you sure?', {
    title: 'Tauri',
    kind: 'warning',
  })
  if (answer) {
    // 用户点击了 Yes
    console.log('用户点击了 Yes')
  }
  else {
    // 用户点击了 No
    console.log('用户点击了 No')
  }
}
</script>

<template>
  <!-- 高占用进程区域 -->
  <div class="monitor-section">
    <div class="section-title">
      高占用进程
    </div>
    <!-- CPU占用最高进程 -->
    <div class="process-item">
      <div class="process-info">
        <div id="topCpuProcessName" class="process-name">
          CPU占用最高：Chrome.exe
        </div>
        <div id="topCpuProcessUsage" class="process-usage">
          占用率：35.2%
        </div>
      </div>
      <button class="btn btn-danger" @click="close">
        关闭进程
      </button>
    </div>
    <!-- 内存占用最高进程 -->
    <div class="process-item">
      <div class="process-info">
        <div id="topMemProcessName" class="process-name">
          内存占用最高：Node.exe
        </div>
        <div id="topMemProcessUsage" class="process-usage">
          占用率：18.5%
        </div>
      </div>
      <button class="btn btn-danger">
        关闭进程
      </button>
    </div>
  </div>
</template>

<style lang='scss' scoped>
.process-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 15px;
    border: 1px solid #e6e6e6;
    border-radius: 6px;
    margin-bottom: 10px;
}

.process-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
}

.process-name {
    font-size: 14px;
    color: #303133;
}

.process-usage {
    font-size: 14px;
    color: #909399;
}

/* 按钮样式 */
.btn {
    padding: 6px 12px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 12px;
    transition: all 0.2s;
}

.btn-danger {
    background-color: #f56c6c;
    color: white;
}

.btn-danger:hover {
    background-color: #f78989;
}
</style>
