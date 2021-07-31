<center>
<h1>SEL PROTOCOL</h1>
<h3>Magic Protocol For Everyone</h3>
</center>

<center>
<img src="./gui-admin/public/logo.svg" alt="sel logo" />
</center>

## DESCRIPTION

SEL is a protocol enabling sucure remote execution through URI by utilizing XDG URL handler with end to end encrytion. KOOMPI created this helping users in trobleshooting their computing problem by just clicking on the SEL link issued by KOOMPI OS maintainers to deal with any problems users might have.

## INSTALLATION

### Uers

Firstly we need to install SEL protocol first.

```shell
sudo pacman -S sel-protol
```

**NOTES:**

If sel-protocol is not available, then you will need include KOOMPI's repository to your `/etc/pacman.conf`

### ADMIN

```shell
sudo pacman -S sel-protol sel-admin
```

## USAGE

For **users**, when you have any problems just report your problems to KOOMPI admins through our [Telegram community]("https://t.me/koompi"). After that you just need open SEL Wallet application, sign up, and copy your public key for KOOMPI's admin when they ask for. After that they will give you a link. Just click on the link then your default browser will handle the link. Firefox is recommended.

For **admins**, firstly when user report problem, just open SEL Admin application, sign in to your account, ask users for the PUBLIC KEY, and paste in the the public key input box. Secondly, you need to input SEL URL as belowd. After that click on sign, then you get a HTTP link for your to use. The link is only work for one user since it signed by SEL server with user's public key.

**SEL URI**

URI without parameters

```shell
sel://pacman/update
sel://pi/update
sel://pix/update
```

URI with parameters

```
sel://pacman/install?apps=app_name1,app_name2,...,app_name_nth

sel://pacman/remove?apps=app_name1,app_name2,...,app_name_nth

sel://pi/install?apps=app_name1,app_name2,...,app_name_nth

sel://pi/remove?apps=app_name1,app_name2,...,app_name_nth

sel://pix/install?apps=app_name1,app_name2,...,app_name_nth

sel://pix/remove?apps=app_name1,app_name2,...,app_name_nth

# Grub maintenace
sel://maintenance?ops=grub_update
sel://maintenance?ops=grub_install_efi
sel://maintenance?ops=grub_install_bios

# Switching Kernel
sel://maintenance?ops=kernel_koompi
sel://maintenance?ops=kernel_arch
sel://maintenance?ops=kernel_lts
sel://maintenance?ops=kernel_zen

# Pacman
sel://maintenance?ops=pacman_cache
sel://maintenance?ops=pacman_reinstall_all

# Plasma
sel://maintenance?ops=plasma_refresh_all
sel://maintenance?ops=plasma_refresh_panel
sel://maintenance?ops=plasma_refresh_widget
sel://maintenance?ops=plasma_restore_default

```

## DEVELOPMENT

You can register more maintenance operations by just registering the operation name in `operations.yml` and add a script for it in [maintenace directory](https://github.com/koompi/sel/tree/main/maintenance).
