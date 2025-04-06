
# Open Tools

## riscv_config
https://riscv-config.readthedocs.io/en/stable/quickstart.html

source .~/.profile
`clear && riscv-config -ispec ar64_isa.yaml -pspec ar64_platform.yaml`

## RISCOF
https://riscof.readthedocs.io/en/stable/arch-tests.html


`pip3 install git+https://github.com/riscv/riscof.git`

riscof

https://twilco.github.io/riscv-from-scratch/2019/07/28/riscv-from-scratch-4.html

# TODO

riscv-tests https://riscv.org/riscv-tests/
device tree
linux

GPU

# riscv
https://msyksphinz-self.github.io/riscv-isadoc/html/rv64i.html
https://msyksphinz-self.github.io/riscv-isadoc/html/rvi.html#addi


https://www.youtube.com/watch?v=dOfucXtyEsU

# interrupts (core local interrupts: CLINT)
https://chromitem-soc.readthedocs.io/en/latest/clint.html

# device tree
https://elinux.org/Device_Tree_Usage#chosen_Node

# uart
https://www.lammertbies.nl/comm/info/serial-uart
https://twilco.github.io/riscv-from-scratch/2019/04/27/riscv-from-scratch-2.html
https://osblog.stephenmarz.com/ch2.html


# linking
https://wiki.osdev.org/Linker_Scripts
https://mcyoung.xyz/2021/06/01/linker-script/#object-files-and-sections
https://blog.thea.codes/the-most-thoroughly-commented-linker-script/
https://en.wikipedia.org/wiki/Crt0


## Website to decode an instruction
Note the ISA can have either a little-, big- or bi-endian memory system.
The instructions parcels have a little-endian encoding independent of the memory endianness.

https://riscvasm.lucasteske.dev/#

## Backburner
https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md#2-if-t-static-then-t-must-be-valid-for-the-entire-program

# Archaic WEBASM
https://www.secondstate.io/articles/wasi-access-system-resources/
https://web.dev/read-files/

https://github.com/WICG/file-system-access/issues/139
https://wicg.github.io/file-system-access/#api-getsystemdirectory
https://developer.mozilla.org/en-US/docs/Web/API/File_System_Access_API
https://fs.spec.whatwg.org/
