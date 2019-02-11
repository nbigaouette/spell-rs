from setuptools import setup, find_packages
import os
import re

RUST_BUILD = "debug"
# RUST_BUILD = "release"

def version():
    with open('pyspellrs/Cargo.toml', encoding='utf-8') as f:
        content = f.read()
    m = re.search('version = "([0-9\.]*)".*', content)
    return m.group(1)

def readme():
    """print long description"""
    with open('../README.md', encoding='utf-8') as f:
        return f.read()


def build_native(spec):
    # Step 1: build the rust library
    cmd = ['cargo', 'build']
    if RUST_BUILD == "release":
        cmd.append('--release')
    cmd.append('--package')
    cmd.append('pyspellrs')
    build = spec.add_external_build(
        cmd=cmd,
        path='..'
    )

    # Step 2: add a cffi module based on the dylib we built
    #
    # We use lambdas here for dylib and header_filename so that those are
    # only called after the external build finished.
    cargo_target_dir = os.getenv('CARGO_TARGET_DIR', 'target')
    in_path_dylib = os.path.normpath(os.path.join(cargo_target_dir, RUST_BUILD))
    in_path_header = os.path.normpath("target")
    spec.add_cffi_module(
        module_path='spell._native',
        dylib=lambda: build.find_dylib(
            'pyspellrs', in_path=in_path_dylib),
        header_filename=lambda: build.find_header(
            'spell.h', in_path=in_path_header),
        rtld_flags=['NOW', 'NODELETE']
    )


setup(
    name='spellrs',
    version=version(),
    url="https://travis-ci.org/nbigaouette/spell-rs",
    author="Nicolas Bigaouette",
    author_email="nbigaouette@gmail.com",
    long_description=readme(),
    python_requires='>=3',
    packages=find_packages(),
    include_package_data=True,
    zip_safe=False,
    platforms='any',
    install_requires=[
        'milksnake',
        'wheel',
    ],
    milksnake_tasks=[
        build_native,
    ]
)
