param([Parameter(Mandatory=$true)][string]$FFmpeg)
$ErrorActionPreference = 'Stop'
$repoRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '../../../..'))
$fixtureRoot = $PSScriptRoot
function Run-FFmpeg([string[]]$Arguments) {
    & $FFmpeg -hide_banner -loglevel error -y @Arguments
    if ($LASTEXITCODE -ne 0) { throw "FFmpeg failed: $Arguments" }
}
function Write-Source([string]$Name, [int]$Bits, [int]$Part, [bool]$Tone, [int]$FrameCount = 4417) {
    $path = Join-Path $fixtureRoot $Name
    $writer = [IO.BinaryWriter]::new([IO.File]::Create($path))
    try {
        for ($i = 0; $i -lt $FrameCount; $i++) {
            for ($channel = 0; $channel -lt 2; $channel++) {
                $n = $i + $Part * 4417
                if ($Tone) {
                    $value = if ($i -lt 64 -or $i -ge ($FrameCount - 64)) { 0 } else { [int][Math]::Round(536870911 * [Math]::Sin(2 * [Math]::PI * (997 + $channel * 337) * $n / 44100)) }
                } else {
                    $value = [int]((([long]$n * 15485863 + $channel * 32452843) % 4294967296) - 2147483648)
                    if ($i -lt 4) { $value = @([int]::MinValue, [int]::MaxValue, -1, 1)[$i] }
                }
                if ($Bits -eq 24) { $value = ($value -shr 8) -shl 8 }
                $writer.Write([int]$value)
            }
        }
    } finally { $writer.Dispose() }
}
$entries = [Collections.Generic.List[object]]::new()
foreach ($part in 0,1) {
    $letter = @('a','b')[$part]
    foreach ($bits in 24,32) {
        $source = "pcm$bits-$letter.s32le"
        Write-Source $source $bits $part $false
        foreach ($format in 'wav','flac') {
            # This reference encoder stores FLAC s32 input as 24 significant bits.
            # PCM32 is proved in WAV; do not label a lossy 32->24 conversion lossless.
            if ($bits -eq 32 -and $format -eq 'flac') { continue }
            $name = "pcm$bits-$letter.$format"
            $codec = if ($format -eq 'wav') { "pcm_s${bits}le" } else { 'flac' }
            $extra = if ($format -eq 'flac') { @('-sample_fmt','s32','-bits_per_raw_sample',"$bits") } else { @() }
            Run-FFmpeg (@('-f','s32le','-ar','44100','-ac','2','-i',(Join-Path $fixtureRoot $source),'-c:a',$codec) + $extra + @((Join-Path $fixtureRoot $name)))
            $reference = "$name.ref.s32le"
            Run-FFmpeg @('-i',(Join-Path $fixtureRoot $name),'-f','s32le','-c:a','pcm_s32le',(Join-Path $fixtureRoot $reference))
            if ((Get-FileHash (Join-Path $fixtureRoot $reference)).Hash -ne (Get-FileHash (Join-Path $fixtureRoot $source)).Hash) { throw "Lossless mismatch: $name" }
            $entries.Add(@{ id="decode-$name"; path="tests/fixtures/audio/decode/$name"; reference="tests/fixtures/audio/decode/$reference"; source="tests/fixtures/audio/decode/$source"; format=$format; bits=$bits; sample_type='i32-msb'; frames=4417; rate=44100; channels=2; max_abs_error=0 })
        }
    }
    $source = "tone-$letter.s32le"
    Write-Source $source 32 $part $true
    foreach ($mode in 'cbr','vbr') {
        $name = "tone-$mode-$letter.mp3"
        $extra = if ($mode -eq 'cbr') { @('-b:a','192k') } else { @('-q:a','2') }
        Run-FFmpeg (@('-f','s32le','-ar','44100','-ac','2','-i',(Join-Path $fixtureRoot $source),'-c:a','libmp3lame','-write_xing','1') + $extra + @((Join-Path $fixtureRoot $name)))
        $reference = "$name.ref.f32le"
        Run-FFmpeg @('-c:a','mp3float','-i',(Join-Path $fixtureRoot $name),'-f','f32le','-c:a','pcm_f32le',(Join-Path $fixtureRoot $reference))
        if ((Get-Item (Join-Path $fixtureRoot $reference)).Length -ne 4417 * 2 * 4) { throw "MP3 frame mismatch: $name" }
        $entries.Add(@{ id="decode-$name"; path="tests/fixtures/audio/decode/$name"; reference="tests/fixtures/audio/decode/$reference"; source="tests/fixtures/audio/decode/$source"; format='mp3'; profile=$mode; sample_type='f32'; frames=4417; rate=44100; channels=2; max_abs_error=0.00002 })
    }
}
$files = @{}
# Untagged MP3 provides a negative trim oracle: no invented silence removal.
$name = 'tone-no-xing.mp3'
Run-FFmpeg @('-f','s32le','-ar','44100','-ac','2','-i',(Join-Path $fixtureRoot 'tone-a.s32le'),'-c:a','libmp3lame','-b:a','192k','-write_xing','0',(Join-Path $fixtureRoot $name))
$reference = "$name.ref.f32le"
Run-FFmpeg @('-c:a','mp3float','-i',(Join-Path $fixtureRoot $name),'-f','f32le','-c:a','pcm_f32le',(Join-Path $fixtureRoot $reference))
$entries.Add(@{ id="decode-$name"; path="tests/fixtures/audio/decode/$name"; reference="tests/fixtures/audio/decode/$reference"; source='tests/fixtures/audio/decode/tone-a.s32le'; format='mp3'; profile='CBR without Xing/LAME'; sample_type='f32'; frames=[int]((Get-Item (Join-Path $fixtureRoot $reference)).Length / 8); rate=44100; channels=2; max_abs_error=0.00002; trim_unknown=$true; expected_error='Unsupported' })
Write-Source 'tone-long.s32le' 32 0 $true 22050
$name = 'tone-vbr-long-no-xing.mp3'
Run-FFmpeg @('-f','s32le','-ar','44100','-ac','2','-i',(Join-Path $fixtureRoot 'tone-long.s32le'),'-c:a','libmp3lame','-q:a','2','-write_xing','0',(Join-Path $fixtureRoot $name))
$reference = "$name.ref.f32le"
Run-FFmpeg @('-c:a','mp3float','-i',(Join-Path $fixtureRoot $name),'-f','f32le','-c:a','pcm_f32le',(Join-Path $fixtureRoot $reference))
$entries.Add(@{ id="decode-$name"; path="tests/fixtures/audio/decode/$name"; reference="tests/fixtures/audio/decode/$reference"; source='tests/fixtures/audio/decode/tone-long.s32le'; format='mp3'; profile='VBR without Xing/LAME > 17 MPEG frames'; sample_type='f32'; frames=[int]((Get-Item (Join-Path $fixtureRoot $reference)).Length / 8); rate=44100; channels=2; max_abs_error=0.00002; trim_unknown=$true; expected_error='Unsupported' })
foreach ($entry in $entries) {
    foreach ($key in 'path','reference','source') {
        $relative = $entry[$key]
        $file = Join-Path $repoRoot $relative
        $files[$relative] = @{ sha256=(Get-FileHash $file -Algorithm SHA256).Hash.ToLowerInvariant(); bytes=(Get-Item $file).Length }
    }
}
$manifest = [ordered]@{
    schema_version=1; task='TASK-E01-01'; requirement='REQ-E01-01'; qa=@('QA-001','QA-002','QA-004');
    origin='Synthetic integer sequence and sine signals generated by generate.ps1; no third-party audio';
    permission='Project-generated test data; may be redistributed and transformed with the tests. No final project source license selected.';
    generator='PowerShell 7 generate.ps1; 4417 stereo frames per part, 44100 Hz; explicit little-endian i32';
    ffmpeg_version=(@(& $FFmpeg -version) -join "`n"); ffmpeg_sha256=(Get-FileHash $FFmpeg -Algorithm SHA256).Hash.ToLowerInvariant();
    command='./tests/fixtures/audio/decode/generate.ps1 -FFmpeg <path-to-reference-ffmpeg>';
    oracle='Lossless FFmpeg decoded s32le must equal independently generated source bytes; MP3 FFmpeg mp3float with Xing/LAME trim, exact frame count, max absolute error <= 2e-5 FS against candidate.';
    files=$files; fixtures=@($entries)
}
$manifestDir = Join-Path $repoRoot 'tests/fixtures/manifests'
New-Item -ItemType Directory -Path $manifestDir -Force | Out-Null
[IO.File]::WriteAllText((Join-Path $manifestDir 'decode.json'), ($manifest | ConvertTo-Json -Depth 10) + "`n", [Text.UTF8Encoding]::new($false))
Write-Output "Generated $($entries.Count) encoded fixtures and independent references."
