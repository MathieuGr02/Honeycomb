#!/bin/bash

sudo mount ubuntu-24-04.ext4 ./vm_fs

cargo build --release

sudo cp ./target/release/Honeycomb ./vm_fs/usr/local/bin/Honeycomb
sudo cp ./fs/testfs ./vm_fs/usr/local/bin/fs

sudo umount ./vm_fs

kernel_image_path=$(pwd)"/vmlinux.bin"
curl --unix-socket /tmp/firecracker.socket -i \
    -X PUT 'http://localhost/boot-source'   \
    -H 'Accept: application/json'           \
    -H 'Content-Type: application/json'     \
    -d "{
        \"kernel_image_path\": \"${kernel_image_path}\",
        \"boot_args\": \"console=ttyS0 reboot=k panic=1 pci=off rw init=/usr/local/bin/Honeycomb run\"
    }"

rootfs_path=$(pwd)"/ubuntu-24-04.ext4"
curl --unix-socket /tmp/firecracker.socket -i \
    -X PUT 'http://localhost/drives/rootfs' \
    -H 'Accept: application/json'           \
    -H 'Content-Type: application/json'     \
    -d "{
        \"drive_id\": \"rootfs\",
        \"path_on_host\": \"${rootfs_path}\",
        \"is_root_device\": true,
        \"is_read_only\": false
    }"

curl --unix-socket /tmp/firecracker.socket -i \
  -X PUT 'http://localhost/actions'       \
  -H  'Accept: application/json'          \
  -H  'Content-Type: application/json'    \
  -d '{
      "action_type": "InstanceStart"
   }'
