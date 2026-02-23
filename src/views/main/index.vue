<script lang="ts" setup>
import { getCurrentWindow } from '@tauri-apps/api/window'

import { onBeforeUnmount, onMounted, useTemplateRef } from 'vue'
import highUsageProcessList from '../../components/HighUsageProcessList/index.vue'
import Setting from '../../components/Setting/index.vue'
import systemResourceUsageList from '../../components/SystemResourceUsageList/index.vue'
import { showAndHiddenWindow } from '../../utils/index'

const appWindow = getCurrentWindow()
let unlisten: () => void
const setting = useTemplateRef('setting')

// 桌面
// 监听应用
async function listenCurrentWindow() {
  // 监听最小化
  unlisten = await appWindow.onResized(async () => {
    const minimized = await appWindow.isMinimized()
    if (!setting.value || !setting.value?.setting?.closeMonitor)
      return
    if (minimized) {
      showAndHiddenWindow('float')
    }
    else {
      console.log('窗口不是最小化状态')
    }
  })
}

onMounted(async () => {
  await listenCurrentWindow()
})
onBeforeUnmount(() => {
  if (unlisten) {
    unlisten()
  }
})
</script>

<template>
  <div class="container">
    <div class="title">
      系统设置与监控中心
    </div>
    <Setting ref="setting" />
    <!-- 系统资源占用列表 -->
    <systemResourceUsageList :setting="setting?.setting" />
    <!-- 高占用进程 -->
    <highUsageProcessList />
  </div>
</template>

<style lang="scss" scoped>
.container {
  margin: 0 auto;
  background: white;
  border-radius: 8px;
  padding: 15px 20px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
}

.title {
  font-size: 20px;
  font-weight: 600;
  margin-bottom: 20px;
  padding-bottom: 10px;
  border-bottom: 1px solid #eee;
}
</style>
