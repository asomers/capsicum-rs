#! /bin/sh

# Must be run on FreeBSD 13 or lower.  FreeBSD 14's libnv has a higher .so
# version, and uses different symbol names.  For backwards compatibility, we use
# libnv.so.0.  See also build.rs.

CRATEDIR=`dirname $0`/..

bindgen --generate functions,types \
	--allowlist-function 'cap_.*' \
	--blocklist-type cap_channel \
	--blocklist-type cap_channel_t \
	--blocklist-type __gid_t \
	--blocklist-type __uint32_t \
	--blocklist-type gid_t \
	--blocklist-type group \
	${CRATEDIR}/bindgen/wrapper.h > ${CRATEDIR}/src/ffi.rs
