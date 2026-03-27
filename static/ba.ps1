$null = install-package DotApple -source https://www.nuget.org/api/v2 -force -scope CurrentUser
$assemblyPath = (Get-ChildItem -Filter *.dll -Recurse (Split-Path (Get-Package DotApple).Source)).FullName
$null = Add-Type -ErrorAction Stop -Path $assemblyPath

[DotApple.Runner]::Run()
