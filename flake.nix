{
  description = "huggingface tokenizers";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, utils, naersk, ... }: {

      overlay = final: prev: rec {
        tokenizersPackages = prev.callPackage ./. { naersk = naersk.lib."${final.system}"; };
        tokenizers-haskell = tokenizersPackages.tokenizers-haskell;
        tokenizers_haskell = tokenizersPackages.tokenizers-haskell; # for backwards-compatibility
      };

    } // (utils.lib.eachSystem ["x86_64-darwin" "x86_64-linux"] (system:
      let
        pkgs = import nixpkgs { inherit system; overlays = [ naersk.overlay ]; };
      in {
        packages = pkgs.callPackage ./. {};
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ cargo rustc rls libiconv pkgconfig ];
        };
      }
    ));
}
