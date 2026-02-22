<script setup lang="ts">
import type { SysStatus } from '../../types'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { inject } from 'vue'
import { showAndHiddenWindow } from '../../utils/index'

const sysStatusRef = inject<SysStatus>('sysStatusRef')

async function dblclick() {
  const mainWin = await WebviewWindow.getByLabel('main')

  showAndHiddenWindow('main')
  if (mainWin) {
    // 取消最小化
    await mainWin.unminimize()
  }
}
</script>

<template>
  <div class="mini-monitor">
    <div id="closeBtn" class="close-btn" @click="dblclick">
      ×
    </div>
    <div class="monitor-item">
      <span class="label">CPU</span>
      <span id="cpuValue" class="value">{{ sysStatusRef?.cpu || '0' }}%</span>
    </div>
    <div class="monitor-item">
      <span class="label">内存</span>
      <span id="memValue" class="value">{{ sysStatusRef?.mem || '0' }}</span>
    </div>
  </div>
</template>

<style lang='scss' scoped>
$height:14px;
.mini-monitor {
    width: 100%;
    height: 100%;
    user-select: none;
    -webkit-app-region: drag;

    .title {
        -webkit-app-region: none;
    }

}

.mini-monitor {
    padding: 8px 10px;
    background: rgba(0, 0, 0, 0.7);
    border-radius: 6px;
    color: #fff;
    font-size: 12px;
    backdrop-filter: blur(4px);
    border: 1px solid rgba(255, 255, 255, 0.1);
}

.monitor-item {
    display: flex;
    justify-content: space-between;
    margin: 4px 0;
    margin-top: 7px;
}

.label {
    color: #ccc;
}

.value {
    color: #409eff;
    font-weight: 600;
}

/* 高占用警示样式 */
.high-usage {
    color: #f56c6c;
}

.close-btn {
    position: absolute;
    top: 1px;
    right: 4px;
    width: $height;
    height: $height;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.2);
    color: #fff;
    text-align: center;
    line-height: $height;
    font-size: 10px;
    cursor: pointer;
    -webkit-app-region: no-drag;
    /* 关闭按钮不触发拖动 */
    transition: background 0.2s;
}

.close-btn:hover {
    background: #f56c6c;
}
</style>
