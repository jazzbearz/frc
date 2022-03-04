use std::ffi::c_void;

extern "C" {
    pub fn get_thread_number() -> *mut u32;
    pub fn initialize_thread_number();
    pub fn uninitialize_thread_number();
    pub fn store_local_pointer(ptr: *mut c_void);
    pub fn get_local_pointer() -> *mut c_void;
}

mod imp;
mod inner;

pub use imp::Frc;

#[cfg(test)]
mod bench {
    struct RandGen {}

    impl RandGen {
        pub fn new() -> RandGen {
            RandGen {}
        }

        fn gen(&self) -> u32 {
            unsafe { *crate::get_thread_number() }
            // let ptr = Box::into_raw(Box::new(123));
            // let ret = (ptr as usize % 10) as u32;
            // unsafe {
            //     ptr.drop_in_place();
            // }
            // ret
        }
    }

    #[test]
    fn run_bench() {
        unsafe {
            crate::initialize_thread_number();
        }
        use std::time::Instant;
        const RUNCNT: u32 = 100;
        let mut val = 0;
        let now = Instant::now();
        {
            val += run_rc();
        }
        let mut elapsed = now.elapsed();
        for _ in 0..RUNCNT {
            let now = Instant::now();
            {
                val += run_rc();
            }
            let c_elapsed = now.elapsed();
            elapsed += c_elapsed;
        }
        println!("RC Elapsed: {:.2?} : {}", elapsed / RUNCNT, val);

        let mut val = 0;
        let now = Instant::now();
        {
            val += run_arc();
        }
        let mut elapsed = now.elapsed();
        for _ in 0..RUNCNT {
            let now = Instant::now();
            {
                val += run_arc();
            }
            let c_elapsed = now.elapsed();
            elapsed += c_elapsed;
        }
        println!("ARC Elapsed: {:.2?} : {}", elapsed / RUNCNT, val);

        let mut val = 0;
        let now = Instant::now();
        {
            val += run_frc();
        }
        let mut elapsed = now.elapsed();
        for _ in 0..RUNCNT {
            let now = Instant::now();
            {
                val += run_frc();
            }
            let c_elapsed = now.elapsed();
            elapsed += c_elapsed;
        }
        println!("FRC Elapsed: {:.2?} : {}", elapsed / RUNCNT, val);

        unsafe {
            crate::uninitialize_thread_number();
        }
    }

    fn run_rc() -> u32 {
        let rc = std::rc::Rc::new(RandGen::new());
        let mut variable = 0;
        for _ in 0..10000 {
            let v = rc.clone();
            variable += v.gen();
        }
        variable
    }

    fn run_arc() -> u32 {
        let rc = std::sync::Arc::new(RandGen::new());
        let mut variable = 0;
        for _ in 0..10000 {
            let v = rc.clone();
            variable += v.gen();
        }
        variable
    }

    fn run_frc() -> u32 {
        let rc = crate::Frc::new(RandGen::new());
        let mut variable = 0;
        for _ in 0..10000 {
            let v = rc.clone();
            variable += v.gen();
        }
        variable
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::{BTreeMap, BTreeSet},
        ops::Add,
        sync::atomic::AtomicU32,
    };

    use crate::{
        get_local_pointer, get_thread_number, initialize_thread_number, store_local_pointer,
        uninitialize_thread_number,
    };

    #[test]
    fn run_test() {
        thread_counter_test();
        tokio_test();
    }

    fn tokio_test() {
        const WORKER_CNT: usize = 8;
        let mut counter_map = crate::Frc::new(BTreeMap::<u32, AtomicU32>::new());
        let lock = std::sync::Arc::new(std::sync::Mutex::new(0_u32));
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(WORKER_CNT)
            .enable_all()
            .on_thread_start(|| unsafe {
                let lset_o = std::boxed::Box::new(tokio::task::LocalSet::new());
                let lset_raw: *const tokio::task::LocalSet = std::boxed::Box::into_raw(lset_o);
                let lset_ptr = std::ptr::NonNull::new_unchecked(lset_raw as *mut _).as_ptr();
                store_local_pointer(lset_ptr);
                initialize_thread_number();
            })
            .on_thread_stop(|| unsafe {
                let lset = get_local_pointer() as *mut tokio::task::LocalSet;
                lset.drop_in_place();
                uninitialize_thread_number();
            })
            .build();

        for i in 0..WORKER_CNT {
            counter_map.insert(i as u32, AtomicU32::new(0));
        }
        if let Ok(runt) = runtime {
            let sv = crate::Frc::new(100);
            for _ in 0..(WORKER_CNT * 100) {
                let svv = sv.clone();
                let llock = lock.clone();
                let cmap = counter_map.clone();
                runt.spawn(async move {
                    let tn = unsafe { *get_thread_number() };
                    if *svv != 100 {
                        panic!("stored value is not 100:: possible corruption")
                    }
                    match llock.lock() {
                        Ok(v) => {
                            let _ = v.add(1);
                            if let Some(cnt) = cmap.get(&tn) {
                                cnt.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            }
                        }
                        Err(e) => {
                            panic!("{}", e);
                        }
                    }
                });
            }
            runt.block_on(async {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            });
            let mut totals = 0;
            for i in 0..WORKER_CNT {
                if let Some(cnt) = counter_map.get(&(i as u32)) {
                    let ccnt = cnt.load(std::sync::atomic::Ordering::Relaxed);
                    totals += ccnt;
                    println!("{}:{}:{}", i, ccnt, totals);
                }
            }
            if totals != WORKER_CNT as u32 * 100 {
                panic!("SIZE NOT MATCH expected {}:{}", totals, WORKER_CNT * 100);
            }
        }
    }

    fn thread_counter_test() {
        unsafe {
            crate::initialize_thread_number();
            let tno = crate::get_thread_number();
            let counter_set = crate::Frc::new(std::sync::Mutex::new(BTreeSet::<u32>::new()));
            println!("thread main no: {}", *tno);
            {
                counter_set.lock().unwrap().insert(*tno);
            }

            let cset_1 = counter_set.clone();
            let cset_2 = counter_set.clone();

            let handler = std::thread::spawn(move || {
                crate::initialize_thread_number();
                let tno = crate::get_thread_number();
                println!("thread sub no: {}", *tno);
                {
                    if !cset_1.lock().unwrap().insert(*tno) {
                        panic!("already inserted thread number : {}", *tno);
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
                crate::uninitialize_thread_number();
            });
            let handler2 = std::thread::spawn(move || {
                crate::initialize_thread_number();
                let tno = crate::get_thread_number();
                println!("thread sub no: {}", *tno);
                {
                    if !cset_2.lock().unwrap().insert(*tno) {
                        panic!("already inserted thread number : {}", *tno);
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
                crate::uninitialize_thread_number();
            });
            handler.join().unwrap();
            handler2.join().unwrap();

            {
                counter_set.lock().unwrap().clear();
            }

            let cset_1 = counter_set.clone();
            let cset_2 = counter_set.clone();
            let cset_3 = counter_set.clone();

            let handler = std::thread::spawn(move || {
                crate::initialize_thread_number();
                let tno = crate::get_thread_number();
                println!("thread sub no: {}", *tno);
                {
                    if !cset_1.lock().unwrap().insert(*tno) {
                        panic!("already inserted thread number : {}", *tno);
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
                crate::uninitialize_thread_number();
            });
            let handler2 = std::thread::spawn(move || {
                crate::initialize_thread_number();
                let tno = crate::get_thread_number();
                println!("thread sub no: {}", *tno);
                {
                    if !cset_2.lock().unwrap().insert(*tno) {
                        panic!("already inserted thread number : {}", *tno);
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
                crate::uninitialize_thread_number();
            });
            let handler3 = std::thread::spawn(move || {
                crate::initialize_thread_number();
                let tno = crate::get_thread_number();
                println!("thread sub no: {}", *tno);
                {
                    if !cset_3.lock().unwrap().insert(*tno) {
                        panic!("already inserted thread number : {}", *tno);
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
                crate::uninitialize_thread_number();
            });
            handler.join().unwrap();
            handler2.join().unwrap();
            handler3.join().unwrap();

            {
                counter_set.lock().unwrap().clear();
            }

            let cset_1 = counter_set;

            let handler = std::thread::spawn(move || {
                crate::initialize_thread_number();
                let tno = crate::get_thread_number();
                println!("thread sub no: {}", *tno);
                {
                    if !cset_1.lock().unwrap().insert(*tno) {
                        panic!("already inserted thread number : {}", *tno);
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
                crate::uninitialize_thread_number();
            });
            handler.join().unwrap();

            crate::uninitialize_thread_number();
        }
    }
}
