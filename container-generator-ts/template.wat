(module
  ;; Memory declaration
  (memory (export "memory") 1)

  ;; Import the abort function
  (import "env" "abort" (func $abort (param i32 i32 i32 i32)))

  ;; Define our data section - this will be replaced by our script
  (data (i32.const 0) "DATA_PLACEHOLDER")

  ;; Export the __execute function
  (func (export "__execute") (result i32)
    ;; Create a CallResponse with empty alkanes and our data
    ;; Format of CallResponse: [alkanes_count(16 bytes)][data]
    
    ;; First, we need to create the CallResponse
    ;; We'll allocate memory at position 1024 for our CallResponse
    (i32.const 1024)              ;; Destination address for CallResponse
    
    ;; Store alkanes count (0) as first 16 bytes
    (i64.const 0)                 ;; No alkanes (first 8 bytes)
    (i64.store)                   ;; Store at position 1024
    
    (i32.const 1024)
    (i32.const 8)
    (i32.add)                     ;; Position 1024 + 8
    (i64.const 0)                 ;; No alkanes (second 8 bytes)
    (i64.store)                   ;; Store at position 1024 + 8
    
    ;; Now copy our data after the alkanes count
    (i32.const 1024)              ;; Destination base
    (i32.const 16)                ;; Destination offset (after alkanes count)
    (i32.add)                     ;; Destination address
    (i32.const 0)                 ;; Source address (our data)
    (i32.const DATA_SIZE)         ;; Size to copy - will be replaced by script
    (memory.copy)                 ;; Copy the data
    
    ;; Now we need to create the arraybuffer layout
    ;; Format: [size(4 bytes)][data]
    ;; Where data is our CallResponse: [alkanes_count(16 bytes)][data]
    
    ;; Calculate the total size of our CallResponse
    (i32.const 16)                ;; 16 bytes for alkanes count
    (i32.const DATA_SIZE)         ;; Size of our data - will be replaced by script
    (i32.add)                     ;; Total size of CallResponse
    
    ;; Allocate memory at position 2048 for our arraybuffer layout
    (i32.const 2048)              ;; Destination address for arraybuffer layout
    (local.tee 0)                 ;; Save the address in local 0
    
    ;; Store the size of our CallResponse as a 4-byte little-endian u32
    (i32.const 16)                ;; 16 bytes for alkanes count
    (i32.const DATA_SIZE)         ;; Size of our data - will be replaced by script
    (i32.add)                     ;; Total size of CallResponse
    (i32.store)                   ;; Store at position 2048
    
    ;; Now copy our CallResponse after the size
    (i32.const 2048)              ;; Destination base
    (i32.const 4)                 ;; Destination offset (after size)
    (i32.add)                     ;; Destination address
    (i32.const 1024)              ;; Source address (our CallResponse)
    (i32.const 16)                ;; 16 bytes for alkanes count
    (i32.const DATA_SIZE)         ;; Size of our data - will be replaced by script
    (i32.add)                     ;; Total size to copy
    (memory.copy)                 ;; Copy the data
    
    ;; Return the pointer to the arraybuffer layout + 4
    ;; (the +4 is because the runtime expects the pointer to point after the size)
    (local.get 0)                 ;; Get the base address
    (i32.const 4)                 ;; Offset
    (i32.add)                     ;; Add the offset
  )
)