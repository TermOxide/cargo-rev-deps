{
  lib,
  rustPlatform,
  makeBinaryWrapper,
  cargo-workspaces,
  gitMinimal,
}:
let
  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./Cargo.toml
      ./Cargo.lock
      ./src
    ];
  };

  cargoTOML = builtins.fromTOML (builtins.readFile ./Cargo.toml);
in
rustPlatform.buildRustPackage {
  pname = cargoTOML.package.name;
  inherit (cargoTOML.package) version;
  inherit src;

  __structuredAttrs = true;

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [ makeBinaryWrapper ];

  # cargo itself comes from `$CARGO`, set by the cargo that runs the subcommand
  postInstall = ''
    wrapProgram $out/bin/cargo-rev-deps \
      --suffix PATH : ${
        lib.makeBinPath [
          cargo-workspaces
          gitMinimal
        ]
      }
  '';

  meta = {
    inherit (cargoTOML.package) description;
    homepage = cargoTOML.package.repository;
    license = lib.licenses.bsd2;
    maintainers = [ lib.maintainers.cizniarova ];
    mainProgram = "cargo-rev-deps";
  };
}
