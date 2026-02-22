// src/memory.rs
use x86_64::structures::paging::{PhysFrame, FrameAllocator, Size4KiB, Mapper, Page, OffsetPageTable};
use x86_64::PhysAddr;
use x86_64::VirtAddr;
use x86_64::registers::control::Cr3;
use spin::Mutex;
use limine::response::MemoryMapResponse;
use limine::memory_map::EntryType;

pub struct SimpleFrameAllocator {
    next_free: u64,
    memory_end: u64,
}

impl SimpleFrameAllocator {
    pub const fn new() -> Self {
        SimpleFrameAllocator {
            next_free: 0,
            memory_end: 0,
        }
    }

    pub fn init(&mut self, memory_map: &MemoryMapResponse) {
        for entry in memory_map.entries() {
            // Comparar directamente con el enum
            if entry.entry_type == EntryType::USABLE {
                self.next_free = entry.base;
                self.memory_end = entry.base + entry.length;
                crate::println!("FrameAllocator: usando región {:#x} - {:#x}", 
                    self.next_free, self.memory_end);
                break;
            }
        }
    }
}

unsafe impl FrameAllocator<Size4KiB> for SimpleFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        if self.next_free + 4096 <= self.memory_end {
            let frame = PhysFrame::containing_address(PhysAddr::new(self.next_free));
            self.next_free += 4096;
            Some(frame)
        } else {
            None
        }
    }
}

pub static FRAME_ALLOCATOR: Mutex<SimpleFrameAllocator> = Mutex::new(SimpleFrameAllocator::new());

pub fn map_range(
    virt_start: VirtAddr,
    page_count: u64,
    flags: x86_64::structures::paging::PageTableFlags,
) -> Result<(), &'static str> {
    let mut allocator = FRAME_ALLOCATOR.lock();
    
    let (level_4_table, _) = Cr3::read();
    let offset_opt = crate::HHDM_OFFSET.lock();
    let phys_to_virt_offset = *offset_opt.as_ref().unwrap();  // Sin dereferenciar
    
    unsafe {
        let mut mapper = OffsetPageTable::new(
            &mut *(crate::phys_to_virt(level_4_table.start_address().as_u64()) as *mut _),
            VirtAddr::new(phys_to_virt_offset),
        );

        for i in 0..page_count {
            let page = Page::<Size4KiB>::containing_address(virt_start + i * 4096);
            let frame = allocator.allocate_frame().ok_or("No hay frames disponibles")?;
            
            mapper.map_to(page, frame, flags, &mut *allocator)
                .map_err(|_| "Error al mapear")?
                .flush();
        }
    }
    
    Ok(())
}
