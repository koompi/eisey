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

buidl() {
    sed -i 's/ maintenance/ \/usr\/share\/org.koompi.sel\/maintenance/g' $srcdir/$pkgname/operation.yml
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
    
    # build server
    cargo build --release --package backend
}

package_sel-protocol() {
    cd $srcdir/sel
    sed -i 's/ maintenance/ \/usr\/share\/org.koompi.sel\/maintenance/g' $srcdir/$pkgname/operation.yml    
    
    mkdir -p $pkgdir/usr/bin
    mkdir -p $pkgdir/usr/share/applications
    mkdir -p $pkgdir/usr/share/org.koompi.sel/maintenance
    
    install -Dm755 $srcdir/sel/target/release/user-cli $pkgdir/usr/bin/sel-protocol
    install -Dm755 $srcdir/sel/target/release/user-gui $pkgdir/usr/bin/sel-wallet
    install -Dm777 $srcdir/sel/sel.desktop $pkgdir/usr/share/applications/sel.desktop
    install -Dm777 $srcdir/sel/sel-protocol.desktop $pkgdir/usr/share/applications/sel-protocol.desktop
    install -Dm755 $srcdir/sel/maintenance/*.sh $pkgdir/usr/share/org.koompi.sel/maintenance/
    install -Dm644 $srcdir/sel/operation.yml $pkgdir/usr/share/org.koompi.sel/
    install -Dm644 $srcdir/sel/gui-user/build/sel.svg $pkgdir/usr/share/icons/koompi/sel.svg
    install -Dm644 $srcdir/sel/pubkey $pkgdir/usr/share/org.koompi.sel/pub.key

}

package_sel-admin() {
    mkdir -p $pkgdir/usr/bin
    mkdir -p $pkgdir/usr/share/applications
    mkdir -p $pkgdir/usr/share/icons/koompi
    cd $srcdir/sel
    install -Dm777 $srcdir/sel/sel-admin.desktop $pkgdir/usr/share/applications/sel-admin.desktop
    install -Dm755 $srcdir/sel/target/release/issuer-gui $pkgdir/usr/bin/sel-admin
    install -Dm644 $srcdir/sel/gui-admin/build/sel.svg $pkgdir/usr/share/icons/koompi/sel-admin.svg
}


# # Maintainer <Brilliant PHAL, phalbrilliant@gmail.com>
# pkgname=sel
# pkgver=0.1.0
# pkgrel=1
# pkgdesc="Simplify package management and maintenance in KOOMPI OS"
# arch=('x86_64')
# license=('GPL3')
# source=('git+https://github.com/koompi/sel.git')
# sha256sums=('SKIP')
# depends=('glibc' 'rustup' 'gnupg' 'xdg-utils')
# makedepends=('nodejs' 'npm')
# install=$pkgname.install

# package() {
#     sed -i 's/ maintenance/ \/usr\/share\/org.koompi.sel\/maintenance/g' $srcdir/$pkgname/operation.yml
#     cd $srcdir/sel
#     rustup default stable
#     cargo build --release --bin user-cli
#     cargo build --release --bin issuer-cli
#     cd $srcdir/sel/gui-client
#     npm install
#     npm run build
#     cd $srcdir/sel
#     cargo build --release --bin user-gui
    
#     mkdir -p $pkgdir/usr/bin
#     mkdir -p $pkgdir/usr/share/applications
#     mkdir -p $pkgdir/usr/share/org.koompi.sel/maintenance

#     install -Dm755 $srcdir/sel/target/release/user-cli $pkgdir/usr/bin/sel-protocol
#     install -Dm755 $srcdir/sel/target/release/user-gui $pkgdir/usr/bin/sel-wallet
#     install -Dm755 $srcdir/sel/target/release/issuer-cli $pkgdir/usr/bin/sel-issuer
#     install -Dm644 $srcdir/sel/sel.desktop $pkgdir/usr/share/applications/sel.desktop
#     install -Dm644 $srcdir/sel/sel-protocol.desktop $pkgdir/usr/share/applications/sel-protocol.desktop
#     install -Dm755 $srcdir/sel/maintenance/*.sh $pkgdir/usr/share/org.koompi.sel/maintenance/
#     install -Dm644 $srcdir/sel/operation.yml $pkgdir/usr/share/org.koompi.sel/
#     install -Dm644 $srcdir/sel/gui-client/build/sel.svg $pkgdir/usr/share/icons/koompi/sel.svg

# }
