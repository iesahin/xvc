#!/usr/bin/env zsh

HERE="$( cd "$(dirname "$0")" ; pwd -P )"

XVC="${HERE}/../target/debug/xvc"

OUT="${HERE}/src/ref/"

$XVC --help > ${OUT}/xvc.txt

for cmd in check-ignore file init pipeline root remote ; do
    $XVC $cmd --help > ${OUT}/xvc-${cmd}.txt
done


for filecmd in checkout hash list track pull push fetch ; do
    $XVC file $filecmd --help > ${OUT}/xvc-file-${filecmd}.txt
done


for pipecmd in dag delete export import list new run step update ; do
    $XVC pipeline $pipecmd --help > ${OUT}/xvc-pipeline-${pipecmd}.txt
done

for remotecmd in list new remove ; do
    $XVC remote $remotecmd --help  > ${OUT}/xvc-remote-${remotecmd}.txt
done

for remotenewcmd in local generic s3 r2 minio gcs wasabi digital-ocean  ; do
    $XVC remote new $remotenewcmd --help > ${OUT}/xvc-remote-new-${remotenewcmd}.txt
done
