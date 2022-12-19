#! /bin/sh

# Must be run on FreeBSD 13 or lower.  FreeBSD 14's libnv has a higher .so
# version, and uses different symbol names.  For backwards compatibility, we use
# libnv.so.0.  See also build.rs.

CRATEDIR=`dirname $0`/..
cat > ${CRATEDIR}/src/lib.rs << HERE
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
HERE

bindgen --generate functions,types \
	--allowlist-function 'cap_.*' \
	${CRATEDIR}/bindgen/wrapper.h >> ${CRATEDIR}/src/lib.rs
