import { getAllWindows, getCurrentWindow } from '@tauri-apps/api/window'
import { ask } from '@tauri-apps/plugin-dialog'
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from '@tauri-apps/plugin-notification'

async function showNotification(title: string, body: string) {
  let permissionGranted = await isPermissionGranted()

  // 如果没有，我们需要请求它
  if (!permissionGranted) {
    const permission = await requestPermission()
    permissionGranted = permission === 'granted'
  }

  // 一旦获得许可，我们就可以发送通知
  if (permissionGranted) {
    sendNotification({ title, body })
  }
}
async function confirm(_message: string) {
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
const appWindow = getCurrentWindow()
async function showAndHiddenWindow(label: string) {
  // 显示桌面
  await appWindow.hide()
  // 2. 找到悬浮窗并显示
  const windows = await getAllWindows()
  const floatWin = windows.find(w => w.label === label)
  await floatWin?.show()
}
export {
  appWindow,
  confirm,
  showAndHiddenWindow,
  showNotification,
}
