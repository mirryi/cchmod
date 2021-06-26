# Copyright 2017-2021 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2

EAPI=7

CRATES="
	atty-0.2.14
	autocfg-1.0.1
	bitflags-1.2.1
	clap-3.0.0-beta.2
	clap_derive-3.0.0-beta.2
	hashbrown-0.9.1
	heck-0.3.3
	hermit-abi-0.1.18
	indexmap-1.6.2
	lazy_static-1.4.0
	libc-0.2.97
	os_str_bytes-2.4.0
	proc-macro-error-1.0.4
	proc-macro-error-attr-1.0.4
	proc-macro2-1.0.27
	quote-1.0.9
	strsim-0.10.0
	syn-1.0.73
	termcolor-1.1.2
	textwrap-0.12.1
	thiserror-1.0.25
	thiserror-impl-1.0.25
	unicode-segmentation-1.7.1
	unicode-width-0.1.8
	unicode-xid-0.2.2
	vec_map-0.8.2
	version_check-0.9.3
	winapi-0.3.9
	winapi-i686-pc-windows-gnu-0.4.0
	winapi-util-0.1.5
	winapi-x86_64-pc-windows-gnu-0.4.0
"

inherit cargo

DESCRIPTION="chmod calculator."
HOMEPAGE="https://github.com/Dophin2009/cchmod.git"
SRC_URI="$(cargo_crate_uris ${CRATES}) https://github.com/Dophin2009/${PN}/archive/refs/tags/v${PV}.tar.gz -> ${P}.tar.gz"

LICENSE="Apache-2.0 MIT Unlicense"
SLOT="0"
KEYWORDS="~amd64"
RESTRICT="mirror"
