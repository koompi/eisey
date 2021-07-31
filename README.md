# SEL



## How to install

On KOOMPI Linux v2.6.0 *up*
```bash
pi -Sy sel
```

On KOOMPI Linux v2.6.0 *down*
```bash 
wget https://dev.koompi.org/koompi/sel-0.1.0-1-x86_64.pkg.tar.zst
pi -U sel-0.1.0-1-x86_64.pkg.tar.zst
```

## How to build and install

```bash
git clone https://github.com/koompi/sel.git --depth 1
cd sel
makepkg -si
```

