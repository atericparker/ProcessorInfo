use windows::Win32::System::SystemInformation::{
    GetLogicalProcessorInformation, SYSTEM_LOGICAL_PROCESSOR_INFORMATION,
    PROCESSOR_CACHE_TYPE, LOGICAL_PROCESSOR_RELATIONSHIP, RelationCache, RelationNumaNode, RelationProcessorCore, RelationProcessorPackage
};
use windows::Win32::Foundation::{GetLastError, ERROR_INSUFFICIENT_BUFFER};
use std::mem::size_of;

fn main() {
    let mut buffer_size = 0;
    let mut buffer = Vec::new();

    // First call to get the required buffer size
    unsafe {
        let result = GetLogicalProcessorInformation(None, &mut buffer_size);
        if result.is_err() {
            let error = GetLastError();
            if error != ERROR_INSUFFICIENT_BUFFER {
                panic!("Unexpected error: {:?}", error);
            }
        }
    }

    // Allocate the buffer
    buffer.resize(buffer_size as usize / size_of::<SYSTEM_LOGICAL_PROCESSOR_INFORMATION>(), 
                  SYSTEM_LOGICAL_PROCESSOR_INFORMATION::default());

    // Second call to get the actual data
    let success = unsafe {
        GetLogicalProcessorInformation(Some(buffer.as_mut_ptr()), &mut buffer_size)
    };

    if success.is_err() {
        panic!("Failed to get logical processor information: {:?}", unsafe { GetLastError() });
    }

    println!("Logical Processor Information:");
    for info in buffer.iter() {
        match info.Relationship {
            RelationProcessorCore => {
                println!("  Processor Core:");
                println!("    ProcessorMask: {:b}", info.ProcessorMask);
                println!("    Flags: {}", if unsafe { info.Anonymous.ProcessorCore.Flags } == 1 { "Hyperthreaded" } else { "Not Hyperthreaded" });
            },
            RelationNumaNode => {
                println!("  NUMA Node:");
                println!("    NodeNumber: {}", unsafe { info.Anonymous.NumaNode.NodeNumber });
            },
            RelationCache => {
                let cache = unsafe { &info.Anonymous.Cache };
                println!("  Cache:");
                println!("    Level: {}", cache.Level);
                println!("    Associativity: {}", cache.Associativity);
                println!("    LineSize: {}", cache.LineSize);
                println!("    Size: {}", cache.Size);
                println!("    Type: {}", match cache.Type {
                    CacheUnified => "Unified",
                    CacheInstruction => "Instruction",
                    CacheData => "Data",
                    CacheTrace => "Trace",
                    _ => "Unknown",
                });
            },
            RelationProcessorPackage => {
                println!("  Processor Package:");
                println!("    ProcessorMask: {:b}", info.ProcessorMask);
            },
            _ => println!("  Unknown relationship type"),
        }
    }

    // Print total number of logical processors
    let total_logical_processors = buffer.iter()
        .filter(|info| info.Relationship == RelationProcessorCore)
        .map(|info| info.ProcessorMask.count_ones())
        .sum::<u32>();

    println!("Total logical processors: {}", total_logical_processors);
}
