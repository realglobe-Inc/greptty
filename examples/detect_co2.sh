#!/bin/sh

preifx="${1}"
if [ -z "${preifx}" ]; then
  echo "spefify device path prefix as 1st argument" >&2
  exit 1
fi

greptty_result="$(printf 'co2 ^co2=[0-9][0-9]*' | greptty -b 9600 "${prefix}" | grep '^co2' | head -n 1)"
co2_baud_rate="$(echo "${greptty_result}" | cut -d ' ' -f 2)"
co2_device="$(echo "${greptty_result}" | cut -d ' ' -f 3)"

if [ -z "${co2_device}" ]; then
  echo "No device found" >&2
  exit 1
fi

echo "CO2 device found: ${co2_device}" >&2
echo "baud rate: ${co2_baud_rate}" >&2

uname_s="$(uname -s)"
if [ "${uname_s}" = "Darwin" ]; then
  stty -f "${co2_device}" raw "${co2_baud_rate}"
elif [ "${uname_s}" = "Linux" ]; then
  stty -F "${co2_device}" raw "${co2_baud_rate}"
fi

cat "${co2_device}"
