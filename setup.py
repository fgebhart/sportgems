import sys

from setuptools import setup

try:
    from setuptools_rust import RustExtension
except ImportError:
    import subprocess
    errno = subprocess.call([sys.executable, '-m', 'pip', 'install', 'setuptools-rust'])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension

setup_requires = ['setuptools-rust>=0.6.0']
install_requires = []
tests_require = install_requires + ['pytest', 'pytest-benchmark']

setup(
    name='sportgems',
    version='0.1.0',
    classifiers=[
        'License :: OSI Approved :: MIT License',
        'Development Status :: 3 - Alpha',
        'Intended Audience :: Developers',
        'Programming Language :: Python',
        'Programming Language :: Rust',
        'Operating System :: POSIX',
        'Operating System :: MacOS :: MacOS X',
    ],
    packages=['sportgems'],
    rust_extensions=[RustExtension('sportgems._sportgems', 'Cargo.toml')],
    install_requires=install_requires,
    tests_require=tests_require,
    setup_requires=setup_requires,
    include_package_data=True,
    zip_safe=False,
    cmdclass=dict()
)
