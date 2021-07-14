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
    sed -i 's/ maintenance/ \/usr\/share\/org.koompi.sel\/maintenance/g' $srcdir/$pkgname/operation.yml
    cd $srcdir/sel
    rustup default stable
    cargo build --release
    
    mkdir -p $pkgdir/usr/bin
    mkdir -p $pkgdir/usr/share/applications
    mkdir -p $pkgdir/usr/share/org.koompi.sel/maintenance

    install -Dm755 $srcdir/sel/target/release/sel $pkgdir/usr/bin/sel
    install -Dm644 $srcdir/sel/sel.desktop $pkgdir/usr/share/applications/sel.desktop
    install -Dm755 $srcdir/sel/maintenance/*.sh $pkgdir/usr/share/org.koompi.sel/maintenance/
    install -Dm644 $srcdir/sel/operation.yml $pkgdir/usr/share/org.koompi.sel/

}
