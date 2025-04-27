{
  rustPlatform,
  gtk4,
  libadwaita,
  pkg-config,
  fetchFromGitHub,
  hicolor-icon-theme,
  wrapGAppsHook4,
  copyDesktopItems,
  makeDesktopItem
}:
rustPlatform.buildRustPackage rec {
  pname = "nasin";
  version = "1.0.2";

  src = fetchFromGitHub {
    owner = "jonot-cyber";
    repo = pname;
    rev = "master";
    hash = "sha256-CPSajEFKJIvJe1G2u5oev7SCepZJ+1JlxDZkNhHlPlY=";
  };

  buildInputs = [ gtk4 libadwaita ];

  nativeBuildInputs = [ pkg-config hicolor-icon-theme wrapGAppsHook4 copyDesktopItems ];
  cargoHash = "sha256-6sZXx7nPlTqYjCxx4IJlbn5L17s2DhuDOnUJxHTAPtw=";

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
