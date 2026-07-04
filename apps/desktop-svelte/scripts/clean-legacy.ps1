$dirs = @(
  "src\lib\components\login",
  "src\lib\components\onboarding",
  "src\lib\components\widgets",
  "src\lib\components\academics",
  "src\lib\components\students",
  "src\lib\components\exams",
  "src\lib\components\metrics",
  "src\lib\components\bulk-input",
  "src\lib\components\template-designer",
  "src\lib\components\shared",
  "src\lib\components\skeletons",
  "src\lib\components\layout",
  "src\lib\components\forms",
  "src\lib\components\sheets",
  "src\lib\services\queries"
)

$files = @(
  "src\lib\utils\debounce.ts",
  "src\lib\utils\ipc-error.ts",
  "src\lib\i18n-utils.ts",
  "src\lib\services\ipc.ts",
  "src\lib\services\ipc-dedup.ts",
  "src\lib\services\query-client.ts",
  "src\lib\services\prefetch.ts",
  "src\lib\stores\academic.svelte.ts",
  "src\lib\stores\students.svelte.ts",
  "src\lib\stores\exams.svelte.ts"
)

foreach ($dir in $dirs) {
  if (Test-Path $dir) {
    Remove-Item -Path $dir -Recurse -Force
    Write-Host "Deleted directory: $dir"
  }
}

foreach ($file in $files) {
  if (Test-Path $file) {
    Remove-Item -Path $file -Force
    Write-Host "Deleted file: $file"
  }
}

# Also clean up empty parents if needed
if (Test-Path "src\lib\services") {
  $items = Get-ChildItem "src\lib\services"
  if ($items.Count -eq 0) {
    Remove-Item "src\lib\services" -Force
    Write-Host "Deleted empty services folder"
  }
}

Write-Host "Clean-up finished successfully."
