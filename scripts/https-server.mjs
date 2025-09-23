import fs from 'node:fs'
import https from 'node:https'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const port = Number(process.env.PORT || 4443)
const host = process.env.HOST || '127.0.0.1'

const here = path.dirname(fileURLToPath(import.meta.url))
const repoRoot = path.resolve(here, '..')
const pfxPath =
  process.env.PFX_PATH ||
  path.join(repoRoot, process.env.PFX_FILENAME || 'servercert.pfx')
const passphrase = process.env.PFX_PASSWORD || 'pass'

if (!fs.existsSync(pfxPath)) {
  console.error(`PFX not found at ${pfxPath}`)
  process.exit(2)
}

const pfx = fs.readFileSync(pfxPath)

const server = https.createServer({ pfx, passphrase }, (req, res) => {
  res.writeHead(200, { 'Content-Type': 'text/plain' })
  res.end('ok')
})

server.on('error', (err) => {
  console.error('HTTPS server error:', err)
  process.exit(1)
})

server.listen(port, host, () => {
  console.log(
    `https server listening at https://${host}:${port} using ${pfxPath}`
  )
})
