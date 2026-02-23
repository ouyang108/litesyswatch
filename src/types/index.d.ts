export interface SysStatus {
  cpu: string
  mem: string
  mem_used: string
  top_cpu: { name: string, value: string }
  top_mem: { name: string, value: string }
}
export interface Setting {
  closeMonitor: boolean
  cpuThreshold: number
  memThreshold: number
}
