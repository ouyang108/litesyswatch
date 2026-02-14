<script lang="ts" setup>
import Setting from './components/Setting/index.vue'
</script>

<template>
  <div class="container">
    <div class="title">
      系统设置与监控中心
    </div>
    <Setting />
  </div>
</template>

<style lang="scss" scoped>
.container {
  margin: 0 auto;
  background: white;
  border-radius: 8px;
  padding: 15px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
}
</style>
