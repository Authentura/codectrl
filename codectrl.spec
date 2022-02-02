Name:           codectrl
Version:        0.1.0
Release:        1
Summary:        A language agnostic logger program.
License:        MIT
BuildRequires:  gobject-introspection-devel cairo-devel atk-devel pango-devel gdk-pixbuf2-devel gtk3-devel gcc
Source0:        https://github.com/pwnCTRL/codectrl/archive/main.tar.gz#/codectrl.tar.gz

%description
The language agnostic logger program for developers, testers and security personnel alike.

%prep
%autosetup -n %{name}-main

%install
rm -rf $RPM_BUILD_ROOT
mkdir -p $RPM_BUILD_ROOT%{_bindir}
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rustup.sh
chmod +x /tmp/rustup.sh
/tmp/rustup.sh --default-toolchain nightly -y
cargo build --release
strip --strip-all target/release/codectrl
cp target/release/codectrl $RPM_BUILD_ROOT%{_bindir}

%files
%{_bindir}/codectrl