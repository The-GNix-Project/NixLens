{ pkgs ? import <nixpkgs> {} }:
let packages = with pkgs; [
    llvm
    clang
    llvmPackages.libclang
    stdenv.cc.cc.lib
    pkg-config
    python3
  ];
in
pkgs.mkShell {
  packages = packages;
  
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