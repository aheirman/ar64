echo "==== RISC-V TEST SUITE ===="

intercept-build make

# Set --target=riscv64-lp64-none-elf -fuse-ld=lld
RISCV_GCC_OPTS ?= --target=riscv64-lp64-none-elf -fuse-ld=lld -static -mcmodel=medany -fvisibility=hidden -nostdlib -nostartfiles

function run_single_test() {
    echo
    echo "==== TEST: $1 ====" 
    echo
    # go there and store current location
    pushd ./riscv-tests/isa
    make "$1"
    llvm-objcopy "$1" -O binary "$1.bin"

    # go back
    popd

    pushd ./../sim
    RUST_BACKTRACE=1 RUST_BACKTRACE=full cargo run -- -T "./../tests/riscv-tests/isa/$1.bin"
    if [ $? -gt 0 ]; then
        echo "Test failed!"
        exit $?
    fi

    popd
}

run_single_test rv64ui-p-add

echo 
echo "==== ALL TESTS PASSED ===="
echo