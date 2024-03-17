use std::arch::asm;
fn main() {
    println!("Hello, world!");
    backtrace();
}
extern "C" {
    fn stext();
    fn etext();
}

/// Returns the current frame pointer.
#[inline(always)]
#[cfg(any(target_arch = "x86_64", target_arch = "riscv32", target_arch = "riscv64"))]
pub fn fp() -> usize {
    let ptr: usize;
    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!("mov {}, x29", out(reg) ptr);
    }
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    unsafe {
        asm!("mv {}, s0", out(reg) ptr);
    }

    ptr
}

/// Returns the current link register.
#[inline(always)]
#[cfg(any(target_arch = "x86_64", target_arch = "riscv32", target_arch = "riscv64"))]
pub fn lr() -> usize {
    let ptr: usize;
    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!("mov {}, x30", out(reg) ptr);
    }
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    unsafe {
        asm!("mv {}, ra", out(reg) ptr);
    }

    ptr
}

// Print the backtrace starting from the caller
pub fn backtrace() {
    #[cfg(target_arch = "x86_64")]
    print!("Backtrace:\n");
    #[cfg(any(target_arch = "x86_64", target_arch = "riscv32", target_arch = "riscv64"))]
    unsafe {
        let mut current_pc = lr();
        let mut current_fp = fp();
        let mut stack_num = 0;
        while current_pc >= stext as usize && current_pc <= etext as usize && current_fp as usize != 0 {
            println!("#{} {:#018X} fp {:#018X}", stack_num, current_pc - 4, current_fp);
            stack_num = stack_num + 1;
            #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
            {
                current_fp = *(current_fp as *const usize).offset(-2);
                current_pc = *(current_fp as *const usize).offset(-1);
            }
            #[cfg(target_arch = "x86_64")]
            {
                current_fp = *(current_fp as *const usize);
                if current_fp != 0 {
                    current_pc = *(current_fp as *const usize).offset(1);
                }
            }
        }
    }
}