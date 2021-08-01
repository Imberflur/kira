pub mod producer;

use crate::{
	clock::{Clock, ClockId},
	parameter::{tween::Tween, Parameter, ParameterId},
	sound::{
		instance::{Instance, InstanceId},
		Sound, SoundId,
	},
	track::{SubTrackId, Track, TrackId},
	value::Value,
};

pub(crate) enum SoundCommand {
	Add(SoundId, Sound),
}

pub(crate) enum InstanceCommand {
	Add(InstanceId, Instance),
	SetVolume(InstanceId, Value),
	SetPlaybackRate(InstanceId, Value),
	SetPanning(InstanceId, Value),
	Pause { id: InstanceId, tween: Tween },
	Resume { id: InstanceId, tween: Tween },
	Stop { id: InstanceId, tween: Tween },
}

pub(crate) enum ParameterCommand {
	Add(ParameterId, Parameter),
	Set {
		id: ParameterId,
		target: f64,
		tween: Tween,
	},
	Pause(ParameterId),
	Resume(ParameterId),
}

pub(crate) enum MixerCommand {
	AddSubTrack(SubTrackId, Track),
	SetTrackVolume(TrackId, Value),
	SetTrackPanning(TrackId, Value),
}

pub(crate) enum ClockCommand {
	Add(ClockId, Clock),
	SetInterval(ClockId, Value),
	Start(ClockId),
	Pause(ClockId),
	Stop(ClockId),
}

pub(crate) enum Command {
	Sound(SoundCommand),
	Instance(InstanceCommand),
	Parameter(ParameterCommand),
	Mixer(MixerCommand),
	Clock(ClockCommand),
	Pause(Tween),
	Resume(Tween),
}
