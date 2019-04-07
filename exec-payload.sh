#!/bin/sh

usage()
{
	cat << EOF
Usage: sudo ./exec-payload.sh <payload_dir>
Where:
	payload_dir: Valid target payload directory containing bootloader.conf.
EOF

	exit 1
}

if [ $# -lt 1 -o $# -gt 2 ]; then
	usage
fi

# If the user is not root, there is no point in going forward
if [ "${USER}" != "root" ]; then
	echo "exec-payload.sh requires root privilege"
	exit 1
fi

payload_dir="$1"

if [ ! -r "${payload_dir}/bootloader.conf" ]; then
	echo "Error: bootloader.conf missing in \"${payload_dir}\"."
	usage
fi

source "${payload_dir}/bootloader.conf"

if [ -z "${LDK_DIR}" ]; then
    echo "Please set LDK_DIR environment variable to the driver package."
    exit 1
fi;

BL_DIR="${LDK_DIR}/bootloader"
KERNEL_DIR="${LDK_DIR}/kernel"
DTB_DIR="${KERNEL_DIR}/dtb"

tmp_dir="/tmp/exec-payload.$$"
fn_img="${tmp_dir}/image.bin"
fn_addr="${tmp_dir}/load-addr.txt"
flashapp=tegraflash.py
instance=

function rm_tmp_dir {
	rm -rf "${tmp_dir}"
}

trap rm_tmp_dir exit SIGHUP SIGINT SIGTERM

function chkerr {
	ret=$?
	if [ ${ret} -ne 0 ]; then
		echo $1
		exit ${ret}
	fi
}

mkdir -p "${tmp_dir}"
chkerr "Could not create temporary dir"

if [ -z "${BOOTLOADER_ENTRY}" ]; then
	BOOTLOADER_ENTRY=`"${LDK_DIR}/elf-get-entry.py" "${BOOTLOADER_ELF}"`
	chkerr "Could not determine entry point of bootloader binary"
fi;

"${BL_DIR}/gen-tboot-img.py" "${BOOTLOADER_BIN}" ${BOOTLOADER_ENTRY} \
	"${fn_img}" "${fn_addr}"
chkerr "Could not add TBOOT header to bootloader binary"

fake_pt="${tmp_dir}/fake-pt.xml"
cat > "${fake_pt}" <<ENDOFHERE
<?xml version="1.0"?>
<partition_layout version="01.00.0000">
    <device type="sdmmc" instance="3">
        <partition name="WB0" id="10" type="WB0">
            <allocation_policy> sequential </allocation_policy>
            <filesystem_type> basic </filesystem_type>
            <size> 6291456 </size>
            <file_system_attribute> 0 </file_system_attribute>
            <allocation_attribute> 8 </allocation_attribute>
            <percent_reserved> 0 </percent_reserved>
            <filename> ${LDK_DIR}/${WB0BOOT} </filename>
        </partition>
    </device>
</partition_layout>
ENDOFHERE
chkerr "Could not create fake partition table file"

echo "${BL_DIR}/${target_board}/BCT/${EMMC_BCT}"

"${BL_DIR}/${flashapp}" \
	${instance} \
	--chip 0x21 \
	--cfg "${fake_pt}" \
	--applet "${LDK_DIR}/${SOSFILE}" \
	--bct "${BL_DIR}/${target_board}/BCT/${EMMC_BCT}" \
	--odmdata ${ODMDATA} \
	--boardconfig "${LDK_DIR}/${BCFFILE}" \
	--bldtb "${DTB_DIR}/${DTB_FILE}" \
	--kerneldtb "${DTB_DIR}/${DTB_FILE}" \
	--applet-cpu "${LDK_DIR}/${TBCFILE}" \
	--tos "${TOSFILE}" \
	--bl "${fn_img}" \
	--bl-load `cat "${fn_addr}"` \
	--wb "${LDK_DIR}/${WB0BOOT}" \
	--bpf "${LDK_DIR}/${BPFFILE}" \
    --out "${tmp_dir}" \
	--cmd rcmbl
    
chkerr "Bootloader download failed"

# vi: ts=8 sw=8 noexpandtab
