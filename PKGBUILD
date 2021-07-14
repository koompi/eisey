# Maintainer <Brilliant PHAL, phalbrilliant@gmail.com>
pkgname=sel
pkgver=0.1.0
pkgrel=1
pkgdesc="Simplify package management and maintenance in KOOMPI OS"
arch=('x86_64')
license=('GPL3')
source=('git+https://github.com/koompi/sel.git')
depends=('glibc' 'rustup' 'gnupg' 'pgpme' 'xdg-utils')
install=$pkgname.install
package() {
    
    cd $srcdir/sel
    rustup default stable
    cargo build --release
    
    mkdir -p $pkgdir/usr/bin
    mkdir -p $pkgdir/usr/share/applications
    
    install -Dm755 $srcdir/sel/target/release/sel $pkgdir/usr/bin/sel
    install -Dm644 $srcdir/sel/sel.desktop $pkgdir/usr/share/applications/sel.desktop
}
