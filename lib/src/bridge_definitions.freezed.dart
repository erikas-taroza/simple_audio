// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'bridge_definitions.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$Callback {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Error field0) error,
    required TResult Function() mediaControlSkipPrev,
    required TResult Function() mediaControlSkipNext,
    required TResult Function() playbackLooped,
    required TResult Function() durationCalculated,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Error field0)? error,
    TResult? Function()? mediaControlSkipPrev,
    TResult? Function()? mediaControlSkipNext,
    TResult? Function()? playbackLooped,
    TResult? Function()? durationCalculated,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Error field0)? error,
    TResult Function()? mediaControlSkipPrev,
    TResult Function()? mediaControlSkipNext,
    TResult Function()? playbackLooped,
    TResult Function()? durationCalculated,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Callback_Error value) error,
    required TResult Function(Callback_MediaControlSkipPrev value)
        mediaControlSkipPrev,
    required TResult Function(Callback_MediaControlSkipNext value)
        mediaControlSkipNext,
    required TResult Function(Callback_PlaybackLooped value) playbackLooped,
    required TResult Function(Callback_DurationCalculated value)
        durationCalculated,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Callback_Error value)? error,
    TResult? Function(Callback_MediaControlSkipPrev value)?
        mediaControlSkipPrev,
    TResult? Function(Callback_MediaControlSkipNext value)?
        mediaControlSkipNext,
    TResult? Function(Callback_PlaybackLooped value)? playbackLooped,
    TResult? Function(Callback_DurationCalculated value)? durationCalculated,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Callback_Error value)? error,
    TResult Function(Callback_MediaControlSkipPrev value)? mediaControlSkipPrev,
    TResult Function(Callback_MediaControlSkipNext value)? mediaControlSkipNext,
    TResult Function(Callback_PlaybackLooped value)? playbackLooped,
    TResult Function(Callback_DurationCalculated value)? durationCalculated,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CallbackCopyWith<$Res> {
  factory $CallbackCopyWith(Callback value, $Res Function(Callback) then) =
      _$CallbackCopyWithImpl<$Res, Callback>;
}

/// @nodoc
class _$CallbackCopyWithImpl<$Res, $Val extends Callback>
    implements $CallbackCopyWith<$Res> {
  _$CallbackCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$Callback_ErrorCopyWith<$Res> {
  factory _$$Callback_ErrorCopyWith(
          _$Callback_Error value, $Res Function(_$Callback_Error) then) =
      __$$Callback_ErrorCopyWithImpl<$Res>;
  @useResult
  $Res call({Error field0});

  $ErrorCopyWith<$Res> get field0;
}

/// @nodoc
class __$$Callback_ErrorCopyWithImpl<$Res>
    extends _$CallbackCopyWithImpl<$Res, _$Callback_Error>
    implements _$$Callback_ErrorCopyWith<$Res> {
  __$$Callback_ErrorCopyWithImpl(
      _$Callback_Error _value, $Res Function(_$Callback_Error) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Callback_Error(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Error,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ErrorCopyWith<$Res> get field0 {
    return $ErrorCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$Callback_Error implements Callback_Error {
  const _$Callback_Error(this.field0);

  @override
  final Error field0;

  @override
  String toString() {
    return 'Callback.error(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Callback_Error &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Callback_ErrorCopyWith<_$Callback_Error> get copyWith =>
      __$$Callback_ErrorCopyWithImpl<_$Callback_Error>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Error field0) error,
    required TResult Function() mediaControlSkipPrev,
    required TResult Function() mediaControlSkipNext,
    required TResult Function() playbackLooped,
    required TResult Function() durationCalculated,
  }) {
    return error(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Error field0)? error,
    TResult? Function()? mediaControlSkipPrev,
    TResult? Function()? mediaControlSkipNext,
    TResult? Function()? playbackLooped,
    TResult? Function()? durationCalculated,
  }) {
    return error?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Error field0)? error,
    TResult Function()? mediaControlSkipPrev,
    TResult Function()? mediaControlSkipNext,
    TResult Function()? playbackLooped,
    TResult Function()? durationCalculated,
    required TResult orElse(),
  }) {
    if (error != null) {
      return error(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Callback_Error value) error,
    required TResult Function(Callback_MediaControlSkipPrev value)
        mediaControlSkipPrev,
    required TResult Function(Callback_MediaControlSkipNext value)
        mediaControlSkipNext,
    required TResult Function(Callback_PlaybackLooped value) playbackLooped,
    required TResult Function(Callback_DurationCalculated value)
        durationCalculated,
  }) {
    return error(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Callback_Error value)? error,
    TResult? Function(Callback_MediaControlSkipPrev value)?
        mediaControlSkipPrev,
    TResult? Function(Callback_MediaControlSkipNext value)?
        mediaControlSkipNext,
    TResult? Function(Callback_PlaybackLooped value)? playbackLooped,
    TResult? Function(Callback_DurationCalculated value)? durationCalculated,
  }) {
    return error?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Callback_Error value)? error,
    TResult Function(Callback_MediaControlSkipPrev value)? mediaControlSkipPrev,
    TResult Function(Callback_MediaControlSkipNext value)? mediaControlSkipNext,
    TResult Function(Callback_PlaybackLooped value)? playbackLooped,
    TResult Function(Callback_DurationCalculated value)? durationCalculated,
    required TResult orElse(),
  }) {
    if (error != null) {
      return error(this);
    }
    return orElse();
  }
}

abstract class Callback_Error implements Callback {
  const factory Callback_Error(final Error field0) = _$Callback_Error;

  Error get field0;
  @JsonKey(ignore: true)
  _$$Callback_ErrorCopyWith<_$Callback_Error> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Callback_MediaControlSkipPrevCopyWith<$Res> {
  factory _$$Callback_MediaControlSkipPrevCopyWith(
          _$Callback_MediaControlSkipPrev value,
          $Res Function(_$Callback_MediaControlSkipPrev) then) =
      __$$Callback_MediaControlSkipPrevCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Callback_MediaControlSkipPrevCopyWithImpl<$Res>
    extends _$CallbackCopyWithImpl<$Res, _$Callback_MediaControlSkipPrev>
    implements _$$Callback_MediaControlSkipPrevCopyWith<$Res> {
  __$$Callback_MediaControlSkipPrevCopyWithImpl(
      _$Callback_MediaControlSkipPrev _value,
      $Res Function(_$Callback_MediaControlSkipPrev) _then)
      : super(_value, _then);
}

/// @nodoc

class _$Callback_MediaControlSkipPrev implements Callback_MediaControlSkipPrev {
  const _$Callback_MediaControlSkipPrev();

  @override
  String toString() {
    return 'Callback.mediaControlSkipPrev()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Callback_MediaControlSkipPrev);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Error field0) error,
    required TResult Function() mediaControlSkipPrev,
    required TResult Function() mediaControlSkipNext,
    required TResult Function() playbackLooped,
    required TResult Function() durationCalculated,
  }) {
    return mediaControlSkipPrev();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Error field0)? error,
    TResult? Function()? mediaControlSkipPrev,
    TResult? Function()? mediaControlSkipNext,
    TResult? Function()? playbackLooped,
    TResult? Function()? durationCalculated,
  }) {
    return mediaControlSkipPrev?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Error field0)? error,
    TResult Function()? mediaControlSkipPrev,
    TResult Function()? mediaControlSkipNext,
    TResult Function()? playbackLooped,
    TResult Function()? durationCalculated,
    required TResult orElse(),
  }) {
    if (mediaControlSkipPrev != null) {
      return mediaControlSkipPrev();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Callback_Error value) error,
    required TResult Function(Callback_MediaControlSkipPrev value)
        mediaControlSkipPrev,
    required TResult Function(Callback_MediaControlSkipNext value)
        mediaControlSkipNext,
    required TResult Function(Callback_PlaybackLooped value) playbackLooped,
    required TResult Function(Callback_DurationCalculated value)
        durationCalculated,
  }) {
    return mediaControlSkipPrev(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Callback_Error value)? error,
    TResult? Function(Callback_MediaControlSkipPrev value)?
        mediaControlSkipPrev,
    TResult? Function(Callback_MediaControlSkipNext value)?
        mediaControlSkipNext,
    TResult? Function(Callback_PlaybackLooped value)? playbackLooped,
    TResult? Function(Callback_DurationCalculated value)? durationCalculated,
  }) {
    return mediaControlSkipPrev?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Callback_Error value)? error,
    TResult Function(Callback_MediaControlSkipPrev value)? mediaControlSkipPrev,
    TResult Function(Callback_MediaControlSkipNext value)? mediaControlSkipNext,
    TResult Function(Callback_PlaybackLooped value)? playbackLooped,
    TResult Function(Callback_DurationCalculated value)? durationCalculated,
    required TResult orElse(),
  }) {
    if (mediaControlSkipPrev != null) {
      return mediaControlSkipPrev(this);
    }
    return orElse();
  }
}

abstract class Callback_MediaControlSkipPrev implements Callback {
  const factory Callback_MediaControlSkipPrev() =
      _$Callback_MediaControlSkipPrev;
}

/// @nodoc
abstract class _$$Callback_MediaControlSkipNextCopyWith<$Res> {
  factory _$$Callback_MediaControlSkipNextCopyWith(
          _$Callback_MediaControlSkipNext value,
          $Res Function(_$Callback_MediaControlSkipNext) then) =
      __$$Callback_MediaControlSkipNextCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Callback_MediaControlSkipNextCopyWithImpl<$Res>
    extends _$CallbackCopyWithImpl<$Res, _$Callback_MediaControlSkipNext>
    implements _$$Callback_MediaControlSkipNextCopyWith<$Res> {
  __$$Callback_MediaControlSkipNextCopyWithImpl(
      _$Callback_MediaControlSkipNext _value,
      $Res Function(_$Callback_MediaControlSkipNext) _then)
      : super(_value, _then);
}

/// @nodoc

class _$Callback_MediaControlSkipNext implements Callback_MediaControlSkipNext {
  const _$Callback_MediaControlSkipNext();

  @override
  String toString() {
    return 'Callback.mediaControlSkipNext()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Callback_MediaControlSkipNext);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Error field0) error,
    required TResult Function() mediaControlSkipPrev,
    required TResult Function() mediaControlSkipNext,
    required TResult Function() playbackLooped,
    required TResult Function() durationCalculated,
  }) {
    return mediaControlSkipNext();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Error field0)? error,
    TResult? Function()? mediaControlSkipPrev,
    TResult? Function()? mediaControlSkipNext,
    TResult? Function()? playbackLooped,
    TResult? Function()? durationCalculated,
  }) {
    return mediaControlSkipNext?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Error field0)? error,
    TResult Function()? mediaControlSkipPrev,
    TResult Function()? mediaControlSkipNext,
    TResult Function()? playbackLooped,
    TResult Function()? durationCalculated,
    required TResult orElse(),
  }) {
    if (mediaControlSkipNext != null) {
      return mediaControlSkipNext();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Callback_Error value) error,
    required TResult Function(Callback_MediaControlSkipPrev value)
        mediaControlSkipPrev,
    required TResult Function(Callback_MediaControlSkipNext value)
        mediaControlSkipNext,
    required TResult Function(Callback_PlaybackLooped value) playbackLooped,
    required TResult Function(Callback_DurationCalculated value)
        durationCalculated,
  }) {
    return mediaControlSkipNext(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Callback_Error value)? error,
    TResult? Function(Callback_MediaControlSkipPrev value)?
        mediaControlSkipPrev,
    TResult? Function(Callback_MediaControlSkipNext value)?
        mediaControlSkipNext,
    TResult? Function(Callback_PlaybackLooped value)? playbackLooped,
    TResult? Function(Callback_DurationCalculated value)? durationCalculated,
  }) {
    return mediaControlSkipNext?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Callback_Error value)? error,
    TResult Function(Callback_MediaControlSkipPrev value)? mediaControlSkipPrev,
    TResult Function(Callback_MediaControlSkipNext value)? mediaControlSkipNext,
    TResult Function(Callback_PlaybackLooped value)? playbackLooped,
    TResult Function(Callback_DurationCalculated value)? durationCalculated,
    required TResult orElse(),
  }) {
    if (mediaControlSkipNext != null) {
      return mediaControlSkipNext(this);
    }
    return orElse();
  }
}

abstract class Callback_MediaControlSkipNext implements Callback {
  const factory Callback_MediaControlSkipNext() =
      _$Callback_MediaControlSkipNext;
}

/// @nodoc
abstract class _$$Callback_PlaybackLoopedCopyWith<$Res> {
  factory _$$Callback_PlaybackLoopedCopyWith(_$Callback_PlaybackLooped value,
          $Res Function(_$Callback_PlaybackLooped) then) =
      __$$Callback_PlaybackLoopedCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Callback_PlaybackLoopedCopyWithImpl<$Res>
    extends _$CallbackCopyWithImpl<$Res, _$Callback_PlaybackLooped>
    implements _$$Callback_PlaybackLoopedCopyWith<$Res> {
  __$$Callback_PlaybackLoopedCopyWithImpl(_$Callback_PlaybackLooped _value,
      $Res Function(_$Callback_PlaybackLooped) _then)
      : super(_value, _then);
}

/// @nodoc

class _$Callback_PlaybackLooped implements Callback_PlaybackLooped {
  const _$Callback_PlaybackLooped();

  @override
  String toString() {
    return 'Callback.playbackLooped()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Callback_PlaybackLooped);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Error field0) error,
    required TResult Function() mediaControlSkipPrev,
    required TResult Function() mediaControlSkipNext,
    required TResult Function() playbackLooped,
    required TResult Function() durationCalculated,
  }) {
    return playbackLooped();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Error field0)? error,
    TResult? Function()? mediaControlSkipPrev,
    TResult? Function()? mediaControlSkipNext,
    TResult? Function()? playbackLooped,
    TResult? Function()? durationCalculated,
  }) {
    return playbackLooped?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Error field0)? error,
    TResult Function()? mediaControlSkipPrev,
    TResult Function()? mediaControlSkipNext,
    TResult Function()? playbackLooped,
    TResult Function()? durationCalculated,
    required TResult orElse(),
  }) {
    if (playbackLooped != null) {
      return playbackLooped();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Callback_Error value) error,
    required TResult Function(Callback_MediaControlSkipPrev value)
        mediaControlSkipPrev,
    required TResult Function(Callback_MediaControlSkipNext value)
        mediaControlSkipNext,
    required TResult Function(Callback_PlaybackLooped value) playbackLooped,
    required TResult Function(Callback_DurationCalculated value)
        durationCalculated,
  }) {
    return playbackLooped(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Callback_Error value)? error,
    TResult? Function(Callback_MediaControlSkipPrev value)?
        mediaControlSkipPrev,
    TResult? Function(Callback_MediaControlSkipNext value)?
        mediaControlSkipNext,
    TResult? Function(Callback_PlaybackLooped value)? playbackLooped,
    TResult? Function(Callback_DurationCalculated value)? durationCalculated,
  }) {
    return playbackLooped?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Callback_Error value)? error,
    TResult Function(Callback_MediaControlSkipPrev value)? mediaControlSkipPrev,
    TResult Function(Callback_MediaControlSkipNext value)? mediaControlSkipNext,
    TResult Function(Callback_PlaybackLooped value)? playbackLooped,
    TResult Function(Callback_DurationCalculated value)? durationCalculated,
    required TResult orElse(),
  }) {
    if (playbackLooped != null) {
      return playbackLooped(this);
    }
    return orElse();
  }
}

abstract class Callback_PlaybackLooped implements Callback {
  const factory Callback_PlaybackLooped() = _$Callback_PlaybackLooped;
}

/// @nodoc
abstract class _$$Callback_DurationCalculatedCopyWith<$Res> {
  factory _$$Callback_DurationCalculatedCopyWith(
          _$Callback_DurationCalculated value,
          $Res Function(_$Callback_DurationCalculated) then) =
      __$$Callback_DurationCalculatedCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Callback_DurationCalculatedCopyWithImpl<$Res>
    extends _$CallbackCopyWithImpl<$Res, _$Callback_DurationCalculated>
    implements _$$Callback_DurationCalculatedCopyWith<$Res> {
  __$$Callback_DurationCalculatedCopyWithImpl(
      _$Callback_DurationCalculated _value,
      $Res Function(_$Callback_DurationCalculated) _then)
      : super(_value, _then);
}

/// @nodoc

class _$Callback_DurationCalculated implements Callback_DurationCalculated {
  const _$Callback_DurationCalculated();

  @override
  String toString() {
    return 'Callback.durationCalculated()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Callback_DurationCalculated);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Error field0) error,
    required TResult Function() mediaControlSkipPrev,
    required TResult Function() mediaControlSkipNext,
    required TResult Function() playbackLooped,
    required TResult Function() durationCalculated,
  }) {
    return durationCalculated();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Error field0)? error,
    TResult? Function()? mediaControlSkipPrev,
    TResult? Function()? mediaControlSkipNext,
    TResult? Function()? playbackLooped,
    TResult? Function()? durationCalculated,
  }) {
    return durationCalculated?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Error field0)? error,
    TResult Function()? mediaControlSkipPrev,
    TResult Function()? mediaControlSkipNext,
    TResult Function()? playbackLooped,
    TResult Function()? durationCalculated,
    required TResult orElse(),
  }) {
    if (durationCalculated != null) {
      return durationCalculated();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Callback_Error value) error,
    required TResult Function(Callback_MediaControlSkipPrev value)
        mediaControlSkipPrev,
    required TResult Function(Callback_MediaControlSkipNext value)
        mediaControlSkipNext,
    required TResult Function(Callback_PlaybackLooped value) playbackLooped,
    required TResult Function(Callback_DurationCalculated value)
        durationCalculated,
  }) {
    return durationCalculated(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Callback_Error value)? error,
    TResult? Function(Callback_MediaControlSkipPrev value)?
        mediaControlSkipPrev,
    TResult? Function(Callback_MediaControlSkipNext value)?
        mediaControlSkipNext,
    TResult? Function(Callback_PlaybackLooped value)? playbackLooped,
    TResult? Function(Callback_DurationCalculated value)? durationCalculated,
  }) {
    return durationCalculated?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Callback_Error value)? error,
    TResult Function(Callback_MediaControlSkipPrev value)? mediaControlSkipPrev,
    TResult Function(Callback_MediaControlSkipNext value)? mediaControlSkipNext,
    TResult Function(Callback_PlaybackLooped value)? playbackLooped,
    TResult Function(Callback_DurationCalculated value)? durationCalculated,
    required TResult orElse(),
  }) {
    if (durationCalculated != null) {
      return durationCalculated(this);
    }
    return orElse();
  }
}

abstract class Callback_DurationCalculated implements Callback {
  const factory Callback_DurationCalculated() = _$Callback_DurationCalculated;
}

/// @nodoc
mixin _$Error {
  String get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) networkStream,
    required TResult Function(String field0) decode,
    required TResult Function(String field0) open,
    required TResult Function(String field0) preload,
    required TResult Function(String field0) playPreload,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? networkStream,
    TResult? Function(String field0)? decode,
    TResult? Function(String field0)? open,
    TResult? Function(String field0)? preload,
    TResult? Function(String field0)? playPreload,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? networkStream,
    TResult Function(String field0)? decode,
    TResult Function(String field0)? open,
    TResult Function(String field0)? preload,
    TResult Function(String field0)? playPreload,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Error_NetworkStream value) networkStream,
    required TResult Function(Error_Decode value) decode,
    required TResult Function(Error_Open value) open,
    required TResult Function(Error_Preload value) preload,
    required TResult Function(Error_PlayPreload value) playPreload,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Error_NetworkStream value)? networkStream,
    TResult? Function(Error_Decode value)? decode,
    TResult? Function(Error_Open value)? open,
    TResult? Function(Error_Preload value)? preload,
    TResult? Function(Error_PlayPreload value)? playPreload,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Error_NetworkStream value)? networkStream,
    TResult Function(Error_Decode value)? decode,
    TResult Function(Error_Open value)? open,
    TResult Function(Error_Preload value)? preload,
    TResult Function(Error_PlayPreload value)? playPreload,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $ErrorCopyWith<Error> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ErrorCopyWith<$Res> {
  factory $ErrorCopyWith(Error value, $Res Function(Error) then) =
      _$ErrorCopyWithImpl<$Res, Error>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$ErrorCopyWithImpl<$Res, $Val extends Error>
    implements $ErrorCopyWith<$Res> {
  _$ErrorCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_value.copyWith(
      field0: null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$Error_NetworkStreamCopyWith<$Res>
    implements $ErrorCopyWith<$Res> {
  factory _$$Error_NetworkStreamCopyWith(_$Error_NetworkStream value,
          $Res Function(_$Error_NetworkStream) then) =
      __$$Error_NetworkStreamCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$Error_NetworkStreamCopyWithImpl<$Res>
    extends _$ErrorCopyWithImpl<$Res, _$Error_NetworkStream>
    implements _$$Error_NetworkStreamCopyWith<$Res> {
  __$$Error_NetworkStreamCopyWithImpl(
      _$Error_NetworkStream _value, $Res Function(_$Error_NetworkStream) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Error_NetworkStream(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$Error_NetworkStream implements Error_NetworkStream {
  const _$Error_NetworkStream(this.field0);

  @override
  final String field0;

  @override
  String toString() {
    return 'Error.networkStream(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Error_NetworkStream &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Error_NetworkStreamCopyWith<_$Error_NetworkStream> get copyWith =>
      __$$Error_NetworkStreamCopyWithImpl<_$Error_NetworkStream>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) networkStream,
    required TResult Function(String field0) decode,
    required TResult Function(String field0) open,
    required TResult Function(String field0) preload,
    required TResult Function(String field0) playPreload,
  }) {
    return networkStream(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? networkStream,
    TResult? Function(String field0)? decode,
    TResult? Function(String field0)? open,
    TResult? Function(String field0)? preload,
    TResult? Function(String field0)? playPreload,
  }) {
    return networkStream?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? networkStream,
    TResult Function(String field0)? decode,
    TResult Function(String field0)? open,
    TResult Function(String field0)? preload,
    TResult Function(String field0)? playPreload,
    required TResult orElse(),
  }) {
    if (networkStream != null) {
      return networkStream(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Error_NetworkStream value) networkStream,
    required TResult Function(Error_Decode value) decode,
    required TResult Function(Error_Open value) open,
    required TResult Function(Error_Preload value) preload,
    required TResult Function(Error_PlayPreload value) playPreload,
  }) {
    return networkStream(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Error_NetworkStream value)? networkStream,
    TResult? Function(Error_Decode value)? decode,
    TResult? Function(Error_Open value)? open,
    TResult? Function(Error_Preload value)? preload,
    TResult? Function(Error_PlayPreload value)? playPreload,
  }) {
    return networkStream?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Error_NetworkStream value)? networkStream,
    TResult Function(Error_Decode value)? decode,
    TResult Function(Error_Open value)? open,
    TResult Function(Error_Preload value)? preload,
    TResult Function(Error_PlayPreload value)? playPreload,
    required TResult orElse(),
  }) {
    if (networkStream != null) {
      return networkStream(this);
    }
    return orElse();
  }
}

abstract class Error_NetworkStream implements Error {
  const factory Error_NetworkStream(final String field0) =
      _$Error_NetworkStream;

  @override
  String get field0;
  @override
  @JsonKey(ignore: true)
  _$$Error_NetworkStreamCopyWith<_$Error_NetworkStream> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Error_DecodeCopyWith<$Res> implements $ErrorCopyWith<$Res> {
  factory _$$Error_DecodeCopyWith(
          _$Error_Decode value, $Res Function(_$Error_Decode) then) =
      __$$Error_DecodeCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$Error_DecodeCopyWithImpl<$Res>
    extends _$ErrorCopyWithImpl<$Res, _$Error_Decode>
    implements _$$Error_DecodeCopyWith<$Res> {
  __$$Error_DecodeCopyWithImpl(
      _$Error_Decode _value, $Res Function(_$Error_Decode) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Error_Decode(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$Error_Decode implements Error_Decode {
  const _$Error_Decode(this.field0);

  @override
  final String field0;

  @override
  String toString() {
    return 'Error.decode(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Error_Decode &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Error_DecodeCopyWith<_$Error_Decode> get copyWith =>
      __$$Error_DecodeCopyWithImpl<_$Error_Decode>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) networkStream,
    required TResult Function(String field0) decode,
    required TResult Function(String field0) open,
    required TResult Function(String field0) preload,
    required TResult Function(String field0) playPreload,
  }) {
    return decode(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? networkStream,
    TResult? Function(String field0)? decode,
    TResult? Function(String field0)? open,
    TResult? Function(String field0)? preload,
    TResult? Function(String field0)? playPreload,
  }) {
    return decode?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? networkStream,
    TResult Function(String field0)? decode,
    TResult Function(String field0)? open,
    TResult Function(String field0)? preload,
    TResult Function(String field0)? playPreload,
    required TResult orElse(),
  }) {
    if (decode != null) {
      return decode(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Error_NetworkStream value) networkStream,
    required TResult Function(Error_Decode value) decode,
    required TResult Function(Error_Open value) open,
    required TResult Function(Error_Preload value) preload,
    required TResult Function(Error_PlayPreload value) playPreload,
  }) {
    return decode(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Error_NetworkStream value)? networkStream,
    TResult? Function(Error_Decode value)? decode,
    TResult? Function(Error_Open value)? open,
    TResult? Function(Error_Preload value)? preload,
    TResult? Function(Error_PlayPreload value)? playPreload,
  }) {
    return decode?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Error_NetworkStream value)? networkStream,
    TResult Function(Error_Decode value)? decode,
    TResult Function(Error_Open value)? open,
    TResult Function(Error_Preload value)? preload,
    TResult Function(Error_PlayPreload value)? playPreload,
    required TResult orElse(),
  }) {
    if (decode != null) {
      return decode(this);
    }
    return orElse();
  }
}

abstract class Error_Decode implements Error {
  const factory Error_Decode(final String field0) = _$Error_Decode;

  @override
  String get field0;
  @override
  @JsonKey(ignore: true)
  _$$Error_DecodeCopyWith<_$Error_Decode> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Error_OpenCopyWith<$Res> implements $ErrorCopyWith<$Res> {
  factory _$$Error_OpenCopyWith(
          _$Error_Open value, $Res Function(_$Error_Open) then) =
      __$$Error_OpenCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$Error_OpenCopyWithImpl<$Res>
    extends _$ErrorCopyWithImpl<$Res, _$Error_Open>
    implements _$$Error_OpenCopyWith<$Res> {
  __$$Error_OpenCopyWithImpl(
      _$Error_Open _value, $Res Function(_$Error_Open) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Error_Open(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$Error_Open implements Error_Open {
  const _$Error_Open(this.field0);

  @override
  final String field0;

  @override
  String toString() {
    return 'Error.open(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Error_Open &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Error_OpenCopyWith<_$Error_Open> get copyWith =>
      __$$Error_OpenCopyWithImpl<_$Error_Open>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) networkStream,
    required TResult Function(String field0) decode,
    required TResult Function(String field0) open,
    required TResult Function(String field0) preload,
    required TResult Function(String field0) playPreload,
  }) {
    return open(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? networkStream,
    TResult? Function(String field0)? decode,
    TResult? Function(String field0)? open,
    TResult? Function(String field0)? preload,
    TResult? Function(String field0)? playPreload,
  }) {
    return open?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? networkStream,
    TResult Function(String field0)? decode,
    TResult Function(String field0)? open,
    TResult Function(String field0)? preload,
    TResult Function(String field0)? playPreload,
    required TResult orElse(),
  }) {
    if (open != null) {
      return open(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Error_NetworkStream value) networkStream,
    required TResult Function(Error_Decode value) decode,
    required TResult Function(Error_Open value) open,
    required TResult Function(Error_Preload value) preload,
    required TResult Function(Error_PlayPreload value) playPreload,
  }) {
    return open(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Error_NetworkStream value)? networkStream,
    TResult? Function(Error_Decode value)? decode,
    TResult? Function(Error_Open value)? open,
    TResult? Function(Error_Preload value)? preload,
    TResult? Function(Error_PlayPreload value)? playPreload,
  }) {
    return open?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Error_NetworkStream value)? networkStream,
    TResult Function(Error_Decode value)? decode,
    TResult Function(Error_Open value)? open,
    TResult Function(Error_Preload value)? preload,
    TResult Function(Error_PlayPreload value)? playPreload,
    required TResult orElse(),
  }) {
    if (open != null) {
      return open(this);
    }
    return orElse();
  }
}

abstract class Error_Open implements Error {
  const factory Error_Open(final String field0) = _$Error_Open;

  @override
  String get field0;
  @override
  @JsonKey(ignore: true)
  _$$Error_OpenCopyWith<_$Error_Open> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Error_PreloadCopyWith<$Res> implements $ErrorCopyWith<$Res> {
  factory _$$Error_PreloadCopyWith(
          _$Error_Preload value, $Res Function(_$Error_Preload) then) =
      __$$Error_PreloadCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$Error_PreloadCopyWithImpl<$Res>
    extends _$ErrorCopyWithImpl<$Res, _$Error_Preload>
    implements _$$Error_PreloadCopyWith<$Res> {
  __$$Error_PreloadCopyWithImpl(
      _$Error_Preload _value, $Res Function(_$Error_Preload) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Error_Preload(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$Error_Preload implements Error_Preload {
  const _$Error_Preload(this.field0);

  @override
  final String field0;

  @override
  String toString() {
    return 'Error.preload(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Error_Preload &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Error_PreloadCopyWith<_$Error_Preload> get copyWith =>
      __$$Error_PreloadCopyWithImpl<_$Error_Preload>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) networkStream,
    required TResult Function(String field0) decode,
    required TResult Function(String field0) open,
    required TResult Function(String field0) preload,
    required TResult Function(String field0) playPreload,
  }) {
    return preload(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? networkStream,
    TResult? Function(String field0)? decode,
    TResult? Function(String field0)? open,
    TResult? Function(String field0)? preload,
    TResult? Function(String field0)? playPreload,
  }) {
    return preload?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? networkStream,
    TResult Function(String field0)? decode,
    TResult Function(String field0)? open,
    TResult Function(String field0)? preload,
    TResult Function(String field0)? playPreload,
    required TResult orElse(),
  }) {
    if (preload != null) {
      return preload(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Error_NetworkStream value) networkStream,
    required TResult Function(Error_Decode value) decode,
    required TResult Function(Error_Open value) open,
    required TResult Function(Error_Preload value) preload,
    required TResult Function(Error_PlayPreload value) playPreload,
  }) {
    return preload(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Error_NetworkStream value)? networkStream,
    TResult? Function(Error_Decode value)? decode,
    TResult? Function(Error_Open value)? open,
    TResult? Function(Error_Preload value)? preload,
    TResult? Function(Error_PlayPreload value)? playPreload,
  }) {
    return preload?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Error_NetworkStream value)? networkStream,
    TResult Function(Error_Decode value)? decode,
    TResult Function(Error_Open value)? open,
    TResult Function(Error_Preload value)? preload,
    TResult Function(Error_PlayPreload value)? playPreload,
    required TResult orElse(),
  }) {
    if (preload != null) {
      return preload(this);
    }
    return orElse();
  }
}

abstract class Error_Preload implements Error {
  const factory Error_Preload(final String field0) = _$Error_Preload;

  @override
  String get field0;
  @override
  @JsonKey(ignore: true)
  _$$Error_PreloadCopyWith<_$Error_Preload> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Error_PlayPreloadCopyWith<$Res>
    implements $ErrorCopyWith<$Res> {
  factory _$$Error_PlayPreloadCopyWith(
          _$Error_PlayPreload value, $Res Function(_$Error_PlayPreload) then) =
      __$$Error_PlayPreloadCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$Error_PlayPreloadCopyWithImpl<$Res>
    extends _$ErrorCopyWithImpl<$Res, _$Error_PlayPreload>
    implements _$$Error_PlayPreloadCopyWith<$Res> {
  __$$Error_PlayPreloadCopyWithImpl(
      _$Error_PlayPreload _value, $Res Function(_$Error_PlayPreload) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Error_PlayPreload(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$Error_PlayPreload implements Error_PlayPreload {
  const _$Error_PlayPreload(this.field0);

  @override
  final String field0;

  @override
  String toString() {
    return 'Error.playPreload(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Error_PlayPreload &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Error_PlayPreloadCopyWith<_$Error_PlayPreload> get copyWith =>
      __$$Error_PlayPreloadCopyWithImpl<_$Error_PlayPreload>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) networkStream,
    required TResult Function(String field0) decode,
    required TResult Function(String field0) open,
    required TResult Function(String field0) preload,
    required TResult Function(String field0) playPreload,
  }) {
    return playPreload(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? networkStream,
    TResult? Function(String field0)? decode,
    TResult? Function(String field0)? open,
    TResult? Function(String field0)? preload,
    TResult? Function(String field0)? playPreload,
  }) {
    return playPreload?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? networkStream,
    TResult Function(String field0)? decode,
    TResult Function(String field0)? open,
    TResult Function(String field0)? preload,
    TResult Function(String field0)? playPreload,
    required TResult orElse(),
  }) {
    if (playPreload != null) {
      return playPreload(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Error_NetworkStream value) networkStream,
    required TResult Function(Error_Decode value) decode,
    required TResult Function(Error_Open value) open,
    required TResult Function(Error_Preload value) preload,
    required TResult Function(Error_PlayPreload value) playPreload,
  }) {
    return playPreload(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Error_NetworkStream value)? networkStream,
    TResult? Function(Error_Decode value)? decode,
    TResult? Function(Error_Open value)? open,
    TResult? Function(Error_Preload value)? preload,
    TResult? Function(Error_PlayPreload value)? playPreload,
  }) {
    return playPreload?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Error_NetworkStream value)? networkStream,
    TResult Function(Error_Decode value)? decode,
    TResult Function(Error_Open value)? open,
    TResult Function(Error_Preload value)? preload,
    TResult Function(Error_PlayPreload value)? playPreload,
    required TResult orElse(),
  }) {
    if (playPreload != null) {
      return playPreload(this);
    }
    return orElse();
  }
}

abstract class Error_PlayPreload implements Error {
  const factory Error_PlayPreload(final String field0) = _$Error_PlayPreload;

  @override
  String get field0;
  @override
  @JsonKey(ignore: true)
  _$$Error_PlayPreloadCopyWith<_$Error_PlayPreload> get copyWith =>
      throw _privateConstructorUsedError;
}
