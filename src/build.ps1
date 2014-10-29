function BuildWith-Rustdoc($Src, $Dst)
{
    $out = rustdoc "$Src" --test
    if($LastExitCode -ne 0) {
        $out | Write-Host
        Write-Host "$Src failed tests." -ForegroundColor Red
        return
    }

    if(-not (Test-Path .tmp)) { mkdir .tmp > $null }

    rustdoc "$Src" -o .tmp
    if($LastExitCode -ne 0) {
        Write-Host "$Src failed to build." -ForegroundColor Red
        if(Test-Path .tmp) { Remove-Item .tmp -Recurse }
        return
    }

    Move-Item ".tmp\$($Dst.BaseName).html" $Dst -Force

    if(Test-Path .tmp) { Remove-Item .tmp -Recurse }
}

function Get-Target(
    [Parameter(Mandatory=$true)][String]$Path,
    [Parameter(Mandatory=$true)][ScriptBlock]$Action
)
{
    $src = (Get-Item $Path)
    $src_rel = ([IO.FileInfo][String](Resolve-Path $Path -Relative))
    $ext = $src.Extension
    $src_rel_str = [String]$src_rel
    $src_no_ext = ($src_rel_str).SubString(0, $src_rel_str.Length - $ext.Length)
    switch($ext)
    {
        ".md" {
            $dst = "..\$src_no_ext.html"
            $cmd = "BuildWith-Rustdoc"
        }
        default {
            throw "Can't determine target for $src"
        }
    }

    $dst_fi = [IO.FileInfo]$dst

    if(Test-Path $dst_fi)
    {
        $dst_fi = gi $dst_fi
    }

    $wrapped = {
        Param($Src, $Dst, $Cmd)
        $Action.Invoke($Src, $Dst, $Cmd)
    }

    $wrapped.Invoke($src, $dst_fi, $cmd)
}

ls *.md `
    | where {(gc $_ | select -first 1).Trim().StartsWith("%")} `
    | where { Get-Target $_ { $Src.LastWriteTimeUtc -gt $Dst.LastWriteTimeUtc } } `
    | foreach `
{
    Get-Target $_ {
        Write-Host " + $Dst"
        &$Cmd $Src $Dst
    }
}
