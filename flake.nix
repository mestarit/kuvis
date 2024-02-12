{
  description = "A basic flake with a shell";
  #inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        inherit (nixpkgs) lib;
        pkgs = nixpkgs.legacyPackages.${system};
        runtimeDeps = with pkgs; [
          libxkbcommon wayland # for wayland
          udev alsa-lib vulkan-loader
        ] ++ (with xorg; [ # for x11
          libXcursor
          libXrandr
          libXi
          libX11
        ]);
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            mold-wrapped # a VERY fast linker
            # llvmPackages.bintools # for lld, a fast linker
          ];
          buildInputs = runtimeDeps;
          LD_LIBRARY_PATH = lib.makeLibraryPath runtimeDeps;
        };
      });
}
