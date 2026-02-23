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
onMounted(async () => {
  await listenSysStatus()
})
onBeforeUnmount(() => {
  if (sysStatusUnListen) {
    sysStatusUnListen()
  }
})
provide('sysStatusRef', sysStatusRef)
</script>

<template>
  <router-view />
</template>

<style lang="scss" scoped>

</style>
