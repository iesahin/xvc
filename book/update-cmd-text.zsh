#!/usr/bin/env zsh

HERE="$( cd "$(dirname "$0")" ; pwd -P )"

XVC="${HERE}/../target/debug/xvc"

OUT="${HERE}/src/ref/"

$XVC --help > ${OUT}/xvc.txt

for cmd in $(echo "check-ignore file init pipeline root storage aliases") ; do
    $XVC $cmd --help > ${OUT}/xvc-${cmd}.txt
done


for filecmd in checkout hash list track pull push fetch ; do
    $XVC file $filecmd --help > ${OUT}/xvc-file-${filecmd}.txt
done


for pipecmd in dag delete export import list new run step update ; do
    $XVC pipeline $pipecmd --help > ${OUT}/xvc-pipeline-${pipecmd}.txt
done

for pipestepcmd in dependency new output show update ; do
    $XVC pipeline step $pipestepcmd --help > ${OUT}/xvc-pipeline-step-${pipestepcmd}.txt
done

for storagecmd in list new remove ; do
    $XVC storage $storagecmd --help  > ${OUT}/xvc-storage-${storagecmd}.txt
done

for storagenewcmd in $(echo "local generic s3 r2 minio gcs wasabi digital-ocean") ; do
    $XVC storage new $storagenewcmd --help > ${OUT}/xvc-storage-new-${storagenewcmd}.txt
done
