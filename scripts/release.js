import fs from "node:fs"
import { release } from "@varlet/release"

async function task() {
  const version = JSON.parse(fs.readFileSync("package.json", "utf-8")).version

  const tauriCOnfig = JSON.parse(
    fs.readFileSync("src-tauri/tauri.conf.json", "utf-8"),
  )

  tauriCOnfig.package.version = version
  fs.writeFileSync(
    "src-tauri/tauri.conf.json",
    JSON.stringify(tauriCOnfig, null, 2),
  )
}

release({ task, skipNpmPublish: true })
