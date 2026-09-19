// Copyright 2017 LambdaStack All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Portions from Chris Holcombe
// The MIT License (MIT)
//
// Copyright (c) 2015 Chris Holcombe
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use bitflags::bitflags;
use libc::{size_t, ssize_t, time_t, timeval};

bitflags! {
    pub struct AllocFlags: u32 {
        const LIBRADOS_OP_FLAG_EXCL = 1;
        const LIBRADOS_OP_FLAG_FAILOK = 2;
        const LIBRADOS_OP_FLAG_FADVISE_RANDOM = 4;
        const LIBRADOS_OP_FLAG_FADVISE_SEQUENTIAL = 8;
        const LIBRADOS_OP_FLAG_FADVISE_WILLNEED = 16;
        const LIBRADOS_OP_FLAG_FADVISE_DONTNEED = 32;
        const LIBRADOS_OP_FLAG_FADVISE_NOCACHE = 64;
    }
}

bitflags! {
    pub struct XattrFlags: u32 {
        const LIBRADOS_CMPXATTR_OP_EQ = 1;
        const LIBRADOS_CMPXATTR_OP_NE = 2;
        const LIBRADOS_CMPXATTR_OP_GT = 3;
        const LIBRADOS_CMPXATTR_OP_GTE = 4;
        const LIBRADOS_CMPXATTR_OP_LT = 5;
        const LIBRADOS_CMPXATTR_OP_LTE = 6;
    }
}

// Flags for rados_read_op_operate(), rados_write_op_operate(),
// rados_aio_read_op_operate(), and rados_aio_write_op_operate()
bitflags! {
    pub struct OperationFlags: u32 {
        const LIBRADOS_OPERATION_NOFLAG= 0;
        const LIBRADOS_OPERATION_BALANCE_READS= 1;
        const LIBRADOS_OPERATION_LOCALIZE_READS= 2;
        const LIBRADOS_OPERATION_ORDER_READS_WRITES= 4;
        const LIBRADOS_OPERATION_IGNORE_CACHE= 8;
        const LIBRADOS_OPERATION_SKIPRWLOCKS= 16;
        const LIBRADOS_OPERATION_IGNORE_OVERLAY= 32;
    }
}

pub type rados_t = *mut std::os::raw::c_void;
pub type rados_config_t = *mut std::os::raw::c_void;
pub type rados_ioctx_t = *mut std::os::raw::c_void;
pub type rados_list_ctx_t = *mut std::os::raw::c_void;
pub type rados_snap_t = u64;
pub type rados_xattrs_iter_t = *mut std::os::raw::c_void;
pub type rados_omap_iter_t = *mut std::os::raw::c_void;

#[repr(C)]
#[derive(Copy, Debug)]
pub struct Struct_rados_pool_stat_t {
    pub num_bytes: u64,
    pub num_kb: u64,
    pub num_objects: u64,
    pub num_object_clones: u64,
    pub num_object_copies: u64,
    pub num_objects_missing_on_primary: u64,
    pub num_objects_unfound: u64,
    pub num_objects_degraded: u64,
    pub num_rd: u64,
    pub num_rd_kb: u64,
    pub num_wr: u64,
    pub num_wr_kb: u64,
}

impl std::clone::Clone for Struct_rados_pool_stat_t {
    fn clone(&self) -> Self {
        *self
    }
}

impl std::default::Default for Struct_rados_pool_stat_t {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Debug)]
pub struct Struct_rados_cluster_stat_t {
    pub kb: u64,
    pub kb_used: u64,
    pub kb_avail: u64,
    pub num_objects: u64,
}

impl std::clone::Clone for Struct_rados_cluster_stat_t {
    fn clone(&self) -> Self {
        *self
    }
}

impl std::default::Default for Struct_rados_cluster_stat_t {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

pub type rados_write_op_t = *mut std::os::raw::c_void;

pub type rados_read_op_t = *mut std::os::raw::c_void;

pub type rados_completion_t = *mut std::os::raw::c_void;

pub type rados_callback_t = std::option::Option<
    extern "C" fn(cb: rados_completion_t, arg: *mut std::os::raw::c_void) -> (),
>;

pub type rados_watchcb_t =
    std::option::Option<extern "C" fn(opcode: u8, ver: u64, arg: *mut std::os::raw::c_void) -> ()>;

pub type rados_watchcb2_t = std::option::Option<
    extern "C" fn(
        arg: *mut std::os::raw::c_void,
        notify_id: u64,
        handle: u64,
        notifier_id: u64,
        data: *mut std::os::raw::c_void,
        data_len: size_t,
    ) -> (),
>;

pub type rados_watcherrcb_t = std::option::Option<
    extern "C" fn(pre: *mut std::os::raw::c_void, cookie: u64, err: libc::c_int) -> (),
>;

pub type rados_log_callback_t = std::option::Option<
    extern "C" fn(
        arg: *mut std::os::raw::c_void,
        line: *const libc::c_char,
        who: *const libc::c_char,
        sec: u64,
        nsec: u64,
        seq: u64,
        level: *const libc::c_char,
        msg: *const libc::c_char,
    ) -> (),
>;

#[cfg(unix)]
unsafe extern "C" {
    /**
     * Get the version of librados.
     *
     * The version number is major.minor.extra. Note that this is
     * unrelated to the Ceph version number.
     *
     * TODO: define version semantics, i.e.:
     * - incrementing major is for backwards-incompatible changes
     * - incrementing minor is for backwards-compatible changes
     * - incrementing extra is for bug fixes
     *
     * @param major where to store the major version number
     * @param minor where to store the minor version number
     * @param extra where to store the extra version number
     */
    pub fn rados_version(
        major: *mut libc::c_int,
        minor: *mut libc::c_int,
        extra: *mut libc::c_int,
    ) -> ();

    /**
     * Create a handle for communicating with a RADOS cluster.
     *
     * Ceph environment variables are read when this is called, so if
     * $CEPH_ARGS specifies everything you need to connect, no further
     * configuration is necessary.
     *
     * @param cluster where to store the handle
     * @param id the user to connect as (i.e. admin, not client.admin)
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_create(cluster: *mut rados_t, id: *const libc::c_char) -> libc::c_int;

    /**
     * Extended version of rados_create.
     *
     * Like rados_create, but
     * 1) don't assume 'client\.'+id; allow full specification of name
     * 2) allow specification of cluster name
     * 3) flags for future expansion
     */
    pub fn rados_create2(
        pcluster: *mut rados_t,
        clustername: *const libc::c_char,
        name: *const libc::c_char,
        flags: u64,
    ) -> libc::c_int;

    /**
     * Initialize a cluster handle from an existing configuration.
     *
     * Share configuration state with another rados_t instance.
     *
     * @param cluster where to store the handle
     * @param cct the existing configuration to use
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_create_with_context(cluster: *mut rados_t, cct: rados_config_t) -> libc::c_int;

    /**
     * Ping the monitor with ID mon_id, storing the resulting reply in
     * buf (if specified) with a maximum size of len.
     *
     * The result buffer is allocated on the heap; the caller is
     * expected to release that memory with rados_buffer_free().  The
     * buffer and length pointers can be NULL, in which case they are
     * not filled in.
     *
     * @param cluster         cluster handle
     * @param mon_id \[in\]     ID of the monitor to ping
     * @param outstr \[out\]    double pointer with the resulting reply
     * @param outstrlen \[out\] pointer with the size of the reply in outstr
     */
    pub fn rados_ping_monitor(
        cluster: rados_t,
        mon_id: *const libc::c_char,
        outstr: *mut *mut libc::c_char,
        outstrlen: *mut size_t,
    ) -> libc::c_int;

    /**
     * Connect to the cluster.
     *
     * @note BUG: Before calling this, calling a function that communicates with the
     * cluster will crash.
     *
     * @pre The cluster handle is configured with at least a monitor
     * address. If cephx is enabled, a client name and secret must also be
     * set.
     *
     * @post If this succeeds, any function in librados may be used
     *
     * @param cluster The cluster to connect to.
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_connect(cluster: rados_t) -> libc::c_int;

    /**
     * Disconnects from the cluster.
     *
     * For clean up, this is only necessary after rados_connect() has
     * succeeded.
     *
     * @warning This does not guarantee any asynchronous writes have
     * completed. To do that, you must call rados_aio_flush() on all open
     * io contexts.
     *
     * @warning We implicitly call rados_watch_flush() on shutdown.  If
     * there are watches being used, this should be done explicitly before
     * destroying the relevant IoCtx.  We do it here as a safety measure.
     *
     * @post the cluster handle cannot be used again
     *
     * @param cluster the cluster to shutdown
     */
    pub fn rados_shutdown(cluster: rados_t) -> ();

    /**
     * Configure the cluster handle using a Ceph config file
     *
     * If path is NULL, the default locations are searched, and the first
     * found is used. The locations are:
     * - $CEPH_CONF (environment variable)
     * - /etc/ceph/ceph.conf
     * - ~/.ceph/config
     * - ceph.conf (in the current working directory)
     *
     * @pre rados_connect() has not been called on the cluster handle
     *
     * @param cluster cluster handle to configure
     * @param path path to a Ceph configuration file
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_conf_read_file(cluster: rados_t, path: *const libc::c_char) -> libc::c_int;

    /**
     * Configure the cluster handle with command line arguments
     *
     * argv can contain any common Ceph command line option, including any
     * configuration parameter prefixed by '--' and replacing spaces with
     * dashes or underscores. For example, the following options are equivalent:
     * - --mon-host 10.0.0.1:6789
     * - --mon_host 10.0.0.1:6789
     * - -m 10.0.0.1:6789
     *
     * @pre rados_connect() has not been called on the cluster handle
     *
     * @param cluster cluster handle to configure
     * @param argc number of arguments in argv
     * @param argv arguments to parse
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_conf_parse_argv(
        cluster: rados_t,
        argc: libc::c_int,
        argv: *mut *const libc::c_char,
    ) -> libc::c_int;

    /**
     * Configure the cluster handle with command line arguments, returning
     * any remainders.  Same rados_conf_parse_argv, except for extra
     * remargv argument to hold returns unrecognized arguments.
     *
     * @pre rados_connect() has not been called on the cluster handle
     *
     * @param cluster cluster handle to configure
     * @param argc number of arguments in argv
     * @param argv arguments to parse
     * @param remargv char* array for returned unrecognized arguments
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_conf_parse_argv_remainder(
        cluster: rados_t,
        argc: libc::c_int,
        argv: *mut *const libc::c_char,
        remargv: *mut *const libc::c_char,
    ) -> libc::c_int;

    /**
     * Configure the cluster handle based on an environment variable
     *
     * The contents of the environment variable are parsed as if they were
     * Ceph command line options. If var is NULL, the CEPH_ARGS
     * environment variable is used.
     *
     * @pre rados_connect() has not been called on the cluster handle
     *
     * @note BUG: this is not threadsafe - it uses a static buffer
     *
     * @param cluster cluster handle to configure
     * @param var name of the environment variable to read
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_conf_parse_env(cluster: rados_t, var: *const libc::c_char) -> libc::c_int;

    /**
     * Set a configuration option
     *
     * @pre rados_connect() has not been called on the cluster handle
     *
     * @param cluster cluster handle to configure
     * @param option option to set
     * @param value value of the option
     * @returns 0 on success, negative error code on failure
     * @returns -ENOENT when the option is not a Ceph configuration option
     */
    pub fn rados_conf_set(
        cluster: rados_t,
        option: *const libc::c_char,
        value: *const libc::c_char,
    ) -> libc::c_int;

    /**
     * Get the value of a configuration option
     *
     * @param cluster configuration to read
     * @param option which option to read
     * @param buf where to write the configuration value
     * @param len the size of buf in bytes
     * @returns 0 on success, negative error code on failure
     * @returns -ENAMETOOLONG if the buffer is too short to contain the
     * requested value
     */
    pub fn rados_conf_get(
        cluster: rados_t,
        option: *const libc::c_char,
        buf: *mut libc::c_char,
        len: size_t,
    ) -> libc::c_int;

    /**
     * Read usage info about the cluster
     *
     * This tells you total space, space used, space available, and number
     * of objects. These are not updated immediately when data is written,
     * they are eventually consistent.
     *
     * @param cluster cluster to query
     * @param result where to store the results
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_cluster_stat(
        cluster: rados_t,
        result: *mut Struct_rados_cluster_stat_t,
    ) -> libc::c_int;

    /**
     * Get the fsid of the cluster as a hexadecimal string.
     *
     * The fsid is a unique id of an entire Ceph cluster.
     *
     * @param cluster where to get the fsid
     * @param buf where to write the fsid
     * @param len the size of buf in bytes (should be 37)
     * @returns length of the string on success, negative error code on failure
     * @returns -ERANGE if the buffer is too short to contain the
     * fsid
     *
     * # Note
     * The Ceph documentation states that the return value of 0 means success
     * but actually a value < 0 is an error and a value > 0 is the length
     * which should be 36.
     */
    pub fn rados_cluster_fsid(cluster: rados_t, buf: *mut libc::c_char, len: size_t)
    -> libc::c_int;

    /**
     * Get/wait for the most recent osdmap
     *
     * @param cluster the cluster to shutdown
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_wait_for_latest_osdmap(cluster: rados_t) -> libc::c_int;

    /**
     * List pools
     *
     * Gets a list of pool names as NULL-terminated strings.  The pool
     * names will be placed in the supplied buffer one after another.
     * After the last pool name, there will be two 0 bytes in a row.
     *
     * If len is too short to fit all the pool name entries we need, we will fill
     * as much as we can.
     *
     * Buf may be null to determine the buffer size needed to list all pools.
     *
     * @param cluster cluster handle
     * @param buf output buffer
     * @param len output buffer length
     * @returns length of the buffer we would need to list all pools
     */
    pub fn rados_pool_list(cluster: rados_t, buf: *mut libc::c_char, len: size_t) -> libc::c_int;

    /**
     * Get a configuration handle for a rados cluster handle
     *
     * This handle is valid only as long as the cluster handle is valid.
     *
     * @param cluster cluster handle
     * @returns config handle for this cluster
     */
    pub fn rados_cct(cluster: rados_t) -> rados_config_t;

    /**
     * Get a global id for current instance
     *
     * This id is a unique representation of current connection to the cluster
     *
     * @param cluster cluster handle
     * @returns instance global id
     */
    pub fn rados_get_instance_id(cluster: rados_t) -> u64;

    /**
     * Create an io context
     *
     * The io context allows you to perform operations within a particular
     * pool. For more details see rados_ioctx_t.
     *
     * @param cluster which cluster the pool is in
     * @param pool_name name of the pool
     * @param ioctx where to store the io context
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_ioctx_create(
        cluster: rados_t,
        pool_name: *const libc::c_char,
        ioctx: *mut rados_ioctx_t,
    ) -> libc::c_int;

    /**
     * Create an io context
     *
     * The io context allows you to perform operations within a particular
     * pool. For more details see rados_ioctx_t.
     *
     * @param cluster which cluster the pool is in
     * @param pool_name name of the pool
     * @param ioctx where to store the io context
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_ioctx_create2(
        cluster: rados_t,
        pool_id: i64,
        ioctx: *mut rados_ioctx_t,
    ) -> libc::c_int;

    /**
     * The opposite of rados_ioctx_create
     *
     * This just tells librados that you no longer need to use the io context.
     * It may not be freed immediately if there are pending asynchronous
     * requests on it, but you should not use an io context again after
     * calling this function on it.
     *
     * @warning This does not guarantee any asynchronous
     * writes have completed. You must call rados_aio_flush()
     * on the io context before destroying it to do that.
     *
     * @warning If this ioctx is used by rados_watch, the caller needs to
     * be sure that all registered watches are disconnected via
     * rados_unwatch() and that rados_watch_flush() is called.  This
     * ensures that a racing watch callback does not make use of a
     * destroyed ioctx.
     *
     * @param io the io context to dispose of
     */
    pub fn rados_ioctx_destroy(io: rados_ioctx_t) -> ();

    /**
     * Get configuration handle for a pool handle
     *
     * @param io pool handle
     * @returns rados_config_t for this cluster
     */
    pub fn rados_ioctx_cct(io: rados_ioctx_t) -> rados_config_t;

    /**
     * Get the cluster handle used by this rados_ioctx_t
     * Note that this is a weak reference, and should not
     * be destroyed via rados_shutdown().
     *
     * @param io the io context
     * @returns the cluster handle for this io context
     */
    pub fn rados_ioctx_get_cluster(io: rados_ioctx_t) -> rados_t;

    /**
     * Get pool usage statistics
     *
     * Fills in a rados_pool_stat_t after querying the cluster.
     *
     * @param io determines which pool to query
     * @param stats where to store the results
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_ioctx_pool_stat(
        io: rados_ioctx_t,
        stats: *mut Struct_rados_pool_stat_t,
    ) -> libc::c_int;

    /**
     * Get the id of a pool
     *
     * @param cluster which cluster the pool is in
     * @param pool_name which pool to look up
     * @returns id of the pool
     * @returns -ENOENT if the pool is not found
     */
    pub fn rados_pool_lookup(cluster: rados_t, pool_name: *const libc::c_char) -> i64;

    /**
     * Get the name of a pool
     *
     * @param cluster which cluster the pool is in
     * @param id the id of the pool
     * @param buf where to store the pool name
     * @param maxlen size of buffer where name will be stored
     * @returns length of string stored, or -ERANGE if buffer too small
     */
    pub fn rados_pool_reverse_lookup(
        cluster: rados_t,
        id: i64,
        buf: *mut libc::c_char,
        maxlen: size_t,
    ) -> libc::c_int;

    /**
     * Create a pool with default settings
     *
     * The default crush rule is rule 0.
     *
     * @param cluster the cluster in which the pool will be created
     * @param pool_name the name of the new pool
     * @returns 0 on success, negative error code on failure
     *
     * # Note
     * Returns -17 if pool already exists.
     */
    pub fn rados_pool_create(cluster: rados_t, pool_name: *const libc::c_char) -> libc::c_int;

    /**
     * Create a pool with a specific CRUSH rule
     *
     * @param cluster the cluster in which the pool will be created
     * @param pool_name the name of the new pool
     * @param crush_rule_num which rule to use for placement in the new pool1
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_pool_create_with_crush_rule(
        cluster: rados_t,
        pool_name: *const libc::c_char,
        crush_rule_num: u8,
    ) -> libc::c_int;

    /**
     * Returns the pool that is the base tier for this pool.
     *
     * The return value is the ID of the pool that should be used to read from/write to.
     * If tiering is not set up for the pool, returns \c pool.
     *
     * @param cluster the cluster the pool is in
     * @param pool ID of the pool to query
     * @param base_tier \[out\] base tier, or \c pool if tiering is not configured
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_pool_get_base_tier(
        cluster: rados_t,
        pool: i64,
        base_tier: *mut i64,
    ) -> libc::c_int;

    /**
     * Delete a pool and all data inside it
     *
     * The pool is removed from the cluster immediately,
     * but the actual data is deleted in the background.
     *
     * @param cluster the cluster the pool is in
     * @param pool_name which pool to delete
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_pool_delete(cluster: rados_t, pool_name: *const libc::c_char) -> libc::c_int;

    /**
     * Test whether the specified pool requires alignment or not.
     *
     * @param io pool to query
     * @param req 1 if alignment is supported, 0 if not.
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_ioctx_pool_requires_alignment(io: rados_ioctx_t) -> libc::c_int;

    /**
     * Get the pool id of the io context
     *
     * @param io the io context to query
     * @returns the id of the pool the io context uses
     */
    pub fn rados_ioctx_get_id(io: rados_ioctx_t) -> i64;

    /**
     * Get the pool name of the io context
     *
     * @param io the io context to query
     * @param buf pointer to buffer where name will be stored
     * @param maxlen size of buffer where name will be stored
     * @returns length of string stored, or -ERANGE if buffer too small
     */
    pub fn rados_ioctx_get_pool_name(
        io: rados_ioctx_t,
        buf: *mut libc::c_char,
        maxlen: libc::c_uint,
    ) -> libc::c_int;

    /**
     * Set the key for mapping objects to pgs within an io context.
     *
     * The key is used instead of the object name to determine which
     * placement groups an object is put in. This affects all subsequent
     * operations of the io context - until a different locator key is
     * set, all objects in this io context will be placed in the same pg.
     *
     * @param io the io context to change
     * @param key the key to use as the object locator, or NULL to discard
     * any previously set key
     */
    pub fn rados_ioctx_locator_set_key(io: rados_ioctx_t, key: *const libc::c_char) -> ();

    /**
     * Set the namespace for objects within an io context
     *
     * The namespace specification further refines a pool into different
     * domains.  The mapping of objects to pgs is also based on this
     * value.
     *
     * @param io the io context to change
     * @param nspace the name to use as the namespace, or NULL use the
     * default namespace
     */
    pub fn rados_ioctx_set_namespace(io: rados_ioctx_t, nspace: *const libc::c_char) -> ();

    /**
     * Start listing objects in a pool
     *
     * @param io the pool to list from
     * @param ctx the handle to store list context in
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_nobjects_list_open(io: rados_ioctx_t, ctx: *mut rados_list_ctx_t) -> libc::c_int;

    /**
     * Return hash position of iterator, rounded to the current PG
     *
     * @param ctx iterator marking where you are in the listing
     * @returns current hash position, rounded to the current pg
     */
    pub fn rados_nobjects_list_get_pg_hash_position(ctx: rados_list_ctx_t) -> u32;

    /**
     * Reposition object iterator to a different hash position
     *
     * @param ctx iterator marking where you are in the listing
     * @param pos hash position to move to
     * @returns actual (rounded) position we moved to
     */
    pub fn rados_nobjects_list_seek(ctx: rados_list_ctx_t, pos: u32) -> u32;

    /**
     * Get the next object name and locator in the pool
     *
     * *entry and *key are valid until next call to rados_nobjects_list_*
     *
     * @param ctx iterator marking where you are in the listing
     * @param entry where to store the name of the entry
     * @param key where to store the object locator (set to NULL to ignore)
     * @param nspace where to store the object namespace (set to NULL to ignore)
     * @returns 0 on success, negative error code on failure
     * @returns -ENOENT when there are no more objects to list
     */
    pub fn rados_nobjects_list_next(
        ctx: rados_list_ctx_t,
        entry: *mut *mut *const libc::c_char,
        key: *mut *mut *const libc::c_char,
        nspace: *mut *mut *const libc::c_char,
    ) -> libc::c_int;

    /**
     * Close the object listing handle.
     *
     * This should be called when the handle is no longer needed.
     * The handle should not be used after it has been closed.
     *
     * @param ctx the handle to close
     */
    pub fn rados_nobjects_list_close(ctx: rados_list_ctx_t) -> ();

    /**
     * Create a pool-wide snapshot
     *
     * @param io the pool to snapshot
     * @param snapname the name of the snapshot
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_ioctx_snap_create(io: rados_ioctx_t, snapname: *const libc::c_char)
    -> libc::c_int;

    /**
     * Delete a pool snapshot
     *
     * @param io the pool to delete the snapshot from
     * @param snapname which snapshot to delete
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_ioctx_snap_remove(io: rados_ioctx_t, snapname: *const libc::c_char)
    -> libc::c_int;

    /**
     * Rollback an object to a pool snapshot
     *
     * The contents of the object will be the same as
     * when the snapshot was taken.
     *
     * @param io the pool in which the object is stored
     * @param oid the name of the object to rollback
     * @param snapname which snapshot to rollback to
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_ioctx_snap_rollback(
        io: rados_ioctx_t,
        oid: *const libc::c_char,
        snapname: *const libc::c_char,
    ) -> libc::c_int;

    /**
     * Set the snapshot from which reads are performed.
     *
     * Subsequent reads will return data as it was at the time of that
     * snapshot.
     *
     * @param io the io context to change
     * @param snap the id of the snapshot to set, or LIBRADOS_SNAP_HEAD for no
     * snapshot (i.e. normal operation)
     */
    pub fn rados_ioctx_snap_set_read(io: rados_ioctx_t, snap: rados_snap_t) -> ();

    /**
     * Allocate an ID for a self-managed snapshot
     *
     * Get a unique ID to put in the snaphot context to create a
     * snapshot. A clone of an object is not created until a write with
     * the new snapshot context is completed.
     *
     * @param io the pool in which the snapshot will exist
     * @param snapid where to store the newly allocated snapshot ID
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_ioctx_selfmanaged_snap_create(
        io: rados_ioctx_t,
        snapid: *mut rados_snap_t,
    ) -> libc::c_int;

    /**
     * Remove a self-managed snapshot
     *
     * This increases the snapshot sequence number, which will cause
     * snapshots to be removed lazily.
     *
     * @param io the pool in which the snapshot will exist
     * @param snapid where to store the newly allocated snapshot ID
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_ioctx_selfmanaged_snap_remove(
        io: rados_ioctx_t,
        snapid: rados_snap_t,
    ) -> libc::c_int;

    /**
     * Rollback an object to a self-managed snapshot
     *
     * The contents of the object will be the same as
     * when the snapshot was taken.
     *
     * @param io the pool in which the object is stored
     * @param oid the name of the object to rollback
     * @param snapid which snapshot to rollback to
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_ioctx_selfmanaged_snap_rollback(
        io: rados_ioctx_t,
        oid: *const libc::c_char,
        snapid: rados_snap_t,
    ) -> libc::c_int;

    /**
     * Set the snapshot context for use when writing to objects
     *
     * This is stored in the io context, and applies to all future writes.
     *
     * @param io the io context to change
     * @param seq the newest snapshot sequence number for the pool
     * @param snaps array of snapshots in sorted by descending id
     * @param num_snaps how many snaphosts are in the snaps array
     * @returns 0 on success, negative error code on failure
     * @returns -EINVAL if snaps are not in descending order
     */
    pub fn rados_ioctx_selfmanaged_snap_set_write_ctx(
        io: rados_ioctx_t,
        seq: rados_snap_t,
        snaps: *mut rados_snap_t,
        num_snaps: libc::c_int,
    ) -> libc::c_int;

    /**
     * List all the ids of pool snapshots
     *
     * If the output array does not have enough space to fit all the
     * snapshots, -ERANGE is returned and the caller should retry with a
     * larger array.
     *
     * @param io the pool to read from
     * @param snaps where to store the results
     * @param maxlen the number of rados_snap_t that fit in the snaps array
     * @returns number of snapshots on success, negative error code on failure
     * @returns -ERANGE is returned if the snaps array is too short
     */
    pub fn rados_ioctx_snap_list(
        io: rados_ioctx_t,
        snaps: *mut rados_snap_t,
        maxlen: libc::c_int,
    ) -> libc::c_int;

    /**
     * Get the id of a pool snapshot
     *
     * @param io the pool to read from
     * @param name the snapshot to find
     * @param id where to store the result
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_ioctx_snap_lookup(
        io: rados_ioctx_t,
        name: *const libc::c_char,
        id: *mut rados_snap_t,
    ) -> libc::c_int;

    /**
     * Get the name of a pool snapshot
     *
     * @param io the pool to read from
     * @param id the snapshot to find
     * @param name where to store the result
     * @param maxlen the size of the name array
     * @returns 0 on success, negative error code on failure
     * @returns -ERANGE if the name array is too small
     */
    pub fn rados_ioctx_snap_get_name(
        io: rados_ioctx_t,
        id: rados_snap_t,
        name: *mut libc::c_char,
        maxlen: libc::c_int,
    ) -> libc::c_int;

    /**
     * Find when a pool snapshot occurred
     *
     * @param io the pool the snapshot was taken in
     * @param id the snapshot to lookup
     * @param t where to store the result
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_ioctx_snap_get_stamp(
        io: rados_ioctx_t,
        id: rados_snap_t,
        t: *mut time_t,
    ) -> libc::c_int;

    /**
     * Return the version of the last object read or written to.
     *
     * This exposes the internal version number of the last object read or
     * written via this io context
     *
     * @param io the io context to check
     * @returns last read or written object version
     */
    pub fn rados_get_last_version(io: rados_ioctx_t) -> u64;

    /**
     * Write *len* bytes from *buf* into the *oid* object, starting at
     * offset *off*. The value of *len* must be <= UINT_MAX/2.
     *
     * @note This will never return a positive value not equal to len.
     * @param io the io context in which the write will occur
     * @param oid name of the object
     * @param buf data to write
     * @param len length of the data, in bytes
     * @param off byte offset in the object to begin writing at
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_write(
        io: rados_ioctx_t,
        oid: *const libc::c_char,
        buf: *const libc::c_char,
        len: size_t,
        off: u64,
    ) -> libc::c_int;

    /**
     * Write *len* bytes from *buf* into the *oid* object. The value of
     * *len* must be <= UINT_MAX/2.
     *
     * The object is filled with the provided data. If the object exists,
     * it is atomically truncated and then written.
     *
     * @param io the io context in which the write will occur
     * @param oid name of the object
     * @param buf data to write
     * @param len length of the data, in bytes
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_write_full(
        io: rados_ioctx_t,
        oid: *const libc::c_char,
        buf: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;

    /**
     * Write the same *data_len* bytes from *buf* multiple times into the
     * *oid* object. *write_len* bytes are written in total, which must be
     * a multiple of *data_len*. The value of *write_len* and *data_len*
     * must be <= UINT_MAX/2.
     *
     * @param io the io context in which the write will occur
     * @param oid name of the object
     * @param buf data to write
     * @param data_len length of the data, in bytes
     * @param write_len the total number of bytes to write
     * @param off byte offset in the object to begin writing at
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_writesame(
        io: rados_ioctx_t,
        oid: *const libc::c_char,
        buf: *const libc::c_char,
        data_len: size_t,
        write_len: size_t,
        off: u64,
    ) -> libc::c_int;

    /**
     * Append *len* bytes from *buf* into the *oid* object. The value of
     * *len* must be <= UINT_MAX/2.
     *
     * @param io the context to operate in
     * @param oid the name of the object
     * @param buf the data to append
     * @param len length of buf (in bytes)
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_append(
        io: rados_ioctx_t,
        oid: *const libc::c_char,
        buf: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;

    /**
     * Read data from an object
     *
     * The io context determines the snapshot to read from, if any was set
     * by rados_ioctx_snap_set_read().
     *
     * @param io the context in which to perform the read
     * @param oid the name of the object to read from
     * @param buf where to store the results
     * @param len the number of bytes to read
     * @param off the offset to start reading from in the object
     * @returns number of bytes read on success, negative error code on
     * failure
     */
    pub fn rados_read(
        io: rados_ioctx_t,
        oid: *const libc::c_char,
        buf: *mut libc::c_char,
        len: size_t,
        off: u64,
    ) -> libc::c_int;

    /**
     * Delete an object
     *
     * @note This does not delete any snapshots of the object.
     *
     * @param io the pool to delete the object from
     * @param oid the name of the object to delete
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_remove(io: rados_ioctx_t, oid: *const libc::c_char) -> libc::c_int;

    /**
     * Resize an object
     *
     * If this enlarges the object, the new area is logically filled with
     * zeroes. If this shrinks the object, the excess data is removed.
     *
     * @param io the context in which to truncate
     * @param oid the name of the object
     * @param size the new size of the object in bytes
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_trunc(io: rados_ioctx_t, oid: *const libc::c_char, size: u64) -> libc::c_int;

    /**
     * Get the value of an extended attribute on an object.
     *
     * @param io the context in which the attribute is read
     * @param o name of the object
     * @param name which extended attribute to read
     * @param buf where to store the result
     * @param len size of buf in bytes
     * @returns length of xattr value on success, negative error code on failure
     */
    pub fn rados_getxattr(
        io: rados_ioctx_t,
        o: *const libc::c_char,
        name: *const libc::c_char,
        buf: *mut libc::c_char,
        len: size_t,
    ) -> libc::c_int;

    /**
     * Set an extended attribute on an object.
     *
     * @param io the context in which xattr is set
     * @param o name of the object
     * @param name which extended attribute to set
     * @param buf what to store in the xattr
     * @param len the number of bytes in buf
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_setxattr(
        io: rados_ioctx_t,
        o: *const libc::c_char,
        name: *const libc::c_char,
        buf: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;

    /**
     * Delete an extended attribute from an object.
     *
     * @param io the context in which to delete the xattr
     * @param o the name of the object
     * @param name which xattr to delete
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_rmxattr(
        io: rados_ioctx_t,
        o: *const libc::c_char,
        name: *const libc::c_char,
    ) -> libc::c_int;

    /**
     * Start iterating over xattrs on an object.
     *
     * @post iter is a valid iterator
     *
     * @param io the context in which to list xattrs
     * @param oid name of the object
     * @param iter where to store the iterator
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_getxattrs(
        io: rados_ioctx_t,
        oid: *const libc::c_char,
        iter: *mut rados_xattrs_iter_t,
    ) -> libc::c_int;

    /**
     * Get the next xattr on the object
     *
     * @pre iter is a valid iterator
     *
     * @post name is the NULL-terminated name of the next xattr, and val
     * contains the value of the xattr, which is of length len. If the end
     * of the list has been reached, name and val are NULL, and len is 0.
     *
     * @param iter iterator to advance
     * @param name where to store the name of the next xattr
     * @param val where to store the value of the next xattr
     * @param len the number of bytes in val
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_getxattrs_next(
        iter: rados_xattrs_iter_t,
        name: *mut *const libc::c_char,
        val: *mut *const libc::c_char,
        len: *mut size_t,
    ) -> libc::c_int;

    /**
     * Close the xattr iterator.
     *
     * iter should not be used after this is called.
     *
     * @param iter the iterator to close
     */
    pub fn rados_getxattrs_end(iter: rados_xattrs_iter_t) -> ();

    /**
     * Get the next omap key/value pair on the object
     *
     * @pre iter is a valid iterator
     *
     * @post key and val are the next key/value pair. key is
     * null-terminated, and val has length len. If the end of the list has
     * been reached, key and val are NULL, and len is 0. key and val will
     * not be accessible after rados_omap_get_end() is called on iter, so
     * if they are needed after that they should be copied.
     *
     * @param iter iterator to advance
     * @param key where to store the key of the next omap entry
     * @param val where to store the value of the next omap entry
     * @param len where to store the number of bytes in val
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_omap_get_next(
        iter: rados_omap_iter_t,
        key: *mut *mut libc::c_char,
        val: *mut *mut libc::c_char,
        len: *mut size_t,
    ) -> libc::c_int;

    /**
     * Close the omap iterator.
     *
     * iter should not be used after this is called.
     *
     * @param iter the iterator to close
     */
    pub fn rados_omap_get_end(iter: rados_omap_iter_t) -> ();

    /**
     * Get object size and most recent update time from the OSD.
     *
     * @param io ioctx
     * @param o object name
     * @param psize where to store object size
     * @param pmtime where to store modification time
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_stat(
        io: rados_ioctx_t,
        o: *const libc::c_char,
        psize: *mut u64,
        pmtime: *mut time_t,
    ) -> libc::c_int;

    /**
     * Execute an OSD class method on an object
     *
     * The OSD has a plugin mechanism for performing complicated
     * operations on an object atomically. These plugins are called
     * classes. This function allows librados users to call the custom
     * methods. The input and output formats are defined by the class.
     * Classes in ceph.git can be found in src/cls subdirectories
     *
     * @param io the context in which to call the method
     * @param oid the object to call the method on
     * @param cls the name of the class
     * @param method the name of the method
     * @param in_buf where to find input
     * @param in_len length of in_buf in bytes
     * @param buf where to store output
     * @param out_len length of buf in bytes
     * @returns the length of the output, or
     * -ERANGE if out_buf does not have enough space to store it (For methods that return data). For
     * methods that don't return data, the return value is
     * method-specific.
     */
    pub fn rados_exec(
        io: rados_ioctx_t,
        oid: *const libc::c_char,
        cls: *const libc::c_char,
        method: *const libc::c_char,
        in_buf: *const libc::c_char,
        in_len: size_t,
        buf: *mut libc::c_char,
        out_len: size_t,
    ) -> libc::c_int;

    /**
     * Constructs a completion to use with asynchronous operations
     *
     * The complete and safe callbacks correspond to operations being
     * acked and committed, respectively. The callbacks are called in
     * order of receipt, so the safe callback may be triggered before the
     * complete callback, and vice versa. This is affected by journalling
     * on the OSDs.
     *
     * TODO: more complete documentation of this elsewhere (in the RADOS docs?)
     *
     * @note Read operations only get a complete callback.
     * @note BUG: this should check for ENOMEM instead of throwing an exception
     *
     * @param cb_arg application-defined data passed to the callback functions
     * @param cb_complete the function to be called when the operation is
     * in memory on all replicas
     * @param cb_safe the function to be called when the operation is on
     * stable storage on all replicas
     * @param pc where to store the completion
     * @returns 0
     */
    pub fn rados_aio_create_completion(
        cb_arg: *mut std::os::raw::c_void,
        cb_complete: rados_callback_t,
        cb_safe: rados_callback_t,
        pc: *mut rados_completion_t,
    ) -> libc::c_int;

    /**
     * Block until an operation completes
     *
     * This means it is in memory on all replicas.
     *
     * @note BUG: this should be void
     *
     * @param c operation to wait for
     * @returns 0
     */
    pub fn rados_aio_wait_for_complete(c: rados_completion_t) -> libc::c_int;

    /**
     * Has an asynchronous operation completed?
     *
     * @warning This does not imply that the complete callback has
     * finished
     *
     * @param c async operation to inspect
     * @returns whether c is complete
     */
    pub fn rados_aio_is_complete(c: rados_completion_t) -> libc::c_int;

    /**
     * Is an asynchronous operation safe?
     *
     * @warning This does not imply that the safe callback has
     * finished
     *
     * @param c async operation to inspect
     * @returns whether c is safe
     */
    pub fn rados_aio_is_safe(c: rados_completion_t) -> libc::c_int;

    /**
     * Block until an operation completes and callback completes
     *
     * This means it is in memory on all replicas and can be read.
     *
     * @note BUG: this should be void
     *
     * @param c operation to wait for
     * @returns 0
     */
    pub fn rados_aio_wait_for_complete_and_cb(c: rados_completion_t) -> libc::c_int;

    /**
     * Has an asynchronous operation and callback completed
     *
     * @param c async operation to inspect
     * @returns whether c is complete
     */
    pub fn rados_aio_is_complete_and_cb(c: rados_completion_t) -> libc::c_int;

    /**
     * Is an asynchronous operation safe and has the callback completed
     *
     * @param c async operation to inspect
     * @returns whether c is safe
     */
    pub fn rados_aio_is_safe_and_cb(c: rados_completion_t) -> libc::c_int;

    /**
     * Get the return value of an asychronous operation
     *
     * The return value is set when the operation is complete or safe,
     * whichever comes first.
     *
     * @pre The operation is safe or complete
     *
     * @note BUG: complete callback may never be called when the safe
     * message is received before the complete message
     *
     * @param c async operation to inspect
     * @returns return value of the operation
     */
    pub fn rados_aio_get_return_value(c: rados_completion_t) -> libc::c_int;

    /**
     * Release a completion
     *
     * Call this when you no longer need the completion. It may not be
     * freed immediately if the operation is not acked and committed.
     *
     * @param c completion to release
     */
    pub fn rados_aio_release(c: rados_completion_t) -> ();

    /**
     * Write data to an object asynchronously
     *
     * Queues the write and returns. The return value of the completion
     * will be 0 on success, negative error code on failure.
     *
     * @param io the context in which the write will occur
     * @param oid name of the object
     * @param completion what to do when the write is safe and complete
     * @param buf data to write
     * @param len length of the data, in bytes
     * @param off byte offset in the object to begin writing at
     * @returns 0 on success, -EROFS if the io context specifies a snap_seq
     * other than LIBRADOS_SNAP_HEAD
     */
    pub fn rados_aio_write(
        io: rados_ioctx_t,
        oid: *const libc::c_char,
        completion: rados_completion_t,
        buf: *const libc::c_char,
        len: size_t,
        off: u64,
    ) -> libc::c_int;

    /**
     * Asynchronously append data to an object
     *
     * Queues the append and returns.
     *
     * The return value of the completion will be 0 on success, negative
     * error code on failure.
     *
     * @param io the context to operate in
     * @param oid the name of the object
     * @param completion what to do when the append is safe and complete
     * @param buf the data to append
     * @param len length of buf (in bytes)
     * @returns 0 on success, -EROFS if the io context specifies a snap_seq
     * other than LIBRADOS_SNAP_HEAD
     */
    pub fn rados_aio_append(
        io: rados_ioctx_t,
        oid: *const libc::c_char,
        completion: rados_completion_t,
        buf: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;

    /**
     * Asynchronously write an entire object
     *
     * The object is filled with the provided data. If the object exists,
     * it is atomically truncated and then written.
     * Queues the write_full and returns.
     *
     * The return value of the completion will be 0 on success, negative
     * error code on failure.
     *
     * @param io the io context in which the write will occur
     * @param oid name of the object
     * @param completion what to do when the write_full is safe and complete
     * @param buf data to write
     * @param len length of the data, in bytes
     * @returns 0 on success, -EROFS if the io context specifies a snap_seq
     * other than LIBRADOS_SNAP_HEAD
     */
    pub fn rados_aio_write_full(
        io: rados_ioctx_t,
        oid: *const libc::c_char,
        completion: rados_completion_t,
        buf: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;

    /**
     * Asynchronously write the same buffer multiple times
     *
     * Queues the writesame and returns.
     *
     * The return value of the completion will be 0 on success, negative
     * error code on failure.
     *
     * @param io the io context in which the write will occur
     * @param oid name of the object
     * @param completion what to do when the writesame is safe and complete
     * @param buf data to write
     * @param data_len length of the data, in bytes
     * @param write_len the total number of bytes to write
     * @param off byte offset in the object to begin writing at
     * @returns 0 on success, -EROFS if the io context specifies a snap_seq
     * other than LIBRADOS_SNAP_HEAD
     */
    pub fn rados_aio_writesame(
        io: rados_ioctx_t,
        oid: *const libc::c_char,
        completion: rados_completion_t,
        buf: *const libc::c_char,
        data_len: libc::size_t,
        write_len: libc::size_t,
        off: u64,
    ) -> libc::c_int;

    /**
     * Asynchronously remove an object
     *
     * Queues the remove and returns.
     *
     * The return value of the completion will be 0 on success, negative
     * error code on failure.
     *
     * @param io the context to operate in
     * @param oid the name of the object
     * @param completion what to do when the remove is safe and complete
     * @returns 0 on success, -EROFS if the io context specifies a snap_seq
     * other than LIBRADOS_SNAP_HEAD
     */
    pub fn rados_aio_remove(
        io: rados_ioctx_t,
        oid: *const libc::c_char,
        completion: rados_completion_t,
    ) -> libc::c_int;

    /**
     * Asynchronously read data from an object
     *
     * The io context determines the snapshot to read from, if any was set
     * by rados_ioctx_snap_set_read().
     *
     * The return value of the completion will be number of bytes read on
     * success, negative error code on failure.
     *
     * @note only the 'complete' callback of the completion will be called.
     *
     * @param io the context in which to perform the read
     * @param oid the name of the object to read from
     * @param completion what to do when the read is complete
     * @param buf where to store the results
     * @param len the number of bytes to read
     * @param off the offset to start reading from in the object
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_aio_read(
        io: rados_ioctx_t,
        oid: *const libc::c_char,
        completion: rados_completion_t,
        buf: *mut libc::c_char,
        len: size_t,
        off: u64,
    ) -> libc::c_int;

    /**
     * Block until all pending writes in an io context are safe
     *
     * This is not equivalent to calling rados_aio_wait_for_safe() on all
     * write completions, since this waits for the associated callbacks to
     * complete as well.
     *
     * @note BUG: always returns 0, should be void or accept a timeout
     *
     * @param io the context to flush
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_aio_flush(io: rados_ioctx_t) -> libc::c_int;

    /**
     * Schedule a callback for when all currently pending
     * aio writes are safe. This is a non-blocking version of
     * rados_aio_flush().
     *
     * @param io the context to flush
     * @param completion what to do when the writes are safe
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_aio_flush_async(io: rados_ioctx_t, completion: rados_completion_t) -> libc::c_int;

    /**
     * Asynchronously get object stats (size/mtime)
     *
     * @param io ioctx
     * @param o object name
     * @param completion what to do when the stat is complete
     * @param psize where to store object size
     * @param pmtime where to store modification time
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_aio_stat(
        io: rados_ioctx_t,
        o: *const libc::c_char,
        completion: rados_completion_t,
        psize: *mut u64,
        pmtime: *mut time_t,
    ) -> libc::c_int;

    /**
     * Cancel async operation
     *
     * @param io ioctx
     * @param completion completion handle
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_aio_cancel(io: rados_ioctx_t, completion: rados_completion_t) -> libc::c_int;

    /**
     * Register an interest in an object
     *
     * A watch operation registers the client as being interested in
     * notifications on an object. OSDs keep track of watches on
     * persistent storage, so they are preserved across cluster changes by
     * the normal recovery process. If the client loses its connection to the
     * primary OSD for a watched object, the watch will be removed after
     * a timeout configured with osd_client_watch_timeout.
     * Watches are automatically reestablished when a new
     * connection is made, or a placement group switches OSDs.
     *
     * @param io the pool the object is in
     * @param o the object to watch
     * @param cookie where to store the internal id assigned to this watch
     * @param watchcb what to do when a notify is received on this object
     * @param watcherrcb what to do when the watch session encounters an error
     * @param arg opaque value to pass to the callback
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_watch2(
        io: rados_ioctx_t,
        o: *const libc::c_char,
        cookie: *mut u64,
        watchcb: rados_watchcb2_t,
        watcherrcb: rados_watcherrcb_t,
        arg: *mut std::os::raw::c_void,
    ) -> libc::c_int;

    /**
     * Check on the status of a watch
     *
     * Return the number of milliseconds since the watch was last confirmed.
     * Or, if there has been an error, return that.
     *
     * If there is an error, the watch is no longer valid, and should be
     * destroyed with rados_unwatch2().  The the user is still interested
     * in the object, a new watch should be created with rados_watch2().
     *
     * @param io the pool the object is in
     * @param cookie the watch handle
     * @returns ms since last confirmed on success, negative error code on failure
     */
    pub fn rados_watch_check(io: rados_ioctx_t, cookie: u64) -> libc::c_int;

    /**
     * Unregister an interest in an object
     *
     * Once this completes, no more notifies will be sent to us for this
     * watch. This should be called to clean up unneeded watchers.
     *
     * @param io the pool the object is in
     * @param cookie which watch to unregister
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_unwatch2(io: rados_ioctx_t, cookie: u64) -> libc::c_int;

    /**
     * Sychronously notify watchers of an object
     *
     * This blocks until all watchers of the object have received and
     * reacted to the notify, or a timeout is reached.
     *
     * The reply buffer is optional.  If specified, the client will get
     * back an encoded buffer that includes the ids of the clients that
     * acknowledged the notify as well as their notify ack payloads (if
     * any).  Clients that timed out are not included.  Even clients that
     * do not include a notify ack payload are included in the list but
     * have a 0-length payload associated with them.  The format:
     * ```graph
     *    le32 num_acks
     *    {
     *      le64 gid     global id for the client (for client.1234 that's 1234)
     *      le64 cookie  cookie for the client
     *      le32 buflen  length of reply message buffer
     *      u8 * buflen  payload
     *    } * num_acks
     *    le32 num_timeouts
     *    {
     *      le64 gid     global id for the client
     *      le64 cookie  cookie for the client
     *    } * num_timeouts
     * ```
     * Note: There may be multiple instances of the same gid if there are
     * multiple watchers registered via the same client.
     *
     * Note: The buffer must be released with rados_buffer_free() when the
     * user is done with it.
     *
     * Note: Since the result buffer includes clients that time out, it
     * will be set even when rados_notify() returns an error code (like
     * -ETIMEDOUT).
     *
     * @param io the pool the object is in
     * @param o the name of the object
     * @param buf data to send to watchers
     * @param buf_len length of buf in bytes
     * @param timeout_ms notify timeout (in ms)
     * @param reply_buffer pointer to reply buffer pointer (free with rados_buffer_free)
     * @param reply_buffer_len pointer to size of reply buffer
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_notify2(
        io: rados_ioctx_t,
        o: *const libc::c_char,
        buf: *const libc::c_char,
        buf_len: libc::c_int,
        timeout_ms: u64,
        reply_buffer: *mut *mut libc::c_char,
        reply_buffer_len: *mut size_t,
    ) -> libc::c_int;

    /**
     * Acknolwedge receipt of a notify
     *
     * @param io the pool the object is in
     * @param o the name of the object
     * @param notify_id the notify_id we got on the watchcb2_t callback
     * @param cookie the watcher handle
     * @param buf payload to return to notifier (optional)
     * @param buf_len payload length
     * @returns 0 on success
     */
    pub fn rados_notify_ack(
        io: rados_ioctx_t,
        o: *const libc::c_char,
        notify_id: u64,
        cookie: u64,
        buf: *const libc::c_char,
        buf_len: libc::c_int,
    ) -> libc::c_int;

    /**
     * Flush watch/notify callbacks
     *
     * This call will block until all pending watch/notify callbacks have
     * been executed and the queue is empty.  It should usually be called
     * after shutting down any watches before shutting down the ioctx or
     * librados to ensure that any callbacks do not misuse the ioctx (for
     * example by calling rados_notify_ack after the ioctx has been
     * destroyed).
     *
     * @param cluster the cluster handle
     */
    pub fn rados_watch_flush(cluster: rados_t) -> libc::c_int;

    /**
     * Set allocation hint for an object
     *
     * This is an advisory operation, it will always succeed (as if it was
     * submitted with a LIBRADOS_OP_FLAG_FAILOK flag set) and is not
     * guaranteed to do anything on the backend.
     *
     * @param io the pool the object is in
     * @param o the name of the object
     * @param expected_object_size expected size of the object, in bytes
     * @param expected_write_size expected size of writes to the object, in bytes
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_set_alloc_hint(
        io: rados_ioctx_t,
        o: *const libc::c_char,
        expected_object_size: u64,
        expected_write_size: u64,
    ) -> libc::c_int;

    /**
     * Create a new rados_write_op_t write operation. This will store all actions
     * to be performed atomically. You must call rados_release_write_op when you are
     * finished with it.
     *
     * @note the ownership of a write operartion is passed to the function
     *       performing the operation, so the same instance of @c rados_write_op_t
     *       cannot be used again after being performed.
     *
     * @returns non-NULL on success, NULL on memory allocation error.
     */
    pub fn rados_create_write_op() -> rados_write_op_t;

    /**
     * Free a rados_write_op_t, must be called when you're done with it.
     * @param write_op operation to deallocate, created with rados_create_write_op
     */
    pub fn rados_release_write_op(write_op: rados_write_op_t) -> ();

    /**
     * Set flags for the last operation added to this write_op.
     * At least one op must have been added to the write_op.
     * @param write_op operation to add this action to
     * @param flags see librados.h constants beginning with LIBRADOS_OP_FLAG
     */
    pub fn rados_write_op_set_flags(write_op: rados_write_op_t, flags: libc::c_int) -> ();

    /**
     * Ensure that the object exists before writing
     * @param write_op operation to add this action to
     */
    pub fn rados_write_op_assert_exists(write_op: rados_write_op_t) -> ();

    /**
     * Ensure that the object exists and that its internal version
     * number is equal to "ver" before writing. "ver" should be a
     * version number previously obtained with rados_get_last_version().
     * - If the object's version is greater than the asserted version
     *   then rados_write_op_operate will return -ERANGE instead of
     *   executing the op.
     * - If the object's version is less than the asserted version
     *   then rados_write_op_operate will return -EOVERFLOW instead
     *   of executing the op.
     *
     * @param write_op operation to add this action to
     * @param ver object version number
     */
    pub fn rados_write_op_assert_version(write_op: rados_write_op_t, ver: u64) -> ();

    /**
     * Ensure that given xattr satisfies comparison.
     * If the comparison is not satisfied, the return code of the
     * operation will be -ECANCELED
     * @param write_op operation to add this action to
     * @param name name of the xattr to look up
     * @param comparison_operator currently undocumented, look for
     * LIBRADOS_CMPXATTR_OP_EQ in librados.h
     * @param value buffer to compare actual xattr value to
     * @param value_len length of buffer to compare actual xattr value to
     */
    pub fn rados_write_op_cmpxattr(
        write_op: rados_write_op_t,
        name: *const libc::c_char,
        comparison_operator: u8,
        value: *const libc::c_char,
        value_len: size_t,
    ) -> ();

    /**
    * Ensure that the an omap value satisfies a comparison,
    * with the supplied value on the right hand side (i.e.
    * for OP_LT, the comparison is actual_value < value.
    *
    * @param write_op operation to add this action to
    * @param key which omap value to compare
    * @param comparison_operator one of LIBRADOS_CMPXATTR_OP_EQ,
      LIBRADOS_CMPXATTR_OP_LT, or LIBRADOS_CMPXATTR_OP_GT
    * @param val value to compare with
    * @param val_len length of value in bytes
    * @param prval where to store the return value from this action
    */
    pub fn rados_write_op_omap_cmp(
        write_op: rados_write_op_t,
        key: *const libc::c_char,
        comparison_operator: u8,
        val: *const libc::c_char,
        val_len: size_t,
        prval: *mut libc::c_int,
    ) -> ();

    /**
     * Set an xattr
     * @param write_op operation to add this action to
     * @param name name of the xattr
     * @param value buffer to set xattr to
     * @param value_len length of buffer to set xattr to
     */
    pub fn rados_write_op_setxattr(
        write_op: rados_write_op_t,
        name: *const libc::c_char,
        value: *const libc::c_char,
        value_len: size_t,
    ) -> ();

    /**
     * Remove an xattr
     * @param write_op operation to add this action to
     * @param name name of the xattr to remove
     */
    pub fn rados_write_op_rmxattr(write_op: rados_write_op_t, name: *const libc::c_char) -> ();

    /**
    * Create the object
    * @param write_op operation to add this action to
    * @param exclusive set to either LIBRADOS_CREATE_EXCLUSIVE or
      LIBRADOS_CREATE_IDEMPOTENT
    * will error if the object already exists.
    * @param category category string (DEPRECATED, HAS NO EFFECT)
    */
    pub fn rados_write_op_create(
        write_op: rados_write_op_t,
        exclusive: libc::c_int,
        category: *const libc::c_char,
    ) -> ();

    /**
     * Write to offset
     * @param write_op operation to add this action to
     * @param offset offset to write to
     * @param buffer bytes to write
     * @param len length of buffer
     */
    pub fn rados_write_op_write(
        write_op: rados_write_op_t,
        buffer: *const libc::c_char,
        len: size_t,
        offset: u64,
    ) -> ();

    /**
     * Write whole object, atomically replacing it.
     * @param write_op operation to add this action to
     * @param buffer bytes to write
     * @param len length of buffer
     */
    pub fn rados_write_op_write_full(
        write_op: rados_write_op_t,
        buffer: *const libc::c_char,
        len: size_t,
    ) -> ();

    /**
     * Append to end of object.
     * @param write_op operation to add this action to
     * @param buffer bytes to write
     * @param len length of buffer
     */
    pub fn rados_write_op_append(
        write_op: rados_write_op_t,
        buffer: *const libc::c_char,
        len: size_t,
    ) -> ();

    /**
     * Remove object
     * @param write_op operation to add this action to
     */
    pub fn rados_write_op_remove(write_op: rados_write_op_t) -> ();

    /**
     * Truncate an object
     * @param write_op operation to add this action to
     * @param offset Offset to truncate to
     */
    pub fn rados_write_op_truncate(write_op: rados_write_op_t, offset: u64) -> ();

    /**
     * Zero part of an object
     * @param write_op operation to add this action to
     * @param offset Offset to zero
     * @param len length to zero
     */
    pub fn rados_write_op_zero(write_op: rados_write_op_t, offset: u64, len: u64) -> ();

    /**
     * Execute an OSD class method on an object
     * See rados_exec() for general description.
     *
     * @param write_op operation to add this action to
     * @param cls the name of the class
     * @param method the name of the method
     * @param in_buf where to find input
     * @param in_len length of in_buf in bytes
     * @param prval where to store the return value from the method
     */
    pub fn rados_write_op_exec(
        write_op: rados_write_op_t,
        cls: *const libc::c_char,
        method: *const libc::c_char,
        in_buf: *const libc::c_char,
        in_len: size_t,
        prval: *mut libc::c_int,
    ) -> ();

    /**
     * Set key/value pairs on an object
     *
     * @param write_op operation to add this action to
     * @param keys array of null-terminated char arrays representing keys to set
     * @param vals array of pointers to values to set
     * @param lens array of lengths corresponding to each value
     * @param num number of key/value pairs to set
     */
    pub fn rados_write_op_omap_set(
        write_op: rados_write_op_t,
        keys: *const *const libc::c_char,
        vals: *const *const libc::c_char,
        lens: *const size_t,
        num: size_t,
    ) -> ();

    /**
     * Remove key/value pairs from an object
     *
     * @param write_op operation to add this action to
     * @param keys array of null-terminated char arrays representing keys to remove
     * @param keys_len number of key/value pairs to remove
     */
    pub fn rados_write_op_omap_rm_keys(
        write_op: rados_write_op_t,
        keys: *const *const libc::c_char,
        keys_len: size_t,
    ) -> ();

    /**
     * Remove all key/value pairs from an object
     *
     * @param write_op operation to add this action to
     */
    pub fn rados_write_op_omap_clear(write_op: rados_write_op_t) -> ();

    /**
     * Set allocation hint for an object
     *
     * @param write_op operation to add this action to
     * @param expected_object_size expected size of the object, in bytes
     * @param expected_write_size expected size of writes to the object, in bytes
     */
    pub fn rados_write_op_set_alloc_hint(
        write_op: rados_write_op_t,
        expected_object_size: u64,
        expected_write_size: u64,
    ) -> ();

    /**
     * Perform a write operation synchronously
     * @param write_op operation to perform
     * @param io the ioctx that the object is in
     * @param oid the object id
     * @param mtime the time to set the mtime to, NULL for the current time
     * @param flags flags to apply to the entire operation (LIBRADOS_OPERATION_*)
     */
    pub fn rados_write_op_operate(
        write_op: rados_write_op_t,
        io: rados_ioctx_t,
        oid: *const libc::c_char,
        mtime: *mut time_t,
        flags: libc::c_int,
    ) -> libc::c_int;

    /**
     * Perform a write operation asynchronously
     * @param write_op operation to perform
     * @param io the ioctx that the object is in
     * @param completion what to do when operation has been attempted
     * @param oid the object id
     * @param mtime the time to set the mtime to, NULL for the current time
     * @param flags flags to apply to the entire operation (LIBRADOS_OPERATION_*)
     */
    pub fn rados_aio_write_op_operate(
        write_op: rados_write_op_t,
        io: rados_ioctx_t,
        completion: rados_completion_t,
        oid: *const libc::c_char,
        mtime: *mut time_t,
        flags: libc::c_int,
    ) -> libc::c_int;

    /**
     * Create a new rados_read_op_t read operation. This will store all
     * actions to be performed atomically. You must call
     * rados_release_read_op when you are finished with it (after it
     * completes, or you decide not to send it in the first place).
     *
     * @note the ownership of a read operartion is passed to the function
     *       performing the operation, so the same instance of @c rados_read_op_t
     *       cannot be used again after being performed.
     *
     * @returns non-NULL on success, NULL on memory allocation error.
     */
    pub fn rados_create_read_op() -> rados_read_op_t;

    /**
     * Free a rados_read_op_t, must be called when you're done with it.
     * @param read_op operation to deallocate, created with rados_create_read_op
     */
    pub fn rados_release_read_op(read_op: rados_read_op_t) -> ();

    /**
     * Set flags for the last operation added to this read_op.
     * At least one op must have been added to the read_op.
     * @param read_op operation to add this action to
     * @param flags see librados.h constants beginning with LIBRADOS_OP_FLAG
     */
    pub fn rados_read_op_set_flags(read_op: rados_read_op_t, flags: libc::c_int) -> ();

    /**
     * Ensure that the object exists before reading
     * @param read_op operation to add this action to
     */
    pub fn rados_read_op_assert_exists(read_op: rados_read_op_t) -> ();

    /**
     * Ensure that the object exists and that its internal version
     * number is equal to "ver" before reading. "ver" should be a
     * version number previously obtained with rados_get_last_version().
     * - If the object's version is greater than the asserted version
     *   then rados_read_op_operate will return -ERANGE instead of
     *   executing the op.
     * - If the object's version is less than the asserted version
     *   then rados_read_op_operate will return -EOVERFLOW instead
     *   of executing the op.
     *
     * @param read_op operation to add this action to
     * @param ver object version number
     */
    pub fn rados_read_op_assert_version(write_op: rados_read_op_t, ver: u64) -> ();

    /**
     * Ensure that the an xattr satisfies a comparison
     * If the comparison is not satisfied, the return code of the
     * operation will be -ECANCELED
     * @param read_op operation to add this action to
     * @param name name of the xattr to look up
     * @param comparison_operator currently undocumented, look for
     * LIBRADOS_CMPXATTR_OP_EQ in librados.h
     * @param value buffer to compare actual xattr value to
     * @param value_len length of buffer to compare actual xattr value to
     */
    pub fn rados_read_op_cmpxattr(
        read_op: rados_read_op_t,
        name: *const libc::c_char,
        comparison_operator: u8,
        value: *const libc::c_char,
        value_len: size_t,
    ) -> ();

    /**
     * Start iterating over xattrs on an object.
     *
     * @param read_op operation to add this action to
     * @param iter where to store the iterator
     * @param prval where to store the return value of this action
     */
    pub fn rados_read_op_getxattrs(
        read_op: rados_read_op_t,
        iter: *mut rados_xattrs_iter_t,
        prval: *mut libc::c_int,
    ) -> ();

    /**
    * Ensure that the an omap value satisfies a comparison,
    * with the supplied value on the right hand side (i.e.
    * for OP_LT, the comparison is actual_value < value.
    *
    * @param read_op operation to add this action to
    * @param key which omap value to compare
    * @param comparison_operator one of LIBRADOS_CMPXATTR_OP_EQ,
      LIBRADOS_CMPXATTR_OP_LT, or LIBRADOS_CMPXATTR_OP_GT
    * @param val value to compare with
    * @param val_len length of value in bytes
    * @param prval where to store the return value from this action
    */
    pub fn rados_read_op_omap_cmp(
        read_op: rados_read_op_t,
        key: *const libc::c_char,
        comparison_operator: u8,
        val: *const libc::c_char,
        val_len: size_t,
        prval: *mut libc::c_int,
    ) -> ();

    /**
     * Get object size and mtime
     * @param read_op operation to add this action to
     * @param psize where to store object size
     * @param pmtime where to store modification time
     * @param prval where to store the return value of this action
     */
    pub fn rados_read_op_stat(
        read_op: rados_read_op_t,
        psize: *mut u64,
        pmtime: *mut time_t,
        prval: *mut libc::c_int,
    ) -> ();

    /**
     * Read bytes from offset into buffer.
     *
     * prlen will be filled with the number of bytes read if successful.
     * A short read can only occur if the read reaches the end of the
     * object.
     *
     * @param read_op operation to add this action to
     * @param offset offset to read from
     * @param len length of buffer
     * @param buffer where to put the data
     * @param bytes_read where to store the number of bytes read by this action
     * @param prval where to store the return value of this action
     */
    pub fn rados_read_op_read(
        read_op: rados_read_op_t,
        offset: u64,
        len: size_t,
        buf: *mut libc::c_char,
        bytes_read: *mut size_t,
        prval: *mut libc::c_int,
    ) -> ();

    /**
     * Execute an OSD class method on an object
     * See rados_exec() for general description.
     *
     * The output buffer is allocated on the heap; the caller is
     * expected to release that memory with rados_buffer_free(). The
     * buffer and length pointers can all be NULL, in which case they are
     * not filled in.
     *
     * @param read_op operation to add this action to
     * @param cls the name of the class
     * @param method the name of the method
     * @param in_buf where to find input
     * @param in_len length of in_buf in bytes
     * @param out_buf where to put librados-allocated output buffer
     * @param out_len length of out_buf in bytes
     * @param prval where to store the return value from the method
     */
    pub fn rados_read_op_exec(
        read_op: rados_read_op_t,
        cls: *const libc::c_char,
        method: *const libc::c_char,
        in_buf: *const libc::c_char,
        in_len: size_t,
        out_buf: *mut *mut libc::c_char,
        out_len: *mut size_t,
        prval: *mut libc::c_int,
    ) -> ();

    /**
     * Execute an OSD class method on an object
     * See rados_exec() for general description.
     *
     * If the output buffer is too small, prval will
     * be set to -ERANGE and used_len will be 0.
     *
     * @param read_op operation to add this action to
     * @param cls the name of the class
     * @param method the name of the method
     * @param in_buf where to find input
     * @param in_len length of in_buf in bytes
     * @param out_buf user-provided buffer to read into
     * @param out_len length of out_buf in bytes
     * @param used_len where to store the number of bytes read into out_buf
     * @param prval where to store the return value from the method
     */
    pub fn rados_read_op_exec_user_buf(
        read_op: rados_read_op_t,
        cls: *const libc::c_char,
        method: *const libc::c_char,
        in_buf: *const libc::c_char,
        in_len: size_t,
        out_buf: *mut libc::c_char,
        out_len: size_t,
        used_len: *mut size_t,
        prval: *mut libc::c_int,
    ) -> ();

    /**
     * Start iterating over key/value pairs on an object.
     *
     * They will be returned sorted by key.
     *
     * @param read_op operation to add this action to
     * @param start_after list keys starting after start_after
     * @param filter_prefix list only keys beginning with filter_prefix
     * @param max_return list no more than max_return key/value pairs
     * @param iter where to store the iterator
     * @param pmore flag indicating whether there are more keys to fetch
     * @param prval where to store the return value from this action
     */
    pub fn rados_read_op_omap_get_vals2(
        read_op: rados_read_op_t,
        start_after: *const libc::c_char,
        filter_prefix: *const libc::c_char,
        max_return: u64,
        iter: *mut rados_omap_iter_t,
        pmore: *mut libc::c_uchar,
        prval: *mut libc::c_int,
    ) -> ();

    /**
     * Start iterating over keys on an object.
     *
     * They will be returned sorted by key, and the iterator
     * will fill in NULL for all values if specified.
     *
     * @param read_op operation to add this action to
     * @param start_after list keys starting after start_after
     * @param max_return list no more than max_return keys
     * @param iter where to store the iterator
     * @param pmore flag indicating whether there are more keys to fetch
     * @param prval where to store the return value from this action
     */
    pub fn rados_read_op_omap_get_keys2(
        read_op: rados_read_op_t,
        start_after: *const libc::c_char,
        max_return: u64,
        iter: *mut rados_omap_iter_t,
        pmore: *mut libc::c_uchar,
        prval: *mut libc::c_int,
    ) -> ();

    /**
     * Start iterating over specific key/value pairs
     *
     * They will be returned sorted by key.
     *
     * @param read_op operation to add this action to
     * @param keys array of pointers to null-terminated keys to get
     * @param keys_len the number of strings in keys
     * @param iter where to store the iterator
     * @param prval where to store the return value from this action
     */
    pub fn rados_read_op_omap_get_vals_by_keys(
        read_op: rados_read_op_t,
        keys: *const *const libc::c_char,
        keys_len: size_t,
        iter: *mut rados_omap_iter_t,
        prval: *mut libc::c_int,
    ) -> ();

    /**
     * Perform a read operation synchronously
     * @param read_op operation to perform
     * @param io the ioctx that the object is in
     * @param oid the object id
     * @param flags flags to apply to the entire operation (LIBRADOS_OPERATION_*)
     */
    pub fn rados_read_op_operate(
        read_op: rados_read_op_t,
        io: rados_ioctx_t,
        oid: *const libc::c_char,
        flags: libc::c_int,
    ) -> libc::c_int;

    /**
     * Perform a read operation asynchronously
     * @param read_op operation to perform
     * @param io the ioctx that the object is in
     * @param completion what to do when operation has been attempted
     * @param oid the object id
     * @param flags flags to apply to the entire operation (LIBRADOS_OPERATION_*)
     */
    pub fn rados_aio_read_op_operate(
        read_op: rados_read_op_t,
        io: rados_ioctx_t,
        completion: rados_completion_t,
        oid: *const libc::c_char,
        flags: libc::c_int,
    ) -> libc::c_int;

    /**
     * Take an exclusive lock on an object.
     *
     * @param io the context to operate in
     * @param oid the name of the object
     * @param name the name of the lock
     * @param cookie user-defined identifier for this instance of the lock
     * @param desc user-defined lock description
     * @param duration the duration of the lock. Set to NULL for infinite duration.
     * @param flags lock flags
     * @returns 0 on success, negative error code on failure
     * @returns -EBUSY if the lock is already held by another (client, cookie) pair
     * @returns -EEXIST if the lock is already held by the same (client, cookie) pair
     */
    pub fn rados_lock_exclusive(
        io: rados_ioctx_t,
        o: *const libc::c_char,
        name: *const libc::c_char,
        cookie: *const libc::c_char,
        desc: *const libc::c_char,
        duration: *mut timeval,
        flags: u8,
    ) -> libc::c_int;

    /**
     * Take a shared lock on an object.
     *
     * @param io the context to operate in
     * @param o the name of the object
     * @param name the name of the lock
     * @param cookie user-defined identifier for this instance of the lock
     * @param tag The tag of the lock
     * @param desc user-defined lock description
     * @param duration the duration of the lock. Set to NULL for infinite duration.
     * @param flags lock flags
     * @returns 0 on success, negative error code on failure
     * @returns -EBUSY if the lock is already held by another (client, cookie) pair
     * @returns -EEXIST if the lock is already held by the same (client, cookie) pair
     */
    pub fn rados_lock_shared(
        io: rados_ioctx_t,
        o: *const libc::c_char,
        name: *const libc::c_char,
        cookie: *const libc::c_char,
        tag: *const libc::c_char,
        desc: *const libc::c_char,
        duration: *mut timeval,
        flags: u8,
    ) -> libc::c_int;

    /**
     * Release a shared or exclusive lock on an object.
     *
     * @param io the context to operate in
     * @param o the name of the object
     * @param name the name of the lock
     * @param cookie user-defined identifier for the instance of the lock
     * @returns 0 on success, negative error code on failure
     * @returns -ENOENT if the lock is not held by the specified (client, cookie) pair
     */
    pub fn rados_unlock(
        io: rados_ioctx_t,
        o: *const libc::c_char,
        name: *const libc::c_char,
        cookie: *const libc::c_char,
    ) -> libc::c_int;

    /**
     * List clients that have locked the named object lock and information about
     * the lock.
     *
     * The number of bytes required in each buffer is put in the
     * corresponding size out parameter. If any of the provided buffers
     * are too short, -ERANGE is returned after these sizes are filled in.
     *
     * @param io the context to operate in
     * @param o the name of the object
     * @param name the name of the lock
     * @param exclusive where to store whether the lock is exclusive (1) or shared (0)
     * @param tag where to store the tag associated with the object lock
     * @param tag_len number of bytes in tag buffer
     * @param clients buffer in which locker clients are stored, separated by '\0'
     * @param clients_len number of bytes in the clients buffer
     * @param cookies buffer in which locker cookies are stored, separated by '\0'
     * @param cookies_len number of bytes in the cookies buffer
     * @param addrs buffer in which locker addresses are stored, separated by '\0'
     * @param addrs_len number of bytes in the clients buffer
     * @returns number of lockers on success, negative error code on failure
     * @returns -ERANGE if any of the buffers are too short
     */
    pub fn rados_list_lockers(
        io: rados_ioctx_t,
        o: *const libc::c_char,
        name: *const libc::c_char,
        exclusive: *mut libc::c_int,
        tag: *mut libc::c_char,
        tag_len: *mut size_t,
        clients: *mut libc::c_char,
        clients_len: *mut size_t,
        cookies: *mut libc::c_char,
        cookies_len: *mut size_t,
        addrs: *mut libc::c_char,
        addrs_len: *mut size_t,
    ) -> ssize_t;

    /**
     * Releases a shared or exclusive lock on an object, which was taken by the
     * specified client.
     *
     * @param io the context to operate in
     * @param o the name of the object
     * @param name the name of the lock
     * @param client the client currently holding the lock
     * @param cookie user-defined identifier for the instance of the lock
     * @returns 0 on success, negative error code on failure
     * @returns -ENOENT if the lock is not held by the specified (client, cookie) pair
     * @returns -EINVAL if the client cannot be parsed
     */
    pub fn rados_break_lock(
        io: rados_ioctx_t,
        o: *const libc::c_char,
        name: *const libc::c_char,
        client: *const libc::c_char,
        cookie: *const libc::c_char,
    ) -> libc::c_int;

    /**
     * Blocklists the specified client from the OSDs
     *
     * @param cluster cluster handle
     * @param client_address client address
     * @param expire_seconds number of seconds to blocklist (0 for default)
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_blocklist_add(
        cluster: rados_t,
        client_address: *mut libc::c_char,
        expire_seconds: u32,
    ) -> libc::c_int;

    /**
     * Send monitor command.
     *
     * @note Takes command string in carefully-formatted JSON; must match
     * defined commands, types, etc.
     *
     * The result buffers are allocated on the heap; the caller is
     * expected to release that memory with rados_buffer_free().  The
     * buffer and length pointers can all be NULL, in which case they are
     * not filled in.
     *
     * @param cluster cluster handle
     * @param cmd an array of char *'s representing the command
     * @param cmdlen count of valid entries in cmd
     * @param inbuf any bulk input data (crush map, etc.)
     * @param inbuflen input buffer length
     * @param outbuf double pointer to output buffer
     * @param outbuflen pointer to output buffer length
     * @param outs double pointer to status string
     * @param outslen pointer to status string length
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_mon_command(
        cluster: rados_t,
        cmd: *mut *const libc::c_char,
        cmdlen: size_t,
        inbuf: *const libc::c_char,
        inbuflen: size_t,
        outbuf: *mut *mut libc::c_char,
        outbuflen: *mut size_t,
        outs: *mut *mut libc::c_char,
        outslen: *mut size_t,
    ) -> libc::c_int;

    /**
     * Send monitor command to a specific monitor.
     *
     * @note Takes command string in carefully-formatted JSON; must match
     * defined commands, types, etc.
     *
     * The result buffers are allocated on the heap; the caller is
     * expected to release that memory with rados_buffer_free().  The
     * buffer and length pointers can all be NULL, in which case they are
     * not filled in.
     *
     * @param cluster cluster handle
     * @param name target monitor's name
     * @param cmd an array of char *'s representing the command
     * @param cmdlen count of valid entries in cmd
     * @param inbuf any bulk input data (crush map, etc.)
     * @param inbuflen input buffer length
     * @param outbuf double pointer to output buffer
     * @param outbuflen pointer to output buffer length
     * @param outs double pointer to status string
     * @param outslen pointer to status string length
     * @returns 0 on success, negative error code on failure
     */
    pub fn rados_mon_command_target(
        cluster: rados_t,
        name: *const libc::c_char,
        cmd: *mut *const libc::c_char,
        cmdlen: size_t,
        inbuf: *const libc::c_char,
        inbuflen: size_t,
        outbuf: *mut *mut libc::c_char,
        outbuflen: *mut size_t,
        outs: *mut *mut libc::c_char,
        outslen: *mut size_t,
    ) -> libc::c_int;

    /**
     * free a rados-allocated buffer
     *
     * Release memory allocated by librados calls like rados_mon_command().
     *
     * @param buf buffer pointer
     */
    pub fn rados_buffer_free(buf: *mut libc::c_char) -> ();

    pub fn rados_osd_command(
        cluster: rados_t,
        osdid: libc::c_int,
        cmd: *mut *const libc::c_char,
        cmdlen: size_t,
        inbuf: *const libc::c_char,
        inbuflen: size_t,
        outbuf: *mut *mut libc::c_char,
        outbuflen: *mut size_t,
        outs: *mut *mut libc::c_char,
        outslen: *mut size_t,
    ) -> libc::c_int;
    pub fn rados_pg_command(
        cluster: rados_t,
        pgstr: *const libc::c_char,
        cmd: *mut *const libc::c_char,
        cmdlen: size_t,
        inbuf: *const libc::c_char,
        inbuflen: size_t,
        outbuf: *mut *mut libc::c_char,
        outbuflen: *mut size_t,
        outs: *mut *mut libc::c_char,
        outslen: *mut size_t,
    ) -> libc::c_int;
    pub fn rados_monitor_log(
        cluster: rados_t,
        level: *const libc::c_char,
        cb: rados_log_callback_t,
        arg: *mut std::os::raw::c_void,
    ) -> libc::c_int;
}
