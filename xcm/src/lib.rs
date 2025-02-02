// Copyright 2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Cross-Consensus Message format data structures.

// NOTE, this crate is meant to be used in many different environments, notably wasm, but not
// necessarily related to FRAME or even Substrate.
//
// Hence, `no_std` rather than sp-runtime.
#![no_std]
extern crate alloc;

use alloc::vec::Vec;
use core::{
	convert::{TryFrom, TryInto},
	result::Result,
};
use derivative::Derivative;
use parity_scale_codec::{Decode, Encode, Error as CodecError, Input};

pub mod v0;
pub mod v1;

pub mod latest {
	pub use super::v1::*;
}

mod double_encoded;
pub use double_encoded::DoubleEncoded;

/// Maximum nesting level for XCM decoding.
pub const MAX_XCM_DECODE_DEPTH: u32 = 8;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Unsupported {}
impl Encode for Unsupported {}
impl Decode for Unsupported {
	fn decode<I: Input>(_: &mut I) -> Result<Self, CodecError> {
		Err("Not decodable".into())
	}
}

/// A single `MultiLocation` value, together with its version code.
#[derive(Derivative, Encode, Decode)]
#[derivative(Clone(bound = ""), Eq(bound = ""), PartialEq(bound = ""), Debug(bound = ""))]
#[codec(encode_bound())]
#[codec(decode_bound())]
pub enum VersionedMultiLocation {
	V0(v0::MultiLocation),
	V1(v1::MultiLocation),
}

impl From<v0::MultiLocation> for VersionedMultiLocation {
	fn from(x: v0::MultiLocation) -> Self {
		VersionedMultiLocation::V0(x)
	}
}

impl<T: Into<v1::MultiLocation>> From<T> for VersionedMultiLocation {
	fn from(x: T) -> Self {
		VersionedMultiLocation::V1(x.into())
	}
}

impl TryFrom<VersionedMultiLocation> for v0::MultiLocation {
	type Error = ();
	fn try_from(x: VersionedMultiLocation) -> Result<Self, ()> {
		use VersionedMultiLocation::*;
		match x {
			V0(x) => Ok(x),
			V1(x) => x.try_into(),
		}
	}
}

impl TryFrom<VersionedMultiLocation> for v1::MultiLocation {
	type Error = ();
	fn try_from(x: VersionedMultiLocation) -> Result<Self, ()> {
		use VersionedMultiLocation::*;
		match x {
			V0(x) => x.try_into(),
			V1(x) => Ok(x),
		}
	}
}

/// A single `MultiAsset` value, together with its version code.
#[derive(Derivative, Encode, Decode)]
#[derivative(Clone(bound = ""), Eq(bound = ""), PartialEq(bound = ""), Debug(bound = ""))]
#[codec(encode_bound())]
#[codec(decode_bound())]
pub enum VersionedMultiAsset {
	V0(v0::MultiAsset),
	V1(v1::MultiAsset),
}

impl From<v0::MultiAsset> for VersionedMultiAsset {
	fn from(x: v0::MultiAsset) -> Self {
		VersionedMultiAsset::V0(x)
	}
}

impl<T: Into<v1::MultiAsset>> From<T> for VersionedMultiAsset {
	fn from(x: T) -> Self {
		VersionedMultiAsset::V1(x.into())
	}
}

impl TryFrom<VersionedMultiAsset> for v0::MultiAsset {
	type Error = ();
	fn try_from(x: VersionedMultiAsset) -> Result<Self, ()> {
		use VersionedMultiAsset::*;
		match x {
			V0(x) => Ok(x),
			V1(x) => x.try_into(),
		}
	}
}

impl TryFrom<VersionedMultiAsset> for v1::MultiAsset {
	type Error = ();
	fn try_from(x: VersionedMultiAsset) -> Result<Self, ()> {
		use VersionedMultiAsset::*;
		match x {
			V0(x) => x.try_into(),
			V1(x) => Ok(x),
		}
	}
}

/// A single `MultiAssets` value, together with its version code.
///
/// NOTE: For XCM v0, this was `Vec<MultiAsset>`.
#[derive(Derivative, Encode, Decode)]
#[derivative(Clone(bound = ""), Eq(bound = ""), PartialEq(bound = ""), Debug(bound = ""))]
#[codec(encode_bound())]
#[codec(decode_bound())]
pub enum VersionedMultiAssets {
	V0(Vec<v0::MultiAsset>),
	V1(v1::MultiAssets),
}

impl From<Vec<v0::MultiAsset>> for VersionedMultiAssets {
	fn from(x: Vec<v0::MultiAsset>) -> Self {
		VersionedMultiAssets::V0(x)
	}
}

impl<T: Into<v1::MultiAssets>> From<T> for VersionedMultiAssets {
	fn from(x: T) -> Self {
		VersionedMultiAssets::V1(x.into())
	}
}

impl TryFrom<VersionedMultiAssets> for Vec<v0::MultiAsset> {
	type Error = ();
	fn try_from(x: VersionedMultiAssets) -> Result<Self, ()> {
		use VersionedMultiAssets::*;
		match x {
			V0(x) => Ok(x),
			V1(x) => x.try_into(),
		}
	}
}

impl TryFrom<VersionedMultiAssets> for v1::MultiAssets {
	type Error = ();
	fn try_from(x: VersionedMultiAssets) -> Result<Self, ()> {
		use VersionedMultiAssets::*;
		match x {
			V0(x) => x.try_into(),
			V1(x) => Ok(x),
		}
	}
}

/// A single XCM message, together with its version code.
#[derive(Derivative, Encode, Decode)]
#[derivative(Clone(bound = ""), Eq(bound = ""), PartialEq(bound = ""), Debug(bound = ""))]
#[codec(encode_bound())]
#[codec(decode_bound())]
pub enum VersionedXcm<Call> {
	V0(v0::Xcm<Call>),
	V1(v1::Xcm<Call>),
}

impl<Call> From<v0::Xcm<Call>> for VersionedXcm<Call> {
	fn from(x: v0::Xcm<Call>) -> Self {
		VersionedXcm::V0(x)
	}
}

impl<Call> From<v1::Xcm<Call>> for VersionedXcm<Call> {
	fn from(x: v1::Xcm<Call>) -> Self {
		VersionedXcm::V1(x)
	}
}

impl<Call> TryFrom<VersionedXcm<Call>> for v0::Xcm<Call> {
	type Error = ();
	fn try_from(x: VersionedXcm<Call>) -> Result<Self, ()> {
		match x {
			VersionedXcm::V0(x) => Ok(x),
			VersionedXcm::V1(x) => x.try_into(),
		}
	}
}

impl<Call> TryFrom<VersionedXcm<Call>> for v1::Xcm<Call> {
	type Error = ();
	fn try_from(x: VersionedXcm<Call>) -> Result<Self, ()> {
		match x {
			VersionedXcm::V0(x) => x.try_into(),
			VersionedXcm::V1(x) => Ok(x),
		}
	}
}

/// Convert an `Xcm` datum into a `VersionedXcm`, based on a destination `MultiLocation` which will interpret it.
pub trait WrapVersion {
	fn wrap_version<Call>(
		dest: &latest::MultiLocation,
		xcm: impl Into<VersionedXcm<Call>>,
	) -> Result<VersionedXcm<Call>, ()>;
}

/// `()` implementation does nothing with the XCM, just sending with whatever version it was authored as.
impl WrapVersion for () {
	fn wrap_version<Call>(
		_: &latest::MultiLocation,
		xcm: impl Into<VersionedXcm<Call>>,
	) -> Result<VersionedXcm<Call>, ()> {
		Ok(xcm.into())
	}
}

/// `WrapVersion` implementation which attempts to always convert the XCM to version 0 before wrapping it.
pub struct AlwaysV0;
impl WrapVersion for AlwaysV0 {
	fn wrap_version<Call>(
		_: &latest::MultiLocation,
		xcm: impl Into<VersionedXcm<Call>>,
	) -> Result<VersionedXcm<Call>, ()> {
		Ok(VersionedXcm::<Call>::V0(xcm.into().try_into()?))
	}
}

/// `WrapVersion` implementation which attempts to always convert the XCM to version 1 before wrapping it.
pub struct AlwaysV1;
impl WrapVersion for AlwaysV1 {
	fn wrap_version<Call>(
		_: &latest::MultiLocation,
		xcm: impl Into<VersionedXcm<Call>>,
	) -> Result<VersionedXcm<Call>, ()> {
		Ok(VersionedXcm::<Call>::V1(xcm.into().try_into()?))
	}
}

/// `WrapVersion` implementation which attempts to always convert the XCM to the latest version before wrapping it.
pub type AlwaysLatest = AlwaysV1;

/// `WrapVersion` implementation which attempts to always convert the XCM to the release version before wrapping it.
pub type AlwaysRelease = AlwaysV0;

pub mod opaque {
	pub mod v0 {
		// Everything from v0
		pub use crate::v0::*;
		// Then override with the opaque types in v0
		pub use crate::v0::opaque::{Order, Xcm};
	}
	pub mod v1 {
		// Everything from v1
		pub use crate::v1::*;
		// Then override with the opaque types in v1
		pub use crate::v1::opaque::{Order, Xcm};
	}

	pub mod latest {
		pub use super::v1::*;
	}

	/// The basic `VersionedXcm` type which just uses the `Vec<u8>` as an encoded call.
	pub type VersionedXcm = super::VersionedXcm<()>;
}
