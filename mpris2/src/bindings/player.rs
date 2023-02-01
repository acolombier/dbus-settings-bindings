// SPDX-License-Identifier: MPL-2.0
//! # DBus interface proxies for: `org.mpris.MediaPlayer2.Player`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from DBus introspection data.
//! Source: `Interface '/org/mpris/MediaPlayer2' from service 'org.mpris.MediaPlayer2.firefox.instance103520' on session bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!
//! This DBus object implements
//! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
//! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
//!
//! * [`zbus::fdo::PropertiesProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PeerProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.

use crate::track::TrackId;
use zbus::dbus_proxy;

#[dbus_proxy(
	interface = "org.mpris.MediaPlayer2.Player",
	default_path = "/org/mpris/MediaPlayer2"
)]
trait Player {
	/// Next method
	fn next(&self) -> zbus::Result<()>;

	/// OpenUri method
	fn open_uri(&self, uri: &str) -> zbus::Result<()>;

	/// Pause method
	fn pause(&self) -> zbus::Result<()>;

	/// Play method
	fn play(&self) -> zbus::Result<()>;

	/// PlayPause method
	fn play_pause(&self) -> zbus::Result<()>;

	/// Previous method
	fn previous(&self) -> zbus::Result<()>;

	/// Seek method
	fn seek(&self, offset: i64) -> zbus::Result<()>;

	/// SetPosition method
	fn set_position(&self, track_id: &TrackId, position: i64) -> zbus::Result<()>;

	/// Stop method
	fn stop(&self) -> zbus::Result<()>;

	/// Seeked signal
	#[dbus_proxy(signal)]
	fn seeked(&self, position: i64) -> zbus::Result<()>;

	/// CanControl property
	#[dbus_proxy(property)]
	fn can_control(&self) -> zbus::Result<bool>;

	/// CanGoNext property
	#[dbus_proxy(property)]
	fn can_go_next(&self) -> zbus::Result<bool>;

	/// CanGoPrevious property
	#[dbus_proxy(property)]
	fn can_go_previous(&self) -> zbus::Result<bool>;

	/// CanPause property
	#[dbus_proxy(property)]
	fn can_pause(&self) -> zbus::Result<bool>;

	/// CanPlay property
	#[dbus_proxy(property)]
	fn can_play(&self) -> zbus::Result<bool>;

	/// CanSeek property
	#[dbus_proxy(property)]
	fn can_seek(&self) -> zbus::Result<bool>;

	/// MaximumRate property
	#[dbus_proxy(property)]
	fn maximum_rate(&self) -> zbus::Result<f64>;

	/// Metadata property
	#[dbus_proxy(property)]
	fn metadata(
		&self,
	) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

	/// MinimumRate property
	#[dbus_proxy(property)]
	fn minimum_rate(&self) -> zbus::Result<f64>;

	/// PlaybackStatus property
	#[dbus_proxy(property)]
	fn playback_status(&self) -> zbus::Result<String>;

	/// Position property
	#[dbus_proxy(property)]
	fn position(&self) -> zbus::Result<i64>;

	/// Rate property
	#[dbus_proxy(property)]
	fn rate(&self) -> zbus::Result<f64>;
	#[dbus_proxy(property)]
	fn set_rate(&self, value: f64) -> zbus::Result<()>;

	/// Shuffle property (optional)
	#[dbus_proxy(property)]
	fn shuffle(&self) -> zbus::Result<bool>;
	#[dbus_proxy(property)]
	fn set_shuffle(&self, value: bool) -> zbus::Result<()>;

	/// LoopStatus property (optional)
	#[dbus_proxy(property)]
	fn loop_status(&self) -> zbus::Result<String>;
	#[dbus_proxy(property)]
	fn set_loop_status(&self, value: String) -> zbus::Result<()>;

	/// Volume property
	#[dbus_proxy(property)]
	fn volume(&self) -> zbus::Result<f64>;
	#[dbus_proxy(property)]
	fn set_volume(&self, value: f64) -> zbus::Result<()>;
}
