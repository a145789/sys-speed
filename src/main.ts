import { listen } from "@tauri-apps/api/event"
import { invoke } from "@tauri-apps/api/tauri"
import { LogicalPosition, appWindow } from "@tauri-apps/api/window"

type SystemInfo = {
  cpuUsage: number
  memoryUsage: number
  networkSpeedUp: number
  networkSpeedDown: number
}

const MAX_KB = 1024 * 1024
function formatBytes(bytes: number) {
  if (bytes < 1024) {
    return `${bytes} B`
  }
  if (bytes < MAX_KB) {
    return `${(bytes / 1024).toFixed(0)} KB`
  }
  return `${(bytes / MAX_KB).toFixed(2)} MB`
}

function setElement(id: string, num: number) {
  const element = document.querySelector(id) as HTMLDivElement
  if (!element) {
    return
  }
  const _num = Number(num.toFixed(2))
  element.textContent = `${_num}%`
  if (_num > 80) {
    element.style.color = "#de2a18"
    element.style.fontWeight = "bold"
  } else {
    element.style.color = ""
    element.style.fontWeight = "normal"
  }
}

async function getSystemInfo() {
  const { cpuUsage, memoryUsage, networkSpeedDown, networkSpeedUp } =
    (await invoke("get_sys_info")) as SystemInfo

  setElement("#cpu-usage", cpuUsage)
  setElement("#memory-usage", memoryUsage)

  // biome-ignore lint/style/noNonNullAssertion: <explanation>
  document.querySelector("#network-usage")!.textContent = `↓ ${formatBytes(
    networkSpeedDown,
  )} / ↑ ${formatBytes(networkSpeedUp)}`
}

let timer: number
function loopGetInfo() {
  getSystemInfo()
  timer = setInterval(getSystemInfo, 1000)
}

function hiddenWindow() {
  console.log(222)

  invoke("plugin:window|hide_window")
}

window.onload = async () => {
  console.log("load")

  listen("tray-click", () => {
    invoke("plugin:window|show_window")
    clearInterval(timer)
    loopGetInfo()
  })

  listen("screen-center", async () => {
    invoke("plugin:window|show_window")
    await appWindow.center()
    clearInterval(timer)
    loopGetInfo()
  })

  listen("hidden", () => {
    clearInterval(timer)
  })

  await appWindow.setPosition(new LogicalPosition(1730, 810))

  window.addEventListener("mousedown", async () => {
    await appWindow.startDragging()
  })

  clearInterval(timer)
  loopGetInfo()

  // 禁用键盘事件
  document.addEventListener("keydown", (e) => e.preventDefault())
  // 禁用滚轮和鼠标右键
  document.addEventListener("wheel", (e) => e.preventDefault())
  document.addEventListener("contextmenu", (e) => e.preventDefault())
  // 禁用选择文本
  document.addEventListener("selectstart", (e) => e.preventDefault())

  // 监听鼠标左键双击
  document.addEventListener("dblclick", hiddenWindow)
}
