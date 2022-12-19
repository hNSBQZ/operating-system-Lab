pub struct PidHandle(pub usize);
truct PidAllocator {
        current: usize,
            recycled: Vec<usize>,
}

impl PidAllocator {
        pub fn new() -> Self {
                    PidAllocator {
                                    current: 0,
                                                recycled: Vec::new(),
                                                        }
                        }
            pub fn alloc(&mut self) -> PidHandle {
                        if let Some(pid) = self.recycled.pop() {
                                        PidHandle(pid)
                                                    } else {
                                                                    self.current += 1;
                                                                                PidHandle(self.current - 1)
                                                                                            }
                            }
                pub fn dealloc(&mut self, pid: usize) {
                            assert!(pid < self.current);
                                    assert!(
                                                    self.recycled.iter().find(|ppid| **ppid == pid).is_none(),
                                                                "pid {} has been deallocated!", pid
                                                                        );
                                            self.recycled.push(pid);
                                                }
}

lazy_static! {
        static ref PID_ALLOCATOR : UPSafeCell<PidAllocator> = unsafe {
                    UPSafeCell::new(PidAllocator::new())
                            };
}
