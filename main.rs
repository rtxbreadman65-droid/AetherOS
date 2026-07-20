#![no_std]
#![no_main]

use uefi::prelude::*;
use uefi::Status;
use core::fmt::Write;
use core::arch::asm;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::table::runtime::ResetType;
use uefi::table::boot::SearchType;
use uefi::Identify;
use uefi::table::boot::{MemoryType, AllocateType};
static mut SYSTEM_TABLE_POINTER: *mut uefi::table::SystemTable<Boot> = core::ptr::null_mut();
static KERNEL_BYTES: &[u8] = include_bytes!("../kernel.bin");

fn memory_alloc(system_table: &mut SystemTable<Boot>) -> u64 {

    let pages = 1000;
    let address = 0x0200_0000;

	let physical_address = system_table.boot_services() 
	    .allocate_pages(
	    	AllocateType::Address(address),
	    	MemoryType::LOADER_DATA,
	    	pages
	    ).unwrap();

	physical_address
}

fn shutdown(system_table: &mut SystemTable<Boot>) -> ! {

    let _ = system_table.stdout().write_str("Shuting down system in 5 seconds.\n");
    system_table.boot_services().stall(5_000_000);

    let reset_type = ResetType::SHUTDOWN;
    let status = Status::SUCCESS;

    system_table.runtime_services()
        .reset(
        	reset_type,
        	status,
        	None
        )
	
}

#[entry]

fn efi_main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {

    unsafe {
    	SYSTEM_TABLE_POINTER = &mut system_table as *mut SystemTable<Boot>
    }

	let _ = system_table.stdout().clear();
	let _ = system_table.stdout().write_str("==================================\n[+] Welcome to Hobby OS Bootloader.\n==================================\n");

    let kernel_address = memory_alloc(&mut system_table);
    let raw_buffer = unsafe {
    	core::slice::from_raw_parts_mut(kernel_address as *mut u8, KERNEL_BYTES.len())
    };

    raw_buffer.copy_from_slice(KERNEL_BYTES);

    let _ = system_table.stdout().write_str("\n[+] Kernel copied to ram successfully!\n");
    let _ = system_table.stdout().write_str("[+] Booting kernel in 5 seconds.\n");
    system_table.boot_services().stall(5_000_000);
    let _ = system_table.stdout().clear();

    let (fb_ptr, screen_with, screen_height, stride) = {

                   let search_type = SearchType::ByProtocol(&GraphicsOutput::GUID);

                   let handle = system_table.boot_services()
                        .locate_handle_buffer(
        	                        search_type
                        )
                        .expect("[-] Error cannot locate handle buffer.");

                   let buffer_frame = handle[0];
                   let mut open_protocol = system_table.boot_services() 
                           .open_protocol_exclusive::<GraphicsOutput>(buffer_frame)
                           .expect("[-] Error cannot open protocol explusive.");

                   let mode_info = open_protocol.current_mode_info();
                   let screen_with = mode_info.resolution().0;
                   let screen_height = mode_info.resolution().1;
                   let stride = mode_info.stride();

                   let frame_buffer_ptr = open_protocol.frame_buffer().as_mut_ptr() as *mut u32;

                   (frame_buffer_ptr, stride, screen_with, screen_height)

                 };

    let rsdp = {

                   use uefi::table::cfg;
                   let entry = system_table.config_table();
                   let mut acpi_ptr = 0;

                   for rdsp in entry {
                       if rdsp.guid == cfg::ACPI2_GUID {
                           acpi_ptr = rdsp.address as usize
                           };
                   }
                   
                   acpi_ptr
    	
    };

    unsafe {
            
            let _ = system_table.exit_boot_services(uefi::table::boot::MemoryType::LOADER_DATA);
	        asm!("jmp {}", in(reg) kernel_address, in("rdi") fb_ptr, in("rsi") stride, in("rdx") screen_with, in("rcx") screen_height, in("r8") rsdp, options(noreturn));

	}

}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {

    unsafe {

        let system_table = &mut *SYSTEM_TABLE_POINTER;
    	let _ = system_table.stdout().write_str("[-] Error.\n");

    	shutdown(system_table)
    }
}
