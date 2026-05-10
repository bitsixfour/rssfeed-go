{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = [
    pkgs.rustc
    pkgs.cargo
    pkgs.pkg-config
    pkgs.openssl

    # Dioxus CLI
    pkgs.dioxus-cli

    # GTK + WebKitGTK (the actual WebView backend on Linux)
    pkgs.gtk3
    pkgs.rustlings
    pkgs.webkitgtk_4_1
    pkgs.glib
    pkgs.pango
    pkgs.atk
    pkgs.cairo
    pkgs.gdk-pixbuf
    pkgs.xdotool
    # IPC / system
    pkgs.dbus
    pkgs.librsvg
  ];

  # pkg-config needs to find all the above
  PKG_CONFIG_PATH = pkgs.lib.makeSearchPathOutput "dev" "lib/pkgconfig" [
    pkgs.openssl
    pkgs.gtk3
    pkgs.webkitgtk_4_1
    pkgs.glib
    pkgs.dbus
  ];
}
