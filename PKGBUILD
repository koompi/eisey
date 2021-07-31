# Maintainer <Brilliant PHAL, phalbrilliant@gmail.com>
pkgname=('sel-protocol' 'sel-admin')
pkgbase=sel
pkgver=0.1.0
arch=('x86_64')
pkgrel=1
license=('GPL3')
source=('git+https://github.com/koompi/sel.git')
sha256sums=('SKIP')
depends=('glibc' 'rustup' 'gnupg' 'xdg-utils')
makedepends=('nodejs' 'npm')


build() {
    sed -i 's/ maintenance/ \/usr\/share\/org.koompi.sel\/maintenance/g' $srcdir/sel/operation.yml
    sed -i 's/http:\/\/localhost:8080/https:\/\/sel.koompi.org/g' $srcdir/sel/gui-admin/src/Home.js
    sed -i 's/http:\/\/localhost:8080/https:\/\/sel.koompi.org/g' $srcdir/sel/gui-admin/src/SignIn.js
    cd $srcdir/sel
    rustup default stable
    
    # build client
    cd $srcdir/sel/gui-client
    npm install
    npm run build
    cd $srcdir/sel
    cargo build --release --bin user-gui

    # build amdin
    cd $srcdir/sel/gui-admin
    npm install
    npm run build
    cd $srcdir/sel
    cargo build --release --bin issuer-gui
    
    # build cli
    cargo build --release --bin user-cli
}

package_sel-protocol() {
    pkgname=$pkgbase-protocol
    
    pkgdesc="Simplify package management and maintenance in KOOMPI OS"
    install=$pkgname.install
    cd $srcdir/sel
#     sed -i 's/ maintenance/ \/usr\/share\/org.koompi.sel\/maintenance/g' $srcdir/sel/operation.yml    
    
    mkdir -p $pkgdir/usr/bin
    mkdir -p $pkgdir/usr/share/applications
    mkdir -p $pkgdir/usr/share/org.koompi.sel/maintenance
    
    install -Dm755 $srcdir/sel/target/release/user-cli $pkgdir/usr/bin/sel-protocol
    install -Dm755 $srcdir/sel/target/release/user-gui $pkgdir/usr/bin/sel-wallet
    install -Dm777 $srcdir/sel/sel.desktop $pkgdir/usr/share/applications/sel.desktop
    install -Dm777 $srcdir/sel/sel-protocol.desktop $pkgdir/usr/share/applications/sel-protocol.desktop
    install -Dm755 $srcdir/sel/maintenance/*.sh $pkgdir/usr/share/org.koompi.sel/maintenance/
    install -Dm644 $srcdir/sel/operation.yml $pkgdir/usr/share/org.koompi.sel/
    install -Dm644 $srcdir/sel/gui-client/build/sel.svg $pkgdir/usr/share/icons/koompi/sel.svg
    install -Dm644 $srcdir/sel/pubkey $pkgdir/usr/share/org.koompi.sel/pub.key

}

package_sel-admin() {
    pkgname=$pkgbase-admin
    pkgdesc="Tool for generation sel protocol url"
    mkdir -p $pkgdir/usr/bin
    mkdir -p $pkgdir/usr/share/applications
    mkdir -p $pkgdir/usr/share/icons/koompi
    cd $srcdir/sel
    install -Dm777 $srcdir/sel/sel-admin.desktop $pkgdir/usr/share/applications/sel-admin.desktop
    install -Dm755 $srcdir/sel/target/release/issuer-gui $pkgdir/usr/bin/sel-admin
    install -Dm644 $srcdir/sel/gui-admin/build/sel.svg $pkgdir/usr/share/icons/koompi/sel-admin.svg
}
