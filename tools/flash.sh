#!/usr/bin/env bash

set -eux

artifact="$(basename $1)"
rust_target_folder="$(greadlink -f $(dirname $1)/../..)"

case "${PLATFORM}" in
    "nrf52"|"nrf52840")
        tockloader_flags="--jlink --arch cortex-m4 --board nrf52dk --jtag-device nrf52"
        binary_name=cortex-m4.elf
        tockload=y
        ;;
    "hail")
        tockloader_flags=""
        binary_name=cortex-m4.elf
        tockload=y
        ;;
    "hifive1")
        tockloader_flags=""
        binary_name=rv32imac.elf
        tockload=n
        ;;
    "opentitan")
        tockloader_flags=""
        binary_name=rv32imc.elf
        tockload=n
        ;;
    *)
        echo "Unknown platform \"${PLATFORM}\""
        exit 1
        ;;
esac

libtock_target_path="${rust_target_folder}/tab/${PLATFORM}/${artifact}"
elf_file_name="${libtock_target_path}/${binary_name}"
tab_file_name="${libtock_target_path}.tab"

mkdir -p "${libtock_target_path}"
cp "$1" "${elf_file_name}"

elf2tab -n "${artifact}" -o "${tab_file_name}" "${elf_file_name}" --stack 2048 --app-heap 1024 --kernel-heap 1024 --protected-region-size=64

if [ $tockload == "n" ]; then
	echo "Skipping flashing for platform \"${PLATFORM}\""
	exit 0
fi

tockloader uninstall ${tockloader_flags} || true
tockloader install ${tockloader_flags} "${tab_file_name}"
