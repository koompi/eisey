# Maintainer <Brilliant PHAL, phalbrilliant@gmail.com>
pkgname=sel
pkgver=0.1.0
pkgrel=1
pkgdesc="Simplify package management and maintenance in KOOMPI OS"
arch=('x86_64')
license=('GPL3')
source=('git+https://github.com/koompi/sel.git')
sha256sums=('SKIP')
depends=('glibc' 'rustup' 'gnupg' 'xdg-utils')
makedepends=('nodejs' 'npm')
install=$pkgname.install

package() {
    sed -i 's/ maintenance/ \/usr\/share\/org.koompi.sel\/maintenance/g' $srcdir/$pkgname/operation.yml
    cd $srcdir/sel
    rustup default stable
    cargo build --release --bin user-cli
    cargo build --release --bin issuer-cli
    cd $srcdir/sel/gui-client
    npm install
    npm run build
    cd $srcdir/sel
    cargo build --release --bin user-gui
    
    mkdir -p $pkgdir/usr/bin
    mkdir -p $pkgdir/usr/share/applications
    mkdir -p $pkgdir/usr/share/org.koompi.sel/maintenance

    install -Dm755 $srcdir/sel/target/release/user-cli $pkgdir/usr/bin/sel-protocol
    install -Dm755 $srcdir/sel/target/release/user-gui $pkgdir/usr/bin/sel-wallet
    install -Dm755 $srcdir/sel/target/release/issuer-cli $pkgdir/usr/bin/sel-issuer
    install -Dm644 $srcdir/sel/sel.desktop $pkgdir/usr/share/applications/sel.desktop
    install -Dm644 $srcdir/sel/sel-protocol.desktop $pkgdir/usr/share/applications/sel-protocol.desktop
    install -Dm755 $srcdir/sel/maintenance/*.sh $pkgdir/usr/share/org.koompi.sel/maintenance/
    install -Dm644 $srcdir/sel/operation.yml $pkgdir/usr/share/org.koompi.sel/
    install -Dm644 $srcdir/sel/gui-client/build/sel.svg $pkgdir/usr/share/icons/koompi/sel.svg

}
