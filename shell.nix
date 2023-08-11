let
  pkgs = import <nixpkgs> {};
  rust-toolchain = pkgs.symlinkJoin {
    name = "rust-toolchain";
    paths = [pkgs.rustfmt pkgs.rustc pkgs.cargo pkgs.rustPlatform.rustcSrc pkgs.clippy];
  };
in with pkgs;

mkShell {
  name = "flaggy";
  buildInputs = [
    libiconv
    openssl
    pkgconfig
    rust-analyzer
    rust-toolchain
  ] ++ 
  lib.optionals (!stdenv.isDarwin) [
    procps
  ] ++
  lib.optionals (stdenv.isDarwin) [
    darwin.apple_sdk.frameworks.Security
    darwin.libobjc
  ]
  ;

  NIX_ENFORCE_PURITY = 0;
  NIX_SHELL_PRESERVE_PROMPT = 1;
  RUST_BACKTRACE = "full";
  RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
  
  shellHook = ''
    export RUST_TOOLCHAIN=''$(dirname ''$(whereis -b rustc | cut -d ' ' -f 2))
    echo RUST_TOOLCHAIN: ''$RUST_TOOLCHAIN
    echo RUST_SRC_PATH: ''$RUST_SRC_PATH
  '';

}

