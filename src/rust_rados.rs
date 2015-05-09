extern crate libc;
extern crate rustc_serialize;
use libc::*;
use std::ffi::CString;

#[repr(C)]
struct radost_context;
#[repr(C)]
struct RadosT{
    ctx: *mut radost_context,
}

#[repr(C)]
struct rados_cluster_stat_t {
    kb: u64,
    kb_used: uint64_t,
    kb_avail: uint64_t,
    num_objects: uint64_t,
}

#[link(name = "rados")]
extern {
    // this is a foreign function
    fn rados_create(cluster: *mut *mut RadosT, id: *const c_char) -> c_int;
    fn rados_conf_read_file(cluster: *mut RadosT, path: *const c_char) -> c_int;
    fn rados_connect(cluster: *mut RadosT) -> c_int;
    fn rados_cluster_stat(cluster: *mut RadosT,
        result: *mut rados_cluster_stat_t) -> c_int;
    //rados_ioctx_destroy(io);
    fn rados_shutdown(cluster: *mut RadosT) -> c_int;
}

fn main() {
    let connect_id = CString::new("admin").unwrap();
    let conf_file = CString::new("/etc/ceph/ceph.conf").unwrap();
    unsafe{
        let mut cluster_handle: *mut RadosT = std::mem::uninitialized();
        let cluster_stat: *mut rados_cluster_stat_t = std::mem::uninitialized();
        print!("Connecting to ceph\n");
        let ret_code = rados_create(&mut cluster_handle, connect_id.as_ptr());
        println!("Connection result: {}",ret_code);
        rados_conf_read_file(cluster_handle, conf_file.as_ptr());
        rados_connect(cluster_handle);
        println!("We're Connected\n");
        rados_cluster_stat(cluster_handle, cluster_stat);
        println!("Cluster usage: {}", (*cluster_stat).kb_avail);
        rados_shutdown(cluster_handle);
    }

    print!("rustc succeeded\n");
}
