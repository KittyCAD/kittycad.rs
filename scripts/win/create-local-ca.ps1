param(
  [string]$RootCN,
  [string]$ServerCN,
  [string]$PfxPath,
  [string]$PfxPassword,
  [ValidateSet('CurrentUser','LocalMachine')]
  [string]$TrustScope,
  [switch]$NoTrust
)

$ErrorActionPreference = 'Stop'

if (-not $RootCN) { $RootCN = 'KittyCAD-Test-Root' }
if (-not $ServerCN) { $ServerCN = 'localhost' }
if (-not $PfxPath) { $PfxPath = 'servercert.pfx' }
if (-not $PfxPassword) { $PfxPassword = 'pass' }

# Pick trust scope:
# - In CI (e.g., GitHub Actions), prefer LocalMachine to avoid UI prompts on CurrentUser\Root
# - Otherwise default to CurrentUser for least privilege
if (-not $TrustScope) {
  if ($env:ZOO_CA_TRUST_SCOPE) {
    $TrustScope = $env:ZOO_CA_TRUST_SCOPE
  } elseif ($env:GITHUB_ACTIONS -eq 'true' -or $env:CI -eq 'true') {
    $TrustScope = 'LocalMachine'
  } else {
    $TrustScope = 'CurrentUser'
  }
}

function Log([string]$msg) {
  $ts = Get-Date -Format 'HH:mm:ss'
  Write-Host "[$ts] $msg"
}

function Invoke-With-Timeout([ScriptBlock]$ScriptBlock, [int]$TimeoutSeconds, [object[]]$ArgumentList) {
  $job = Start-Job -ScriptBlock $ScriptBlock -ArgumentList $ArgumentList
  try {
    if (Wait-Job -Job $job -Timeout $TimeoutSeconds) {
      $res = Receive-Job -Job $job -ErrorAction Stop
      return $res
    } else {
      throw "Timeout after $TimeoutSeconds seconds"
    }
  } finally {
    Stop-Job $job -ErrorAction SilentlyContinue | Out-Null
    Remove-Job $job -ErrorAction SilentlyContinue | Out-Null
  }
}

function Invoke-ProcessWithTimeout([string]$File, [string]$Arguments, [int]$TimeoutSeconds) {
  $psi = New-Object System.Diagnostics.ProcessStartInfo
  $psi.FileName = $File
  $psi.Arguments = $Arguments
  $psi.UseShellExecute = $false
  $psi.RedirectStandardOutput = $true
  $psi.RedirectStandardError = $true
  $p = New-Object System.Diagnostics.Process
  $p.StartInfo = $psi
  [void]$p.Start()
  if ($p.WaitForExit($TimeoutSeconds * 1000)) {
    $stdout = $p.StandardOutput.ReadToEnd()
    $stderr = $p.StandardError.ReadToEnd()
    if ($p.ExitCode -ne 0) {
      $msg = "Process failed ($File): $($p.ExitCode)"
      if ($stderr) { $msg += "`nSTDERR:`n$stderr" }
      if ($stdout) { $msg += "`nSTDOUT:`n$stdout" }
      throw $msg
    }
    return $stdout
  } else {
    try { $p.Kill() } catch {}
    throw "Process timeout: $File $Arguments"
  }
}

Log "Creating local root CA and server cert..."


# Create a root CA in CurrentUser store (use splatting for reliability)
$rootParams = @{
  Type              = 'Custom'
  KeySpec           = 'Signature'
  Subject           = "CN=$RootCN"
  KeyExportPolicy   = 'Exportable'
  HashAlgorithm     = 'sha256'
  KeyLength         = 2048
  CertStoreLocation = 'Cert:\CurrentUser\My'
  # Pass as string array so binder maps to enum values
  KeyUsage          = @('CertSign','CRLSign','DigitalSignature')
  NotAfter          = (Get-Date).AddMonths(3)
  TextExtension     = @('2.5.29.19={text}CA=1&pathlength=3')
}
$sw = [System.Diagnostics.Stopwatch]::StartNew()
$root = New-SelfSignedCertificate @rootParams
Log "Root created in $($sw.Elapsed.TotalSeconds.ToString('0.00'))s"

# Create a server cert signed by the root
$serverParams = @{
  Type              = 'Custom'
  Subject           = "CN=$ServerCN"
  KeyExportPolicy   = 'Exportable'
  HashAlgorithm     = 'sha256'
  KeyLength         = 2048
  CertStoreLocation = 'Cert:\CurrentUser\My'
  # Pass as string array
  KeyUsage          = @('DigitalSignature','KeyEncipherment')
  Signer            = $root
  # BasicConstraints CA=0, EKU=Server Authentication, SAN: dns=localhost, ip=127.0.0.1
  TextExtension     = @(
    '2.5.29.19={text}CA=0',
    '2.5.29.37={text}1.3.6.1.5.5.7.3.1',
    '2.5.29.17={text}DNS=localhost&IPAddress=127.0.0.1&IPAddress=::1'
  )
}
$sw.Restart()
$server = New-SelfSignedCertificate @serverParams
Log "Server cert created in $($sw.Elapsed.TotalSeconds.ToString('0.00'))s"

# Optionally trust the root CA. Scope is configurable.
$rootCer = Join-Path $PWD 'root.cer'
$rootPem = Join-Path $PWD 'root.pem'
$sw.Restart()
Export-Certificate -Cert $root -FilePath $rootCer | Out-Null
Log "Root exported in $($sw.Elapsed.TotalSeconds.ToString('0.00'))s"

# Also produce PEM for NODE_EXTRA_CA_CERTS consumers (Node expects PEM)
try {
  $sw.Restart()
  Invoke-ProcessWithTimeout 'certutil' "-encode `"$rootCer`" `"$rootPem`"" 10 | Out-Null
  Log "PEM exported in $($sw.Elapsed.TotalSeconds.ToString('0.00'))s"
} catch {
  Log "PEM export via certutil failed: $($_.Exception.Message)"
}

$timeout = [int]([Environment]::GetEnvironmentVariable('ZOO_CA_IMPORT_TIMEOUT') ?? '120')
if ($NoTrust) {
  Log "Skipping trust import (NoTrust specified)."
} else {
  Log "Importing root into $TrustScope\\Root (timeout ${timeout}s)..."
  $sw.Restart()
  # Try .NET X509Store import synchronously first
  try {
    $storeLocation = if ($TrustScope -eq 'LocalMachine') { [System.Security.Cryptography.X509Certificates.StoreLocation]::LocalMachine } else { [System.Security.Cryptography.X509Certificates.StoreLocation]::CurrentUser }
    $cert = [System.Security.Cryptography.X509Certificates.X509Certificate2]::new($rootCer)
    $store = [System.Security.Cryptography.X509Certificates.X509Store]::new('Root', $storeLocation)
    $store.Open([System.Security.Cryptography.X509Certificates.OpenFlags]::ReadWrite)
    try { $store.Add($cert) } finally { $store.Close() }
    Log "Root imported in $($sw.Elapsed.TotalSeconds.ToString('0.00'))s"
  } catch {
    Log ".NET store import failed: $($_.Exception.GetType().FullName): $($_.Exception.Message). Falling back to certutil..."
    $sw.Restart()
    $args = if ($TrustScope -eq 'LocalMachine') { "-f -q -addstore Root `"$rootCer`"" } else { "-user -f -q -addstore Root `"$rootCer`"" }
    Invoke-ProcessWithTimeout 'certutil' $args $timeout | Out-Null
    Log "certutil import completed in $($sw.Elapsed.TotalSeconds.ToString('0.00'))s"
  }
}

# Export server cert as PFX for the Node HTTPS server
$pwd = ConvertTo-SecureString -String $PfxPassword -Force -AsPlainText
$sw.Restart()
Export-PfxCertificate -Cert $server -FilePath $PfxPath -Password $pwd | Out-Null
Log "PFX exported in $($sw.Elapsed.TotalSeconds.ToString('0.00'))s"

if ($NoTrust) {
  Log "Local CA created. Trust import skipped. PFX at $PfxPath"
} else {
  Log "Local CA created and trusted. PFX at $PfxPath"
}
