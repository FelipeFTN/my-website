# Cheating the Stack: A Developer's Guide to Game Memory Hacking

Memory hacking — the art of reading and writing another process's memory at runtime — sits at a fascinating intersection of systems programming, reverse engineering, and security research. Whether you're building a cheat for a single-player game or studying how anti-cheat systems work, the underlying concepts are the same. Let's dig in.

## What Does Process Memory Look Like?

Every process on a modern OS runs in its own virtual address space. On a 64-bit Linux system, a process gets a 128 TiB address space (theoretically). This space is divided into regions mapped by the kernel:

- **Text segment**: the executable code (read + execute)
- **Data segment**: initialized globals and statics
- **BSS segment**: uninitialized globals (zero-filled at load)
- **Heap**: dynamically allocated memory (grows upward)
- **Stack**: call frames, local variables, return addresses (grows downward)
- **Memory-mapped files**: shared libraries, mmap'd files

When a game stores your health as an integer, that integer lives somewhere in one of these regions — usually the heap (if it's part of a dynamically allocated game object) or the data segment (if it's a global).

## How Game Values Are Stored

Simple values like health, ammo, or gold are typically stored as `int32_t` or `float` in C/C++ game engines. The challenge is *finding* them. A 4GB process has over a billion possible 4-byte addresses. That's where memory scanning comes in.

The key insight: if your health is 100, search for all addresses containing the value `100`. Take a hit, now your health is 87 — search again for `87`. Keep narrowing until you have one or a handful of addresses.

## How Cheat Engine Works (Conceptually)

Cheat Engine and similar tools:

1. **Attach** to the target process via OS APIs
2. **Scan** all readable memory regions for a value
3. **Filter** down candidates as the value changes
4. **Read/write** the address once found

On Linux, this uses `process_vm_readv` / `process_vm_writev`. On Windows, it's `ReadProcessMemory` / `WriteProcessMemory`.

## Writing a Basic Memory Scanner in C

Here's a minimal Linux memory scanner. It reads `/proc/<pid>/maps` to find readable/writable regions, then scans each one for your target value:

```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

/* Read a region of the target process's memory via /proc/<pid>/mem.
   Returns number of bytes actually read (may be less than size). */
static size_t read_region(FILE *mem, uint64_t start, size_t size, uint8_t *buf) {
    if (fseek(mem, (long)start, SEEK_SET) != 0)
        return 0;
    return fread(buf, 1, size, mem);
}

/* Scan a buffer for every occurrence of a 4-byte integer value.
   Prints the virtual address of each match. */
static void scan_buffer(uint64_t base, const uint8_t *buf, size_t len, int32_t target) {
    uint8_t needle[4];
    /* Store target as little-endian bytes (x86/x64 native order) */
    memcpy(needle, &target, 4);

    for (size_t i = 0; i + 4 <= len; i++) {
        if (memcmp(&buf[i], needle, 4) == 0) {
            printf("  found at 0x%lx\n", base + i);
        }
    }
}

int main(int argc, char *argv[]) {
    if (argc != 3) {
        fprintf(stderr, "usage: scanner <pid> <value>\n");
        return 1;
    }

    int pid    = atoi(argv[1]);
    int32_t target = (int32_t)atoi(argv[2]);

    /* Open /proc/<pid>/maps to enumerate memory regions */
    char maps_path[64], mem_path[64];
    snprintf(maps_path, sizeof(maps_path), "/proc/%d/maps", pid);
    snprintf(mem_path,  sizeof(mem_path),  "/proc/%d/mem",  pid);

    FILE *maps = fopen(maps_path, "r");
    FILE *mem  = fopen(mem_path,  "rb");
    if (!maps || !mem) { perror("fopen"); return 1; }

    char line[256];
    while (fgets(line, sizeof(line), maps)) {
        uint64_t start, end;
        char perms[8];

        /* Each line looks like: 7f1234560000-7f1234580000 rw-p ... */
        if (sscanf(line, "%lx-%lx %7s", &start, &end, perms) != 3)
            continue;

        /* Only scan regions we can read and write to */
        if (perms[0] != 'r' || perms[1] != 'w')
            continue;

        size_t size = (size_t)(end - start);
        uint8_t *buf = malloc(size);
        if (!buf) continue;

        size_t got = read_region(mem, start, size, buf);
        if (got > 0)
            scan_buffer(start, buf, got, target);

        free(buf);
    }

    fclose(maps);
    fclose(mem);
    return 0;
}
```

Compile and run as root (or with `ptrace` capability):

```bash
gcc -O2 -o scanner scanner.c
sudo ./scanner <pid> 100
```

## Pointer Chains and Stable Addresses

The addresses you find via scanning are often *dynamic* — they change every time the game restarts because heap allocations are non-deterministic. The solution: **pointer chains**.

A pointer chain starts at a **static** address (in the `.data` or `.bss` segment) that holds a pointer. That pointer points to an object, which may have another pointer at some offset, eventually leading to your target value.

```
static_base + 0x10 → object_ptr
object_ptr + 0x40 → nested_ptr
nested_ptr + 0x8  → health_value
```

To find chains: tools like Cheat Engine's pointer scanner do a backwards BFS from your target address through all known pointers. You can implement this yourself by building a reverse pointer map — for every address A containing value V, record `reverse_map[V] = A`.

## Anti-Cheat Countermeasures

Modern games take this seriously:

- **Kernel-level drivers** (EasyAntiCheat, BattlEye): run in ring 0, monitor memory access patterns, detect injected DLLs, hook syscalls
- **Hypervisor-based** (some AAA titles): run the game under a thin hypervisor, trap memory access at the EPT/SLAT level
- **Value obfuscation**: store health as `real_value XOR random_key`, so scanning for `100` fails
- **Remote attestation**: compare checksums of game memory regions against known-good values server-side
- **TruePlay / VGC**: Windows kernel features for memory integrity

## Why This Makes You a Better Systems Programmer

Understanding memory hacking forces you to deeply understand:

- **Virtual memory** — pages, page tables, the TLB, ASLR
- **ELF/PE formats** — where sections live, how the linker resolves symbols
- **C++ object layout** — vtables, member offsets, RTTI
- **OS APIs** — ptrace, /proc filesystem, Windows debug APIs
- **Concurrency** — games run on multiple threads; you're reading memory concurrently

This knowledge pays dividends when you're writing a memory allocator, debugging a segfault, optimizing cache behavior, or building security-sensitive software. The best way to understand how systems work is to try to break them.
