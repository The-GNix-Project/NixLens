{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  shellHook = ''
    if [ ! -d "venv" ]; then
      python -m venv venv
      source venv/bin/activate
    else
      source venv/bin/activate
    fi
    pip install -r requirements.txt
  '';
}