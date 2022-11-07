# This script sets correct environment variables for PowerShell in TC219 machine.
#
# How to run?
# `. ./scripts/tc219.ps1`

$Env:PATH_GNU_BIN = "C:\\Apps\\Xilinx_Vivado2017\\SDK\\2017.2\\gnu\\aarch32\\nt\\gcc-arm-none-eabi\\bin\\"
$Env:PATH_GNU_INC = "C:\\Apps\\Xilinx_Vivado2017\\SDK\\2017.2\\gnu\\aarch32\\nt\\gcc-arm-none-eabi\\arm-none-eabi\\libc\\usr\\include\\"
$Env:PATH_GNU_LIB = "C:\\Apps\\Xilinx_Vivado2017\\SDK\\2017.2\\gnu\\aarch32\\nt\\gcc-arm-none-eabi\\arm-none-eabi\\libc\\usr\\lib\\"