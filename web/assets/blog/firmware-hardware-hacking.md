# From Silicon to Shell: Hardware & Firmware Hacking Fundamentals

Before software vulnerabilities, before SQL injections, there's hardware. Physical access to a device often bypasses every software security control — and knowing how to exploit that boundary is one of the most powerful skills in a security researcher's toolkit. Let's walk through the fundamentals.

## Physical Debug Interfaces

Modern embedded systems almost universally expose debug interfaces during development. Manufacturers often forget (or choose not) to remove them from production hardware.

### UART (Universal Asynchronous Receiver-Transmitter)
UART is a simple two-wire serial protocol (TX/RX). It's everywhere — routers, IoT devices, embedded controllers. Finding it:
1. Look for a row of 3-4 unpopulated solder pads or through-holes on the PCB
2. Use a multimeter in continuity mode to find GND
3. Look for ~3.3V or 5V on a pin at startup — that's VCC
4. The remaining pins are TX and RX
5. Use a logic analyzer or oscilloscope to identify the baud rate (typically 115200)

Connect with a USB-UART adapter (CP2102, CH340) and `minicom` or `screen /dev/ttyUSB0 115200`. You'll often drop straight into a root shell or boot log.

### JTAG (Joint Test Action Group)
JTAG is a boundary-scan protocol that gives you full hardware debug access: read/write registers, halt the CPU, set breakpoints, dump RAM. Pins: TDI, TDO, TCK, TMS, and optionally TRST/RTCK.

Tools: OpenOCD with a JTAG adapter (J-Link, FT2232H-based). Finding JTAG: JTAGulator, or pattern-matching with a logic analyzer.

### SPI and I2C
SPI flash chips store firmware. I2C appears on EEPROMs holding configuration data. Both are readable with cheap hardware:
- **flashrom** + a CH341A programmer reads most SPI NOR flash chips
- An SOIC-8 clip lets you read chips without desoldering

## Firmware Extraction

```bash
# Identify the flash chip marking (e.g., Winbond W25Q128)
# Connect SOIC clip to CH341A programmer

# Dump with flashrom
flashrom -p ch341a_spi -r firmware.bin

# Analyze with binwalk
binwalk firmware.bin

# Extract filesystem
binwalk -e firmware.bin
```

Binwalk detects compressed data (gzip, lzma), filesystems (squashfs, cramfs, ext2), kernel images, and more. After extraction you'll often find a full Linux root filesystem.

## Initial Analysis

Once you have the firmware binary:

```bash
# Get file type overview
file firmware.bin

# Look for strings
strings firmware.bin | grep -i password
strings firmware.bin | grep -i "admin"
strings firmware.bin | grep -E "([0-9]{1,3}\.){3}[0-9]{1,3}"  # IP addresses

# Entropy analysis (high entropy = encrypted/compressed)
binwalk -E firmware.bin

# Check for ELF binaries inside
find _firmware.bin.extracted/ -name "*.elf" -o -name "busybox"
```

Hardcoded credentials are embarrassingly common. Telnet backdoors, default SSH keys, hard-coded API tokens — real CVEs have been filed for all of these found via `strings`.

## Reverse Engineering with Ghidra

For compiled ARM/MIPS binaries inside the firmware:

1. Open the binary in Ghidra (NSA's free decompiler — it's excellent)
2. Let auto-analysis run
3. Search for interesting strings → cross-reference to functions
4. Look at authentication routines — how is the admin password validated?
5. Find `strcmp` calls — are passwords being compared in constant time? (timing attacks)

A common find: a function like:
```c
int check_password(char *input) {
    return strcmp(input, "sup3r_s3cr3t_2019");  // hardcoded!
}
```

Ghidra's decompiler turns MIPS assembly back into readable C. Combined with the symbol names often left in embedded Linux binaries, you can reconstruct logic quickly.

## Custom Firmware and Flashing

Once you understand the firmware structure, modifications are straightforward:

1. Extract the squashfs filesystem: `unsquashfs squashfs-root.bin`
2. Modify files (add SSH keys, change init scripts, patch binaries)
3. Repack: `mksquashfs squashfs-root/ new-squashfs.bin -comp lzma`
4. Reassemble the firmware image (preserve headers/checksums)
5. Flash back with flashrom or the device's own update mechanism

Many devices have signature verification on their official update path but not on direct flash access — physical access wins again.

## Real-World Mindset: Generic Router

A typical consumer router investigation:
1. Open the case, photograph the PCB
2. Identify the SoC (MediaTek MT7621, Broadcom BCM4908, etc.)
3. Find and tap UART for a shell during boot
4. Dump flash via SPI
5. Extract with binwalk, find squashfs root
6. Grep for credentials, analyze the web interface CGI binaries
7. Look for command injection in web interface parameters

CVE-2022-27255, CVE-2021-20090, CVE-2019-7192 — countless router CVEs have been found exactly this way.

## The Bigger Picture

Hardware hacking teaches you:
- **Trust boundaries**: hardware > OS > application. Physical access breaks the chain.
- **Defense in depth**: signed firmware, encrypted storage, fuse-blown debug interfaces, secure boot
- **Embedded systems design**: resource constraints force interesting security trade-offs
- **Real-world impact**: IoT devices rarely get patched. A router vulnerability from 2015 may still run in millions of homes.

The skills here — reading datasheets, using oscilloscopes, reverse engineering binaries — form a foundation that makes everything else in systems programming and security make more sense.
