// Inspired by Tifflin OS


/// A handle for frozen interrupts
#[derive(Default)]
pub struct HeldInterrupts(bool);

/// Prevent interrupts from firing until the return value is dropped (goes out of scope). 
/// After it is dropped, the interrupts are returned to their prior state, not blindly re-enabled. 
pub fn hold_interrupts() -> HeldInterrupts {
    let enabled = interrupts_enabled();
	let retval = HeldInterrupts(enabled);
    disable_interrupts();
    // trace!("hold_interrupts(): disabled interrupts, were {}", enabled);
    retval
}


impl ::core::ops::Drop for HeldInterrupts {
	fn drop(&mut self)
	{
        // trace!("hold_interrupts(): enabling interrupts? {}", self.0);
		if self.0 {
			enable_interrupts();
		}
	}
}




// Rust wrappers around the x86-family of interrupt-related instructions.
#[inline(always)]
pub fn enable_interrupts() {
    #[cfg(any(target_arch="x86", target_arch="x86_64"))]
    unsafe { asm!("sti" : : : "memory" : "volatile"); }

    #[cfg(any(target_arch="aarch64"))]
    unsafe { asm!("cpsie i" : : : "memory" : "volatile"); }
    
}


#[inline(always)]
pub fn disable_interrupts() {
    #[cfg(any(target_arch="x86", target_arch="x86_64"))]
    unsafe { asm!("cli" : : : "memory" : "volatile"); }

    #[cfg(any(target_arch="aarch64"))]
    unsafe { asm!("cpsid i" : : : "memory" : "volatile"); }
}


#[inline(always)]
pub fn interrupts_enabled() -> bool {
    #[cfg(any(target_arch="x86", target_arch="x86_64"))]
    unsafe { 
        // we only need the lower 16 bits of the eflags/rflags register
        let flags: usize;
		asm!("pushf; pop $0" : "=r" (flags) : : "memory" : "volatile");
		(flags & 0x0200) != 0
     }

    #[cfg(any(target_arch="aarch64"))]
    unsafe {
        let primask:usize;  
        asm!("mrs $0, PRIMASK" : "=r"(primask) : : : "volatile");
        primask == 0
    }

}
