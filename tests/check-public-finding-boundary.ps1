$ErrorActionPreference = "Stop"

function Assert-Contains {
    param(
        [string]$Path,
        [string]$Needle
    )

    $text = Get-Content -Raw -Path $Path
    if ($text.IndexOf($Needle, [StringComparison]::Ordinal) -lt 0) {
        throw "$Path is missing required text: $Needle"
    }
}

$root = Split-Path -Parent $PSScriptRoot

$checks = @(
    @{
        Path = "README.md"
        Needles = @(
            "Research lab only",
            "public finding release boundary",
            "PACKET-PF-05"
        )
    },
    @{
        Path = "docs/adoption/README.md"
        Needles = @(
            "public finding release boundary",
            "Full parliament review",
            "editorial review"
        )
    },
    @{
        Path = "docs/findings/2026-06-broadband-adoption-divide.md"
        Needles = @(
            "Public release boundary",
            "not a network build plan",
            "PACKET-PF-05"
        )
    },
    @{
        Path = "docs/findings/public-finding-release-boundary.md"
        Needles = @(
            'This boundary closes `PACKET-PF-05`',
            "finding path and command used to regenerate it",
            "full parliament review",
            "If any field is missing"
        )
    },
    @{
        Path = "docs/vtrace/VERIFICATION.md"
        Needles = @(
            "public finding release boundary",
            "PACKET-PF-05",
            "full-panel release review"
        )
    },
    @{
        Path = ".pitfall/packet-pitfalls.md"
        Needles = @(
            "## PACKET-PF-05",
            "**Status:** MITIGATED",
            "tests/check-public-finding-boundary.ps1"
        )
    },
    @{
        Path = ".pitfall/packet-invariants.md"
        Needles = @(
            "## PACKET-I-06",
            "Public Findings Require Release Evidence",
            "PACKET-PF-05"
        )
    },
    @{
        Path = ".roles/ROLE.md"
        Needles = @(
            "## PITFALL gates",
            '`PACKET-PF-05` public finding release',
            "Network / Broadband Planner; Network Engineer; Reliability & Operations Engineer; Telecom Economist; Digital-Equity Advocate; Resilience & Security Engineer; Incumbent-ISP & Right-of-Way Realist; Citation Auditor; Scope Keeper; Numeracy Checker"
        )
    }
)

foreach ($check in $checks) {
    $path = Join-Path $root $check.Path
    if (-not (Test-Path -Path $path)) {
        throw "Missing required file: $($check.Path)"
    }
    foreach ($needle in $check.Needles) {
        Assert-Contains -Path $path -Needle $needle
    }
}

Write-Output "PACKET public finding boundary check passed."
