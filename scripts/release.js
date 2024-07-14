import fs from 'node:fs'
import { release } from '@varlet/release'

async function task() {
 const version =  fs.readFileSync('package.json', 'utf-8').toJSON().version

 const tauriCOnfig = fs.readFileSync('../src-tauri/tauri.conf.json', 'utf-8').toJSON()

 tauriCOnfig.version = version
 fs.writeFileSync('../src-tauri/tauri.conf.json', JSON.stringify(tauriCOnfig, null, 2))
}

release({ task, skipNpmPublish: true })
