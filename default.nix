{
  rustPlatform,
  gtk4,
  libadwaita,
  pkg-config,
  hicolor-icon-theme,
  wrapGAppsHook4,
  copyDesktopItems,
  makeDesktopItem
}:

rustPlatform.buildRustPackage {
  pname = "nasin";
  version = "1.0.2";

  src = ./.;

  buildInputs = [ gtk4 libadwaita ];

  nativeBuildInputs = [ pkg-config hicolor-icon-theme wrapGAppsHook4 copyDesktopItems ];
  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  desktopItems = [(makeDesktopItem {
    name = "nasin";
    desktopName = "Nasin";
    terminal = false;
    categories = [ "Office" ];
    icon = "me.jonot.Nasin";
    exec = "nasin";
  })];
  
  postInstall = ''
    mkdir -p $out/share/icons
    cp $src/data/me.jonot.Nasin.desktop $out/share/me.jonot.Nasin.desktop
    cp $src/data/icons/me.jonot.Nasin.svg $out/share/icons/me.jonot.Nasin.svg
  '';
}
