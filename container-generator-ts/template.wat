(module
  ;; Import the abort function and context functions
  (import "env" "abort" (func $abort (param i32 i32 i32 i32)))
  (import "env" "__request_context" (func $__request_context (result i32)))
  (import "env" "__load_context" (func $__load_context (param i32)))

  ;; Memory declaration
  (memory (export "memory") 1)

  ;; Define our data section - this will be replaced by our script
  (data (i32.const 0) "DATA_PLACEHOLDER")

  ;; Export the __execute function
  (func (export "__execute") (result i32)
    ;; Define locals
    (local $context_size i32)
    (local $context_ptr i32)
    (local $buffer_ptr i32)
    (local $opcode i32)
    (local $i i32)
    (local $result i32)
    (local $dest i32)
    (local $src i32)
    (local $size i32)
    
    ;; Request context size
    (call $__request_context)
    (local.set $context_size)
    
    ;; Allocate memory for context
    (i32.const 512)                   ;; Context buffer at position 512
    (local.set $context_ptr)
    
    ;; Create arraybuffer layout for context
    (local.get $context_ptr)
    (local.get $context_size)
    (i32.store)                       ;; Store size at context_ptr
    
    ;; Load context into memory
    (local.get $context_ptr)
    (i32.const 4)
    (i32.add)                         ;; context_ptr + 4 (after size)
    (call $__load_context)
    
    ;; Parse context to get opcode (first input)
    ;; In a real implementation, we would parse the entire context structure
    ;; For simplicity, we'll assume the opcode is at a specific offset
    ;; Based on the Context struct, inputs are after myself, caller, vout, and incoming_alkanes
    ;; We'll use offset 100 as an approximation
    (local.get $context_ptr)
    (i32.const 4)                     ;; Skip size
    (i32.add)
    (i32.const 100)                   ;; Approximate offset to first input
    (i32.add)
    (i32.load)                        ;; Load the opcode
    (local.set $opcode)
    
    ;; Check if opcode is 1000 (GetData)
    (local.get $opcode)
    (i32.const 1000)
    (i32.eq)
    (if (result i32)                  ;; Explicitly declare that if returns i32
      (then
        ;; Opcode is 1000 (GetData), return the data
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
        ;; Calculate destination address
        (i32.const 1024)              ;; Destination base
        (i32.const 16)                ;; Destination offset (after alkanes count)
        (i32.add)                     ;; Destination address
        (local.set $dest)             ;; Store in local
        
        ;; Set source address
        (i32.const 0)                 ;; Source address (our data)
        (local.set $src)              ;; Store in local
        
        ;; Set size to copy
        (i32.const DATA_SIZE)         ;; Size to copy - will be replaced by script
        (local.set $size)             ;; Store in local
        
        ;; Perform the memory copy
        (local.get $dest)
        (local.get $src)
        (local.get $size)
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
        (local.tee $buffer_ptr)       ;; Save the address in local
        
        ;; Store the size of our CallResponse as a 4-byte little-endian u32
        (i32.const 16)                ;; 16 bytes for alkanes count
        (i32.const DATA_SIZE)         ;; Size of our data - will be replaced by script
        (i32.add)                     ;; Total size of CallResponse
        (i32.store)                   ;; Store at position 2048
        
        ;; Now copy our CallResponse after the size
        ;; Calculate destination address
        (i32.const 2048)              ;; Destination base
        (i32.const 4)                 ;; Destination offset (after size)
        (i32.add)                     ;; Destination address
        (local.set $dest)             ;; Store in local
        
        ;; Set source address
        (i32.const 1024)              ;; Source address (our CallResponse)
        (local.set $src)              ;; Store in local
        
        ;; Calculate size to copy
        (i32.const 16)                ;; 16 bytes for alkanes count
        (i32.const DATA_SIZE)         ;; Size of our data - will be replaced by script
        (i32.add)                     ;; Total size to copy
        (local.set $size)             ;; Store in local
        
        ;; Perform the memory copy
        (local.get $dest)
        (local.get $src)
        (local.get $size)
        (memory.copy)                 ;; Copy the data
        
        ;; Return the pointer to the arraybuffer layout + 4
        (local.get $buffer_ptr)       ;; Get the base address
        (i32.const 4)                 ;; Offset
        (i32.add)                     ;; Add the offset - this value is returned
      )
      (else
        ;; Opcode is not 1000, return an empty response
        ;; Create an empty CallResponse with no alkanes and no data
        
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
        
        ;; Now we need to create the arraybuffer layout
        ;; Format: [size(4 bytes)][data]
        ;; Where data is our CallResponse: [alkanes_count(16 bytes)][data]
        
        ;; Calculate the total size of our CallResponse (just the alkanes count)
        (i32.const 16)                ;; 16 bytes for alkanes count
        
        ;; Allocate memory at position 2048 for our arraybuffer layout
        (i32.const 2048)              ;; Destination address for arraybuffer layout
        (local.tee $buffer_ptr)       ;; Save the address in local
        
        ;; Store the size of our CallResponse as a 4-byte little-endian u32
        (i32.const 16)                ;; 16 bytes for alkanes count (no data)
        (i32.store)                   ;; Store at position 2048
        
        ;; Now copy our CallResponse after the size
        ;; Calculate destination address
        (i32.const 2048)              ;; Destination base
        (i32.const 4)                 ;; Destination offset (after size)
        (i32.add)                     ;; Destination address
        (local.set $dest)             ;; Store in local
        
        ;; Set source address
        (i32.const 1024)              ;; Source address (our CallResponse)
        (local.set $src)              ;; Store in local
        
        ;; Set size to copy
        (i32.const 16)                ;; 16 bytes for alkanes count (no data)
        (local.set $size)             ;; Store in local
        
        ;; Perform the memory copy
        (local.get $dest)
        (local.get $src)
        (local.get $size)
        (memory.copy)                 ;; Copy the data
        
        ;; Return the pointer to the arraybuffer layout + 4
        (local.get $buffer_ptr)       ;; Get the base address
        (i32.const 4)                 ;; Offset
        (i32.add)                     ;; Add the offset - this value is returned
      )
    )
  )
)