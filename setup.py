from setuptools import setup, find_packages
from setuptools_rust import RustExtension, Binding

setup(
    name="nix_analyzer",
    version="0.1.0",
    description="A Nix analyzer library with a Rust-powered Nix parser",
    author="Your Name",
    author_email="you@example.com",
    url="https://github.com/yourusername/nix_analyzer",
    packages=find_packages(),  # Automatically finds the `nix_analyzer` package directory.
    rust_extensions=[
        RustExtension(
            "nix_analyzer.nix_parser",  # This is the fully-qualified module name.
            path="Cargo.toml",
            binding=Binding.PyO3,
            debug=False  # Set to True for development builds.
        )
    ],
    # If using setuptools-rust, it's recommended to disable zip_safe.
    include_package_data=True,
    zip_safe=False,
    classifiers=[
        "Programming Language :: Python :: 3",
        "Programming Language :: Rust",
        "Operating System :: OS Independent",
    ],
)
