{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  # Explicitly specify LLVM/clang packages
  packages = with pkgs; [
    llvm
    clang
    llvmPackages.libclang
    stdenv.cc.cc.lib
    pkg-config
    python3
  ];
  # Set environment variables needed by clang-sys
  shellHook = ''
    export LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib"
    export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
      pkgs.llvm
      pkgs.clang
      pkgs.llvmPackages.libclang
    ]}:$LD_LIBRARY_PATH"

    if [ ! -d "venv" ]; then
      python -m venv venv
      source venv/bin/activate
      pip install -r requirements.txt
    else
      source venv/bin/activate
    fi
  '';
}