export LEAN_LIB_DIR=$(lean --print-libdir)
export LD_LIBRARY_PATH=${LD_LIBRARY_PATH}:$(lean --print-libdir)
export DYLD_LIBRARY_PATH=${DYLD_LIBRARY_PATH}:$(lean --print-libdir)