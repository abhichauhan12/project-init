# Basic OS
    Writing a basic os in Rust
## 1. Bare Bones
 a. Freestanding Rust Binary 
  - create a rust binary that run without standard lib 
    without - threads , i/o , files , heap memory , network etc 
    

// to run in qemu 
qemu-system-x86_64 -drive format=raw,file=target/x86_basic_os/debug/bootimage-basic_os.bin