import os
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


def requirements_from_txt(path_to_txt):
    with open(path_to_txt, "r") as f:
        reqs = f.readlines()
    return [req for req in reqs if not req.startswith("#")]


with open("Readme.md", "r") as fh:
    long_description = fh.read()

setup_requires = ['setuptools-rust>=0.6.0']
install_requires = []
tests_require = install_requires + requirements_from_txt("requirements.txt")

setup(
    name='sportgems',
    author="Fabian Gebhart",
    version=__version__,
    description="Find valuable gems 💎 in your tracked sport 🚴 activity!",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/fgebhart/sportgems",
    classifiers=[
        'License :: OSI Approved :: MIT License',
        'Intended Audience :: Developers',
        'Programming Language :: Python',
        'Programming Language :: Rust',
        'Operating System :: POSIX',
        'Operating System :: MacOS :: MacOS X',
    ],
    python_requires=">=3.8",
    packages=['sportgems'],
    rust_extensions=[RustExtension('sportgems._sportgems', 'Cargo.toml')],
    install_requires=install_requires,
    tests_require=tests_require,
    setup_requires=setup_requires,
    include_package_data=True,
    zip_safe=False,
    cmdclass=dict(),
)
