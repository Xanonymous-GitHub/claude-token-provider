#!/usr/bin/env bash

set -euo pipefail

### ================== CONFIG (edit or pass as env) ==================
# Rust target binary name (crate name == binary name by default)
: "${PRODUCT_NAME:=mytool}"

# Reverse-DNS identifier (used by the pkg)
: "${BUNDLE_ID:=com.example.${PRODUCT_NAME}}"

# Version to stamp on the pkg (default: parse Cargo.toml, fallback 1.0.0)
: "${VERSION:=}"
if [[ -z "${VERSION}" ]]; then
  if [[ -f Cargo.toml ]]; then
    VERSION="$(sed -n 's/^version\s*=\s*"\(.*\)"/\1/p' Cargo.toml | head -n1 || true)"
  fi
  : "${VERSION:=1.0.0}"
fi

# Where the CLI lands on the target system (recommended for CLIs)
: "${INSTALL_PATH:=/usr/local/bin}"

# Your 10-char Apple Team ID and certificate Common Names
: "${TEAM_ID:=ABCDE12345}"
: "${APP_CERT_CN:=Developer ID Application: Your Name (${TEAM_ID})}"
: "${INSTALLER_CERT_CN:=Developer ID Installer: Your Name (${TEAM_ID})}"

# notarytool keychain profile name (created via: xcrun notarytool store-credentials ...)
: "${AC_PROFILE:=AC_PROFILE}"

# Toggle to drop a copy or symlink on the console user's Desktop (NOT recommended for production)
# 0 = off (default), 1 = copy, 2 = symlink
: "${INSTALL_TO_DESKTOP:=0}"

# If set to 1, skip notarization (e.g., local dev smoke)
: "${SKIP_NOTARIZE:=0}"

# Path to the prebuilt release binary (the script can build if missing)
: "${BIN:=target/release/${PRODUCT_NAME}}"

# Optional entitlements file for CLI (rarely needed). Leave empty for none.
: "${ENTITLEMENTS:=}"

### ================== SCRIPT START ==================
log() { printf "\033[1;34m[INFO]\033[0m %s\n" "$*"; }
warn(){ printf "\033[1;33m[WARN]\033[0m %s\n" "$*"; }
err() { printf "\033[1;31m[ERR ]\033[0m %s\n" "$*" >&2; }
die() { err "$*"; exit 1; }

need() { command -v "$1" >/dev/null 2>&1 || die "Missing tool: $1"; }

need xcrun; need codesign; need pkgbuild
xcrun --find notarytool >/dev/null
xcrun --find stapler >/dev/null

# Build if the Rust binary is missing
if [[ ! -f "$BIN" ]]; then
  log "Rust release binary not found at $BIN; building with cargo…"
  need cargo
  cargo build --release
  [[ -f "$BIN" ]] || die "Build did not produce $BIN"
fi

# Work dir
ROOT="$(pwd)"
OUTDIR="${ROOT}/dist"
PKGROOT="${OUTDIR}/pkgroot"
SCRIPTSDIR="${OUTDIR}/scripts"
BIN_NAME="$(basename "$PRODUCT_NAME")"
INSTALL_DEST="${PKGROOT}${INSTALL_PATH}"
PKG="${OUTDIR}/${PRODUCT_NAME}-${VERSION}.pkg"

rm -rf "$OUTDIR"
mkdir -p "$INSTALL_DEST" "$SCRIPTSDIR"

# Stage payload
log "Staging payload under ${INSTALL_PATH}…"
cp -f "$BIN" "${INSTALL_DEST}/${BIN_NAME}"
chmod 755 "${INSTALL_DEST}/${BIN_NAME}"
# Clear extended attrs on staged file (sometimes prevents oddities)
xattr -cr "${INSTALL_DEST}/${BIN_NAME}" || true

# Codesign the Mach-O (Hardened Runtime + timestamp; add entitlements if provided)
log "Code-signing Mach-O with Developer ID Application (${APP_CERT_CN})…"
SIGN_ARGS=(--force --timestamp --options runtime -s "$APP_CERT_CN")
[[ -n "${ENTITLEMENTS}" && -f "${ENTITLEMENTS}" ]] && SIGN_ARGS+=(--entitlements "$ENTITLEMENTS")
codesign "${SIGN_ARGS[@]}" "${INSTALL_DEST}/${BIN_NAME}"

# Verify the signature
log "Verifying code signature…"
codesign --verify --strict --verbose=2 "${INSTALL_DEST}/${BIN_NAME}"

# Optional Desktop drop via postinstall (NOT best practice; for special cases only)
POSTINSTALL_USED=0
if [[ "${INSTALL_TO_DESKTOP}" != "0" ]]; then
  POSTINSTALL_USED=1
  log "Generating postinstall script to place the tool on the console user's Desktop (mode=${INSTALL_TO_DESKTOP})…"
  cat > "${SCRIPTSDIR}/postinstall" <<'SH'
#!/bin/bash
set -euo pipefail

console_user="$(/usr/bin/stat -f%Su /dev/console 2>/dev/null || echo "")"
[[ -n "$console_user" && "$console_user" != "root" ]] || exit 0

user_home="$(/usr/bin/dscl . -read "/Users/${console_user}" NFSHomeDirectory 2>/dev/null | awk '{print $2}')"
[[ -d "$user_home" ]] || exit 0

BIN_ON_DISK="__BIN_ON_DISK__"
PRODUCT_NAME="__PRODUCT_NAME__"
mode="__MODE__"   # 1=copy, 2=symlink

desktop="${user_home}/Desktop"
mkdir -p "$desktop"
dest="${desktop}/${PRODUCT_NAME}"

if [[ "$mode" == "1" ]]; then
  /bin/cp -f "$BIN_ON_DISK" "$dest"
  /usr/sbin/chown "${console_user}":staff "$dest"
  /bin/chmod 755 "$dest"
elif [[ "$mode" == "2" ]]; then
  /bin/ln -sf "$BIN_ON_DISK" "$dest"
  /usr/sbin/chown -h "${console_user}":staff "$dest" || true
fi
exit 0
SH
  sed -i '' \
    -e "s|__BIN_ON_DISK__|${INSTALL_PATH}/${BIN_NAME}|g" \
    -e "s|__PRODUCT_NAME__|${BIN_NAME}|g" \
    -e "s|__MODE__|${INSTALL_TO_DESKTOP}|g" \
    "${SCRIPTSDIR}/postinstall"
  chmod 755 "${SCRIPTSDIR}/postinstall"
fi

# Build a signed installer package (.pkg)
log "Building signed installer package…"
PKGBUILD_ARGS=(
  --root "$PKGROOT"
  --identifier "$BUNDLE_ID"
  --version "$VERSION"
  --install-location "/"
  --sign "$INSTALLER_CERT_CN"
)
[[ $POSTINSTALL_USED -eq 1 ]] && PKGBUILD_ARGS+=(--scripts "$SCRIPTSDIR")
pkgbuild "${PKGBUILD_ARGS[@]}" "$PKG"

# Quick checks: pkg signature and app signature
log "pkgutil --check-signature…"
pkgutil --check-signature "$PKG" || true
log "spctl --assess (install)…"
spctl --assess --type install -vv "$PKG" || true

# Notarize & staple (skip if requested)
if [[ "${SKIP_NOTARIZE}" == "1" ]]; then
  warn "SKIP_NOTARIZE=1 → skipping notarization & stapling."
else
  log "Submitting pkg to Apple Notary service (this can take a bit)…"
  xcrun notarytool submit "$PKG" --keychain-profile "$AC_PROFILE" --wait

  log "Stapling notarization ticket to pkg…"
  xcrun stapler staple "$PKG"
  xcrun stapler validate "$PKG"
fi

log "Done."
echo "Artifact: $PKG"
echo "Installed path inside pkg payload: ${INSTALL_PATH}/${BIN_NAME}"
