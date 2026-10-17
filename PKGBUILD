
pkgname=distroller
pkgver=0.1.0
pkgrel=1
pkgdesc="Hyprland display management CLI"
arch=('x86_64')
url="https://github.com/Erox-02/distroller"
license=('GPL-3.0-only')
depends=('glibc')
makedepends=('cargo')

source=("$pkgname-$pkgver.tar.gz::https://github.com/Erox-02/distroller/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

prepare() {
    cd "$srcdir/$pkgname-$pkgver"
    cargo fetch --locked
}

build() {
    cd "$srcdir/$pkgname-$pkgver"
    cargo build --release --frozen
}

package() {
    cd "$srcdir/$pkgname-$pkgver"
    install -Dm755 target/release/distroller "$pkgdir/usr/bin/distroller"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
