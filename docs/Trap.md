# Trap

## 创建栈帧结构体

当产生中断时，我们需要保存当前 **所有寄存器** 的状态，然后处理中断，最后恢复寄存器状态，继续执行之前的命令。我们需要按照特定的格式保存寄存器，以便于我们使用 **栈帧** 结构体查看或修改这些寄存器。可以理解为，在一片连续的内存空间中存放了我们寄存器的状态，我们通过这片空间的首地址（指针）来访问他们。在创建结构体之前，我们在 **Cargo.toml** 中需要引入一些依赖：

```rust
[dependencies]
riscv = { git = "crate/riscv", features = ["inline-asm"] }
```

**riscv32** 中有 32 个通用寄存器和部分特殊寄存器。在 main.rs 的同级目录下创建 **context.rs** 文件，在开头引入一些特殊寄存器：

```rust
use riscv::register::{
    sstatus::Sstatus,
    scause::Scause,
};
```

栈帧结构体的实现如下：

```rust
#[repr(C)]
pub struct TrapFrame {
    pub x: [usize; 32], // General registers
    pub sstatus: Sstatus, // Supervisor Status Register
    pub sepc: usize, // Supervisor exception program counter
    pub stval: usize, // Supervisor trap value
    pub scause: Scause, // Scause register: record the cause of exception/interrupt/trap
}
```

> `#[repr(C)]`表示不希望编译器对结构体的变量顺序做出改变等优化

理解并创建栈帧之后，我们便可以开始对中断进行处理了。

## 设置中断入口点来响应中断

在 main.rs 的同级目录下创建 **trap/trap.asm** 和 **interrupt.rs** 用于处理中断。

当我们的程序遇上中断或异常时， cpu 会跳转到一个指定的地址进行中断处理。在 RISCV 中，这个地址由 **stvec** 控制寄存器保存：

![stvec_riscv32](img/stvec_riscv32.png)

> **ebreak** 和 **ecall** 严格来说属于主动触发的异常。异常是在执行指令的过程中“同步”发生的，相对地中断则是“异步”发生的，由外部信号触发（如时钟、外设、IPI）。

stvec 中包含了 **向量基址（BASE）** 和 **向量模式（MODE）** ，其中 **向量基址（BASE）** 必须按照 4 字节对齐。

RISCV 中有两种中断入口模式：

- **直接模式（Driect）**
  MODE = 0 ，触发任何 **中断异常** 时都把 PC 设置为 BASE
- **向量模式（Vectored）**
  MODE = 1 ，对第 i 种 **中断** ，跳转到 `BASE + i * 4`；对所有 **异常** ，仍跳转到 BASE

为了实现简单，我们采用第一种模式，先进入统一的处理函数，之后再根据中断/异常种类进行不同处理。

在 **interrupt.rs** 中引入 **栈帧** 和 **stvec** ，帮助我们实现 **指定中断处理函数** 的函数：

```rust
use crate::context::TrapFrame;
use riscv::register::stvec;

#[no_mangle]
pub fn init() {
    extern {
        fn __alltraps();
    }
    unsafe {
        sscratch::write(0); // 给中断 asm 初始化
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }
    println!("++++setup interrupt !++++");
}
```

**\_\_alltraps** 便是我们的程序在遇上中断时， cpu 跳转到的地址。现在我们来实现他：

```nasm
# in trap.asm

.section .text
.globl __alltraps
__alltraps:
    SAVE_ALL
    mv a0, sp
    jal rust_trap
.globl __trapret
__trapret:
    RESTORE_ALL
    # return from supervisor call
    sret
```

**SAVE_ALL** 用于保存所有的寄存器的状态， **RESTORE_ALL** 则用于恢复所有的寄存器的状态。为了增加代码的可读性，我们使用了较多的宏。在 **main.rs** 中引入 **trap.asm** 之前，我们需要先定义使用的宏：

```nasm
# in trap/trap.asm

.equ XLENB,     4
.equ XLENb,     32
.macro LOAD a1, a2
    lw \a1, \a2*XLENB(sp)
.endm
.macro STORE a1, a2
    sw \a1, \a2*XLENB(sp)
.endm
```

有了上面定义的宏之后，我们就可以开始编写 **SAVE_ALL** 和 **RESTORE_ALL** 了。增加了这两个部分之后， **trap/trap.asm** 应该长这样：

```nasm
# Constants / Macros defined in Rust code:
#   XLENB
#   LOAD
#   STORE

.equ XLENB,     4
.equ XLENb,     32
.macro LOAD a1, a2
    lw \a1, \a2*XLENB(sp)
.endm
.macro STORE a1, a2
    sw \a1, \a2*XLENB(sp)
.endm

.macro SAVE_ALL
    # If coming from userspace, preserve the user stack pointer and load
    # the kernel stack pointer. If we came from the kernel, sscratch
    # will contain 0, and we should continue on the current stack.
    csrrw sp, sscratch, sp
    bnez sp, _save_context
_restore_kernel_sp:
    csrr sp, sscratch
    # sscratch = previous-sp, sp = kernel-sp
_save_context:
    # provide room for trap frame
    addi sp, sp, -36*XLENB
    # save x registers except x2 (sp)
    STORE x1, 1
    STORE x3, 3
    # tp(x4) = hartid. DON'T change.
    # STORE x4, 4
    STORE x5, 5
    STORE x6, 6
    STORE x7, 7
    STORE x8, 8
    STORE x9, 9
    STORE x10, 10
    STORE x11, 11
    STORE x12, 12
    STORE x13, 13
    STORE x14, 14
    STORE x15, 15
    STORE x16, 16
    STORE x17, 17
    STORE x18, 18
    STORE x19, 19
    STORE x20, 20
    STORE x21, 21
    STORE x22, 22
    STORE x23, 23
    STORE x24, 24
    STORE x25, 25
    STORE x26, 26
    STORE x27, 27
    STORE x28, 28
    STORE x29, 29
    STORE x30, 30
    STORE x31, 31

    # get sp, sstatus, sepc, stval, scause
    # set sscratch = 0
    csrrw s0, sscratch, x0
    csrr s1, sstatus
    csrr s2, sepc
    csrr s3, stval
    csrr s4, scause
    # store sp, sstatus, sepc, sbadvaddr, scause
    STORE s0, 2
    STORE s1, 32
    STORE s2, 33
    STORE s3, 34
    STORE s4, 35
.endm

.macro RESTORE_ALL
    LOAD s1, 32             # s1 = sstatus
    LOAD s2, 33             # s2 = sepc
    andi s0, s1, 1 << 8     # sstatus.SPP = 1?
    bnez s0, _to_kernel     # s0 = back to kernel?
_to_user:
    addi s0, sp, 36*XLENB
    csrw sscratch, s0         # sscratch = kernel-sp
_to_kernel:
    # restore sstatus, sepc
    csrw sstatus, s1
    csrw sepc, s2

    # restore x registers except x2 (sp)
    LOAD x1, 1
    LOAD x3, 3
    # LOAD x4, 4
    LOAD x5, 5
    LOAD x6, 6
    LOAD x7, 7
    LOAD x8, 8
    LOAD x9, 9
    LOAD x10, 10
    LOAD x11, 11
    LOAD x12, 12
    LOAD x13, 13
    LOAD x14, 14
    LOAD x15, 15
    LOAD x16, 16
    LOAD x17, 17
    LOAD x18, 18
    LOAD x19, 19
    LOAD x20, 20
    LOAD x21, 21
    LOAD x22, 22
    LOAD x23, 23
    LOAD x24, 24
    LOAD x25, 25
    LOAD x26, 26
    LOAD x27, 27
    LOAD x28, 28
    LOAD x29, 29
    LOAD x30, 30
    LOAD x31, 31
    # restore sp last
    LOAD x2, 2
.endm

.section .text
.globl __alltraps
__alltraps:
    SAVE_ALL
    mv a0, sp
    jal rust_trap
.globl __trapret
__trapret:
    RESTORE_ALL
    # return from supervisor call
    sret
```

> 由于这部分内容难度较高，所以只进行粗略的介绍。对细节感兴趣的读者可以 [点击这里](中断跳转.md)

**a0** 是 riscv32 中的参数寄存器，用于存放下一个调用的函数的参数。我们给 **a0** 赋值为 **sp** ，也就是栈帧的地址。这里调用的的函数是 **rust_trap** ：

```rust
// in interrupt.rs

global_asm!(include_str!("trap/trap.asm"));

#[no_mangle]
pub extern "C" fn rust_trap(tf: &mut TrapFrame) {
    println!("trap!");
    tf.increase_sepc();
}
```

在 riscv 中，发生中断指令的 **pc** 被存入 **sepc** 。对于大部分情况，中断处理完成后还回到这个指令继续执行。但对于用户主动触发的异常（例如`ebreak`用于触发断点，`ecall`用于系统调用），中断处理函数需要调整 **sepc** 以跳过这条指令。在 riscv 中， **一般** 每条指令都是定长的 4 字节（但如果开启 **压缩指令集** 可就不一定了，这也导致了一个大坑），因此只需将 **sepc** +4 即可，这里我们通过 **increase_sepc** 完成这个功能：

```rust
// in context.rs

impl TrapFrame {
    pub fn increase_sepc(self: &mut Self) {
        self.sepc = self.sepc + 4;
    }
}
```

**注意！！！**

这里我们强调了 **一般** 。在开启 **压缩指令集** 的情况下，对于常用指令，编译器会进行压缩，减小程序的大小。但是有时候这并不是我们希望的。比如这里因为我们要求每条指令都是精准的 **32bits** ，才能够通过 `self.sepc = self.sepc + 4` 跳转至下一条指令（否则会跳转到奇怪的地方）。在 **riscv32-os.json** 中，有一行 `"features": "+m,+a,+c"` 。默认情况下，riscv 指令集只支持加减法， **+m** 增加了乘除指令； **+a** 增加了原子操作； **+c** 增加了代码压缩。这里的压缩是我们不想要的，所以把 **+c** 删去。

至此我们简易的的中断功能已经全部实现完成，让我们设置一个中断试试：

```rust
#![feature(asm)]

mod interrupt;
mod context;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    interrupt::init();
    unsafe{
        asm!("ebreak"::::"volatile");
    }
    panic!("End of rust_main");
}
```

编译运行，屏幕显示：

```
++++setup interrupt !++++
trap!
panicked at 'End of rust_main', src/main.rs:51:5
```

可以看到，我们已经成功进入中断处理函数，并且返回到了 **rust_main** ，触发了 panic 。

## 预告

现在，我们已经实现了简易的中断机制。但是同时我们的 **main.rs** 看起来有一些乱。下一章，我们首先将调整代码结构，简化 **main.rs** ，然后实现时钟中断，并在 **rust_trap** 中区分中断类型，对他们进行不同的处理。
