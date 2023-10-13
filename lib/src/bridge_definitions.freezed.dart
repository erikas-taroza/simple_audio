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
abstract class _$$Callback_ErrorImplCopyWith<$Res> {
  factory _$$Callback_ErrorImplCopyWith(_$Callback_ErrorImpl value,
          $Res Function(_$Callback_ErrorImpl) then) =
      __$$Callback_ErrorImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Error field0});

  $ErrorCopyWith<$Res> get field0;
}

/// @nodoc
class __$$Callback_ErrorImplCopyWithImpl<$Res>
    extends _$CallbackCopyWithImpl<$Res, _$Callback_ErrorImpl>
    implements _$$Callback_ErrorImplCopyWith<$Res> {
  __$$Callback_ErrorImplCopyWithImpl(
      _$Callback_ErrorImpl _value, $Res Function(_$Callback_ErrorImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Callback_ErrorImpl(
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

class _$Callback_ErrorImpl implements Callback_Error {
  const _$Callback_ErrorImpl(this.field0);

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
            other is _$Callback_ErrorImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Callback_ErrorImplCopyWith<_$Callback_ErrorImpl> get copyWith =>
      __$$Callback_ErrorImplCopyWithImpl<_$Callback_ErrorImpl>(
          this, _$identity);

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
  const factory Callback_Error(final Error field0) = _$Callback_ErrorImpl;

  Error get field0;
  @JsonKey(ignore: true)
  _$$Callback_ErrorImplCopyWith<_$Callback_ErrorImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Callback_MediaControlSkipPrevImplCopyWith<$Res> {
  factory _$$Callback_MediaControlSkipPrevImplCopyWith(
          _$Callback_MediaControlSkipPrevImpl value,
          $Res Function(_$Callback_MediaControlSkipPrevImpl) then) =
      __$$Callback_MediaControlSkipPrevImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Callback_MediaControlSkipPrevImplCopyWithImpl<$Res>
    extends _$CallbackCopyWithImpl<$Res, _$Callback_MediaControlSkipPrevImpl>
    implements _$$Callback_MediaControlSkipPrevImplCopyWith<$Res> {
  __$$Callback_MediaControlSkipPrevImplCopyWithImpl(
      _$Callback_MediaControlSkipPrevImpl _value,
      $Res Function(_$Callback_MediaControlSkipPrevImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$Callback_MediaControlSkipPrevImpl
    implements Callback_MediaControlSkipPrev {
  const _$Callback_MediaControlSkipPrevImpl();

  @override
  String toString() {
    return 'Callback.mediaControlSkipPrev()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Callback_MediaControlSkipPrevImpl);
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
      _$Callback_MediaControlSkipPrevImpl;
}

/// @nodoc
abstract class _$$Callback_MediaControlSkipNextImplCopyWith<$Res> {
  factory _$$Callback_MediaControlSkipNextImplCopyWith(
          _$Callback_MediaControlSkipNextImpl value,
          $Res Function(_$Callback_MediaControlSkipNextImpl) then) =
      __$$Callback_MediaControlSkipNextImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Callback_MediaControlSkipNextImplCopyWithImpl<$Res>
    extends _$CallbackCopyWithImpl<$Res, _$Callback_MediaControlSkipNextImpl>
    implements _$$Callback_MediaControlSkipNextImplCopyWith<$Res> {
  __$$Callback_MediaControlSkipNextImplCopyWithImpl(
      _$Callback_MediaControlSkipNextImpl _value,
      $Res Function(_$Callback_MediaControlSkipNextImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$Callback_MediaControlSkipNextImpl
    implements Callback_MediaControlSkipNext {
  const _$Callback_MediaControlSkipNextImpl();

  @override
  String toString() {
    return 'Callback.mediaControlSkipNext()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Callback_MediaControlSkipNextImpl);
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
      _$Callback_MediaControlSkipNextImpl;
}

/// @nodoc
abstract class _$$Callback_PlaybackLoopedImplCopyWith<$Res> {
  factory _$$Callback_PlaybackLoopedImplCopyWith(
          _$Callback_PlaybackLoopedImpl value,
          $Res Function(_$Callback_PlaybackLoopedImpl) then) =
      __$$Callback_PlaybackLoopedImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Callback_PlaybackLoopedImplCopyWithImpl<$Res>
    extends _$CallbackCopyWithImpl<$Res, _$Callback_PlaybackLoopedImpl>
    implements _$$Callback_PlaybackLoopedImplCopyWith<$Res> {
  __$$Callback_PlaybackLoopedImplCopyWithImpl(
      _$Callback_PlaybackLoopedImpl _value,
      $Res Function(_$Callback_PlaybackLoopedImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$Callback_PlaybackLoopedImpl implements Callback_PlaybackLooped {
  const _$Callback_PlaybackLoopedImpl();

  @override
  String toString() {
    return 'Callback.playbackLooped()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Callback_PlaybackLoopedImpl);
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
  const factory Callback_PlaybackLooped() = _$Callback_PlaybackLoopedImpl;
}

/// @nodoc
abstract class _$$Callback_DurationCalculatedImplCopyWith<$Res> {
  factory _$$Callback_DurationCalculatedImplCopyWith(
          _$Callback_DurationCalculatedImpl value,
          $Res Function(_$Callback_DurationCalculatedImpl) then) =
      __$$Callback_DurationCalculatedImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Callback_DurationCalculatedImplCopyWithImpl<$Res>
    extends _$CallbackCopyWithImpl<$Res, _$Callback_DurationCalculatedImpl>
    implements _$$Callback_DurationCalculatedImplCopyWith<$Res> {
  __$$Callback_DurationCalculatedImplCopyWithImpl(
      _$Callback_DurationCalculatedImpl _value,
      $Res Function(_$Callback_DurationCalculatedImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$Callback_DurationCalculatedImpl implements Callback_DurationCalculated {
  const _$Callback_DurationCalculatedImpl();

  @override
  String toString() {
    return 'Callback.durationCalculated()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Callback_DurationCalculatedImpl);
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
  const factory Callback_DurationCalculated() =
      _$Callback_DurationCalculatedImpl;
}

/// @nodoc
mixin _$Error {
  /// The error message.
  String get message => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) networkStream,
    required TResult Function(String message) decode,
    required TResult Function(String message) open,
    required TResult Function(String message) preload,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? networkStream,
    TResult? Function(String message)? decode,
    TResult? Function(String message)? open,
    TResult? Function(String message)? preload,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? networkStream,
    TResult Function(String message)? decode,
    TResult Function(String message)? open,
    TResult Function(String message)? preload,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Error_NetworkStream value) networkStream,
    required TResult Function(Error_Decode value) decode,
    required TResult Function(Error_Open value) open,
    required TResult Function(Error_Preload value) preload,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Error_NetworkStream value)? networkStream,
    TResult? Function(Error_Decode value)? decode,
    TResult? Function(Error_Open value)? open,
    TResult? Function(Error_Preload value)? preload,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Error_NetworkStream value)? networkStream,
    TResult Function(Error_Decode value)? decode,
    TResult Function(Error_Open value)? open,
    TResult Function(Error_Preload value)? preload,
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
  $Res call({String message});
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
    Object? message = null,
  }) {
    return _then(_value.copyWith(
      message: null == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$Error_NetworkStreamImplCopyWith<$Res>
    implements $ErrorCopyWith<$Res> {
  factory _$$Error_NetworkStreamImplCopyWith(_$Error_NetworkStreamImpl value,
          $Res Function(_$Error_NetworkStreamImpl) then) =
      __$$Error_NetworkStreamImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String message});
}

/// @nodoc
class __$$Error_NetworkStreamImplCopyWithImpl<$Res>
    extends _$ErrorCopyWithImpl<$Res, _$Error_NetworkStreamImpl>
    implements _$$Error_NetworkStreamImplCopyWith<$Res> {
  __$$Error_NetworkStreamImplCopyWithImpl(_$Error_NetworkStreamImpl _value,
      $Res Function(_$Error_NetworkStreamImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? message = null,
  }) {
    return _then(_$Error_NetworkStreamImpl(
      message: null == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$Error_NetworkStreamImpl implements Error_NetworkStream {
  const _$Error_NetworkStreamImpl({required this.message});

  /// The error message.
  @override
  final String message;

  @override
  String toString() {
    return 'Error.networkStream(message: $message)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Error_NetworkStreamImpl &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Error_NetworkStreamImplCopyWith<_$Error_NetworkStreamImpl> get copyWith =>
      __$$Error_NetworkStreamImplCopyWithImpl<_$Error_NetworkStreamImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) networkStream,
    required TResult Function(String message) decode,
    required TResult Function(String message) open,
    required TResult Function(String message) preload,
  }) {
    return networkStream(message);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? networkStream,
    TResult? Function(String message)? decode,
    TResult? Function(String message)? open,
    TResult? Function(String message)? preload,
  }) {
    return networkStream?.call(message);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? networkStream,
    TResult Function(String message)? decode,
    TResult Function(String message)? open,
    TResult Function(String message)? preload,
    required TResult orElse(),
  }) {
    if (networkStream != null) {
      return networkStream(message);
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
    required TResult orElse(),
  }) {
    if (networkStream != null) {
      return networkStream(this);
    }
    return orElse();
  }
}

abstract class Error_NetworkStream implements Error {
  const factory Error_NetworkStream({required final String message}) =
      _$Error_NetworkStreamImpl;

  @override

  /// The error message.
  String get message;
  @override
  @JsonKey(ignore: true)
  _$$Error_NetworkStreamImplCopyWith<_$Error_NetworkStreamImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Error_DecodeImplCopyWith<$Res>
    implements $ErrorCopyWith<$Res> {
  factory _$$Error_DecodeImplCopyWith(
          _$Error_DecodeImpl value, $Res Function(_$Error_DecodeImpl) then) =
      __$$Error_DecodeImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String message});
}

/// @nodoc
class __$$Error_DecodeImplCopyWithImpl<$Res>
    extends _$ErrorCopyWithImpl<$Res, _$Error_DecodeImpl>
    implements _$$Error_DecodeImplCopyWith<$Res> {
  __$$Error_DecodeImplCopyWithImpl(
      _$Error_DecodeImpl _value, $Res Function(_$Error_DecodeImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? message = null,
  }) {
    return _then(_$Error_DecodeImpl(
      message: null == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$Error_DecodeImpl implements Error_Decode {
  const _$Error_DecodeImpl({required this.message});

  /// The error message.
  @override
  final String message;

  @override
  String toString() {
    return 'Error.decode(message: $message)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Error_DecodeImpl &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Error_DecodeImplCopyWith<_$Error_DecodeImpl> get copyWith =>
      __$$Error_DecodeImplCopyWithImpl<_$Error_DecodeImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) networkStream,
    required TResult Function(String message) decode,
    required TResult Function(String message) open,
    required TResult Function(String message) preload,
  }) {
    return decode(message);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? networkStream,
    TResult? Function(String message)? decode,
    TResult? Function(String message)? open,
    TResult? Function(String message)? preload,
  }) {
    return decode?.call(message);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? networkStream,
    TResult Function(String message)? decode,
    TResult Function(String message)? open,
    TResult Function(String message)? preload,
    required TResult orElse(),
  }) {
    if (decode != null) {
      return decode(message);
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
    required TResult orElse(),
  }) {
    if (decode != null) {
      return decode(this);
    }
    return orElse();
  }
}

abstract class Error_Decode implements Error {
  const factory Error_Decode({required final String message}) =
      _$Error_DecodeImpl;

  @override

  /// The error message.
  String get message;
  @override
  @JsonKey(ignore: true)
  _$$Error_DecodeImplCopyWith<_$Error_DecodeImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Error_OpenImplCopyWith<$Res> implements $ErrorCopyWith<$Res> {
  factory _$$Error_OpenImplCopyWith(
          _$Error_OpenImpl value, $Res Function(_$Error_OpenImpl) then) =
      __$$Error_OpenImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String message});
}

/// @nodoc
class __$$Error_OpenImplCopyWithImpl<$Res>
    extends _$ErrorCopyWithImpl<$Res, _$Error_OpenImpl>
    implements _$$Error_OpenImplCopyWith<$Res> {
  __$$Error_OpenImplCopyWithImpl(
      _$Error_OpenImpl _value, $Res Function(_$Error_OpenImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? message = null,
  }) {
    return _then(_$Error_OpenImpl(
      message: null == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$Error_OpenImpl implements Error_Open {
  const _$Error_OpenImpl({required this.message});

  /// The error message.
  @override
  final String message;

  @override
  String toString() {
    return 'Error.open(message: $message)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Error_OpenImpl &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Error_OpenImplCopyWith<_$Error_OpenImpl> get copyWith =>
      __$$Error_OpenImplCopyWithImpl<_$Error_OpenImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) networkStream,
    required TResult Function(String message) decode,
    required TResult Function(String message) open,
    required TResult Function(String message) preload,
  }) {
    return open(message);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? networkStream,
    TResult? Function(String message)? decode,
    TResult? Function(String message)? open,
    TResult? Function(String message)? preload,
  }) {
    return open?.call(message);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? networkStream,
    TResult Function(String message)? decode,
    TResult Function(String message)? open,
    TResult Function(String message)? preload,
    required TResult orElse(),
  }) {
    if (open != null) {
      return open(message);
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
    required TResult orElse(),
  }) {
    if (open != null) {
      return open(this);
    }
    return orElse();
  }
}

abstract class Error_Open implements Error {
  const factory Error_Open({required final String message}) = _$Error_OpenImpl;

  @override

  /// The error message.
  String get message;
  @override
  @JsonKey(ignore: true)
  _$$Error_OpenImplCopyWith<_$Error_OpenImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Error_PreloadImplCopyWith<$Res>
    implements $ErrorCopyWith<$Res> {
  factory _$$Error_PreloadImplCopyWith(
          _$Error_PreloadImpl value, $Res Function(_$Error_PreloadImpl) then) =
      __$$Error_PreloadImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String message});
}

/// @nodoc
class __$$Error_PreloadImplCopyWithImpl<$Res>
    extends _$ErrorCopyWithImpl<$Res, _$Error_PreloadImpl>
    implements _$$Error_PreloadImplCopyWith<$Res> {
  __$$Error_PreloadImplCopyWithImpl(
      _$Error_PreloadImpl _value, $Res Function(_$Error_PreloadImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? message = null,
  }) {
    return _then(_$Error_PreloadImpl(
      message: null == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$Error_PreloadImpl implements Error_Preload {
  const _$Error_PreloadImpl({required this.message});

  /// The error message.
  @override
  final String message;

  @override
  String toString() {
    return 'Error.preload(message: $message)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Error_PreloadImpl &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Error_PreloadImplCopyWith<_$Error_PreloadImpl> get copyWith =>
      __$$Error_PreloadImplCopyWithImpl<_$Error_PreloadImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) networkStream,
    required TResult Function(String message) decode,
    required TResult Function(String message) open,
    required TResult Function(String message) preload,
  }) {
    return preload(message);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? networkStream,
    TResult? Function(String message)? decode,
    TResult? Function(String message)? open,
    TResult? Function(String message)? preload,
  }) {
    return preload?.call(message);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? networkStream,
    TResult Function(String message)? decode,
    TResult Function(String message)? open,
    TResult Function(String message)? preload,
    required TResult orElse(),
  }) {
    if (preload != null) {
      return preload(message);
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
    required TResult orElse(),
  }) {
    if (preload != null) {
      return preload(this);
    }
    return orElse();
  }
}

abstract class Error_Preload implements Error {
  const factory Error_Preload({required final String message}) =
      _$Error_PreloadImpl;

  @override

  /// The error message.
  String get message;
  @override
  @JsonKey(ignore: true)
  _$$Error_PreloadImplCopyWith<_$Error_PreloadImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
