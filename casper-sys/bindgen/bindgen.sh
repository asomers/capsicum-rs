#! /bin/sh

# Must be run on FreeBSD 13 or lower.  FreeBSD 14's libnv has a higher .so
# version, and uses different symbol names.  For backwards compatibility, we use
# libnv.so.0.  See also build.rs.

CRATEDIR=`dirname $0`/..

bindgen --generate functions,types \
	--allowlist-function 'cap_.*' \
	--allowlist-function 'service_register' \
	${CRATEDIR}/bindgen/wrapper.h > ${CRATEDIR}/src/ffi.rs
