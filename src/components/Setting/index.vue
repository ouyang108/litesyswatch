<script setup lang="ts">
import { ref } from 'vue'

const setting = ref({

  closeMonitor: true,
  cpuThreshold: 80,
  memThreshold: 80,
})
defineExpose({
  setting,
})
</script>

<template>
  <!-- 设置区域 -->
  <div class="setting-section">
    <div class="section-title">
      基础设置
    </div>

    <!-- 悬浮显示CPU内存开关 -->
    <div style="margin-top: 15px;">
      <label class="switch">
        <input id="floatMonitorSwitch" v-model="setting.closeMonitor" type="checkbox">
        <span class="slider" />
      </label>
      <span id="floatSwitchText" class="switch-text">启动悬浮监控窗</span>
    </div>
    <!-- cpu阈值 -->
    <div style="margin-top: 15px;">
      <label>CPU警告阈值：</label>
      <input
        v-model="setting.cpuThreshold" type="number" class="threshold-input"
        style="width: 60px; margin-left: 10px;" min="0" max="100"
      > %
      <div class="tips">
        如果为0，则不警告
      </div>
    </div>
    <!-- 内存阈值 -->
    <div style="margin-top: 15px;">
      <label>内存警告阈值：</label>
      <input
        v-model="setting.memThreshold" type="number" class="threshold-input"
        style="width: 60px; margin-left: 10px;" min="0" max="100"
      > %
      <div class="tips">
        如果为0，则不警告
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

.threshold-input {
  width: 80px;
  padding: 6px 8px;
  border: 1px solid #e6e6e6;
  border-radius: 4px;
  font-size: 14px;
  outline: none;
}

.threshold-input:focus {
  border-color: #409eff;
}
.tips {

  font-size: 12px;
  color: #909399;
}
</style>
