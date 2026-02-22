<script setup lang="ts">
import { ref } from 'vue'

const setting = ref({
  autoClose: true,
  closeMonitor: true,
  monitorPosition: 'top-left',
})
const monitorPositionOptions = [
  { label: '左上角', value: 'top-left' },
  { label: '右上角', value: 'top-right' },
  { label: '左下角', value: 'bottom-left' },
  { label: '右下角', value: 'bottom-right' },
]
</script>

<template>
  <!-- 设置区域 -->
  <div class="setting-section">
    <div class="section-title">
      基础设置
    </div>
    <!-- 自动关闭软件开关 -->
    <label class="switch">
      <input id="autoQuitSwitch" v-model="setting.autoClose" type="checkbox">
      <span class="slider" />
    </label>
    <span id="switchText" class="switch-text">启动自动关闭</span>

    <!-- 悬浮显示CPU内存开关 -->
    <div style="margin-top: 15px;">
      <label class="switch">
        <input id="floatMonitorSwitch" v-model="setting.closeMonitor" type="checkbox">
        <span class="slider" />
      </label>
      <span id="floatSwitchText" class="switch-text">启动悬浮监控窗</span>

      <!-- 位置选择器 -->
      <div v-show="setting.closeMonitor" id="positionSelector" class="position-selector">
        <label v-for="item in monitorPositionOptions" :key="item.value">
          <input
            v-model="setting.monitorPosition" type="radio" name="monitorPosition" :value="item.value"
            :checked="setting.monitorPosition === item.value"
          > {{ item.label }}
        </label>
      </div>
    </div>
  </div>
</template>

<style lang='scss' scoped>
.setting-section {
  margin-bottom: 30px;
}

/* 自定义开关样式 */
.switch {
  position: relative;
  display: inline-block;
  width: 60px;
  height: 30px;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  transition: .4s;
  border-radius: 30px;
}

.slider:before {
  position: absolute;
  content: "";
  height: 22px;
  width: 22px;
  left: 4px;
  bottom: 4px;
  background-color: white;
  transition: .4s;
  border-radius: 50%;
}

input:checked+.slider {
  background-color: #409eff;
}

input:checked+.slider:before {
  transform: translateX(30px);
}

.switch-text {
  margin-left: 10px;
  font-size: 14px;
  color: #606266;
}

/* 位置选择器样式 */
.position-selector {
  margin: 15px 0 0 75px;
  display: block;
}

.position-selector.show {
  display: block;
}

.position-selector label {
  margin-right: 15px;
  font-size: 14px;
  color: #606266;
  cursor: pointer;
}
</style>
