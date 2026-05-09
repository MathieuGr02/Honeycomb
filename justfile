KERNEL := "vmlinux.bin"
VOLUME := "ubuntu-24-04.ext4"
MOUNT_VOLUME := "fs/firecracker"

stopfc:
    echo "Stopping firecracket socket"

    if {{ path_exists("/tmp/firecracker.socket") }} ; then rm "/tmp/firecracker.socket"; fi

startfc:
    echo "Starting firecracker socket"

    just stopfc

    ./firecracker/firecracker --api-sock /tmp/firecracker.socket

buildvm:
    echo "Building VM"

    echo "Mounting VM volume: {{ VOLUME }}"

    mkdir -p {{ MOUNT_VOLUME }}
    sudo mount {{ VOLUME }} {{ MOUNT_VOLUME }}

    echo "Building cargo release"
    cargo build --release

    echo "Copying data into VM volume"
    sudo cp ./target/release/Honeycomb ./{{ MOUNT_VOLUME }}/usr/local/bin/Honeycomb
    sudo cp -r ./fs/testfs ./{{ MOUNT_VOLUME }}/usr/local/bin/fs

    echo "Unmounting VM volume: {{ VOLUME }}"
    sudo umount {{ MOUNT_VOLUME }}

    echo "Finished building VM"

runvm:
    echo "Running VM"

    curl --unix-socket /tmp/firecracker.socket -i \
        -X PUT 'http://localhost/boot-source'   \
        -H 'Accept: application/json'           \
        -H 'Content-Type: application/json'     \
        -d "{\"kernel_image_path\": \"$(pwd)/{{ KERNEL }}\", \"boot_args\": \"console=ttyS0 reboot=k panic=1 pci=off rw init=/usr/local/bin/Honeycomb run\" }"

    curl --unix-socket /tmp/firecracker.socket -i \
        -X PUT 'http://localhost/drives/rootfs' \
        -H 'Accept: application/json'           \
        -H 'Content-Type: application/json'     \
        -d "{\"drive_id\": \"rootfs\", \"path_on_host\": \"$(pwd)/{{ VOLUME }}\", \"is_root_device\": true, \"is_read_only\": false }"

    curl --unix-socket /tmp/firecracker.socket -i \
      -X PUT 'http://localhost/actions'       \
      -H  'Accept: application/json'          \
      -H  'Content-Type: application/json'    \
      -d '{ "action_type": "InstanceStart" }'

testvm:
    echo "Testing VM"

    just buildvm
    just runvm
