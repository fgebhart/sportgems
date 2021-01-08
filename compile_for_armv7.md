
# compiling sportgems for armv7-unknown-linux-gnueabihf

## download Python
wget https://www.python.org/ftp/python/3.8.6/Python-3.8.6.tgz

## extract it
tar -xvzf Python-3.8.6.tgz

## set paths
export HOSTPYTHON=/tmp/depsBuild/pythonhost/python
export CONFIGURE_PREFIX=/tmp/python
export BUILT_ARM_PYTHON=/tmp/Python-3.8.6

## compile it
CC=arm-linux-gnueabihf-gcc CXX=arm-linux-gnueabihf-g++ AR=arm-linux-gnueabihf-ar \
    RANLIB=arm-linux-gnueabihf-ranlib \
    ./configure --host=arm-linux-gnueabihf --target=arm-linux-gnueabihf \
    --build=x86_64-linux-gnu --prefix=$CONFIGURE_PREFIX \
    --disable-ipv6 ac_cv_file__dev_ptmx=no ac_cv_file__dev_ptc=no \
    ac_cv_have_long_long_format=yes --enable-shared

## make
make HOSTPYTHON=$HOSTPYTHON \
    BLDSHARED="arm-linux-gnueabihf-gcc -shared" CROSS-COMPILE=arm-linux-gnueabihf- \
    CROSS_COMPILE_TARGET=yes HOSTARCH=arm-linux BUILDARCH=arm-linux-gnueabihf

## altinstall
make altinstall HOSTPYTHON=$HOSTPYTHON \
    BLDSHARED="arm-linux-gnueabihf-gcc -shared" CROSS-COMPILE=arm-linux-gnueabihf- \
    CROSS_COMPILE_TARGET=yes HOSTARCH=arm-linux BUILDARCH=arm-linux-gnueabihf \
    prefix=$BUILT_ARM_PYTHON

## build Python wheel
PYO3_CROSS_INCLUDE_DIR="$BUILT_ARM_PYTHON/include" PYO3_CROSS_LIB_DIR="$BUILT_ARM_PYTHON/lib" maturin build --target=armv7-unknown-linux-gnueabihf --release --manylinux=off