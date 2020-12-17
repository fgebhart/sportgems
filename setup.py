import sys

from setuptools import setup
from sportgems import __version__

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
    author="Fabian Gebhart",
    version=__version__,
    classifiers=[
        'License :: OSI Approved :: MIT License',
        'Intended Audience :: Developers',
        'Programming Language :: Python',
        'Programming Language :: Rust',
        'Operating System :: POSIX',
        'Operating System :: MacOS :: MacOS X',
    ],
    packages=['sportgems'],
    url="https://github.com/fgebhart/sportgems",
    rust_extensions=[RustExtension('sportgems._sportgems', 'Cargo.toml')],
    install_requires=install_requires,
    python_requires=">=3.8",
    tests_require=tests_require,
    setup_requires=setup_requires,
    include_package_data=True,
    zip_safe=False,
    cmdclass=dict()
)
