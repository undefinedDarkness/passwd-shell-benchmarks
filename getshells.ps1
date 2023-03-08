#!/usr/bin/env pwsh

# Read the passwd file and count the instances of each login shell (the last field)
$shells = Get-Content -Path "./passwd" | ForEach-Object { ($_ -split ':')[-1] } | Group-Object

# For each login shell, print the number of accounts using each shell
$shells | ForEach-Object {
    $shell = $_.Name
    $count = $_.Count
    Write-Output "$shell : $($count)"
}
