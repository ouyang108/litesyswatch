<script lang="ts" setup>
import type { SysStatus } from './types'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { onBeforeUnmount, onMounted, provide, ref } from 'vue'

const appWindow = getCurrentWindow()
let sysStatusUnListen: () => void
const sysStatusRef = ref<SysStatus>({
  cpu: '0',
  mem: '0',
  mem_used: '0',
  top_cpu: { name: '', value: '' },
  top_mem: { name: '', value: '' },
})

async function listenSysStatus() {
  sysStatusUnListen = await appWindow.listen('sys-status', (event) => {
    if (event.event === 'sys-status') {
      sysStatusRef.value = Object.assign({}, sysStatusRef.value, event.payload)
    }
  })
}
// 监听F5键 禁止刷新
function handleF5Key(event: KeyboardEvent) {
  if (event.key === 'F5') {
    event.preventDefault()
  }
}
function handleContextMenu(event: MouseEvent) {
  event.preventDefault()
  // return () => window.removeEventListener('contextmenu', handleContextMenu)
}
onMounted(async () => {
  await listenSysStatus()
  // 监听F5键
  window.addEventListener('keydown', handleF5Key)
  // 监听右键菜单
  window.addEventListener('contextmenu', handleContextMenu)
})
onBeforeUnmount(() => {
  if (sysStatusUnListen) {
    sysStatusUnListen()
  }
  // 移除F5键监听
  window.removeEventListener('keydown', handleF5Key)
  // 移除右键菜单监听
  window.removeEventListener('contextmenu', handleContextMenu)
})
provide('sysStatusRef', sysStatusRef)
</script>

<template>
  <router-view />
</template>

<style lang="scss" scoped>

</style>
