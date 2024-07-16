import { release } from "@varlet/release"

async function task(version: string) {
  const file = Bun.file("src-tauri/tauri.conf.json", {
    type: "application/json",
  })

  const tauriConfig = (await file.json()) as {
    package: { version: string }
  }

  tauriConfig.package.version = version

  await Bun.write(file, JSON.stringify(tauriConfig, null, 2))
}

release({ task, skipNpmPublish: true })
