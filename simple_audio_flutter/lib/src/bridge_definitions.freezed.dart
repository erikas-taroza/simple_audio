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
mixin _$Error {
  String get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) networkStream,
    required TResult Function(String field0) decode,
    required TResult Function(String field0) open,
    required TResult Function(String field0) preload,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? networkStream,
    TResult? Function(String field0)? decode,
    TResult? Function(String field0)? open,
    TResult? Function(String field0)? preload,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? networkStream,
    TResult Function(String field0)? decode,
    TResult Function(String field0)? open,
    TResult Function(String field0)? preload,
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
  const factory Error_Preload(final String field0) = _$Error_Preload;

  @override
  String get field0;
  @override
  @JsonKey(ignore: true)
  _$$Error_PreloadCopyWith<_$Error_Preload> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$PlaybackState {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Duration field0) started,
    required TResult Function() play,
    required TResult Function() pause,
    required TResult Function() done,
    required TResult Function() stop,
    required TResult Function() preloadPlayed,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Duration field0)? started,
    TResult? Function()? play,
    TResult? Function()? pause,
    TResult? Function()? done,
    TResult? Function()? stop,
    TResult? Function()? preloadPlayed,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Duration field0)? started,
    TResult Function()? play,
    TResult Function()? pause,
    TResult Function()? done,
    TResult Function()? stop,
    TResult Function()? preloadPlayed,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PlaybackState_Started value) started,
    required TResult Function(PlaybackState_Play value) play,
    required TResult Function(PlaybackState_Pause value) pause,
    required TResult Function(PlaybackState_Done value) done,
    required TResult Function(PlaybackState_Stop value) stop,
    required TResult Function(PlaybackState_PreloadPlayed value) preloadPlayed,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PlaybackState_Started value)? started,
    TResult? Function(PlaybackState_Play value)? play,
    TResult? Function(PlaybackState_Pause value)? pause,
    TResult? Function(PlaybackState_Done value)? done,
    TResult? Function(PlaybackState_Stop value)? stop,
    TResult? Function(PlaybackState_PreloadPlayed value)? preloadPlayed,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PlaybackState_Started value)? started,
    TResult Function(PlaybackState_Play value)? play,
    TResult Function(PlaybackState_Pause value)? pause,
    TResult Function(PlaybackState_Done value)? done,
    TResult Function(PlaybackState_Stop value)? stop,
    TResult Function(PlaybackState_PreloadPlayed value)? preloadPlayed,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $PlaybackStateCopyWith<$Res> {
  factory $PlaybackStateCopyWith(
          PlaybackState value, $Res Function(PlaybackState) then) =
      _$PlaybackStateCopyWithImpl<$Res, PlaybackState>;
}

/// @nodoc
class _$PlaybackStateCopyWithImpl<$Res, $Val extends PlaybackState>
    implements $PlaybackStateCopyWith<$Res> {
  _$PlaybackStateCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$PlaybackState_StartedCopyWith<$Res> {
  factory _$$PlaybackState_StartedCopyWith(_$PlaybackState_Started value,
          $Res Function(_$PlaybackState_Started) then) =
      __$$PlaybackState_StartedCopyWithImpl<$Res>;
  @useResult
  $Res call({Duration field0});
}

/// @nodoc
class __$$PlaybackState_StartedCopyWithImpl<$Res>
    extends _$PlaybackStateCopyWithImpl<$Res, _$PlaybackState_Started>
    implements _$$PlaybackState_StartedCopyWith<$Res> {
  __$$PlaybackState_StartedCopyWithImpl(_$PlaybackState_Started _value,
      $Res Function(_$PlaybackState_Started) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$PlaybackState_Started(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Duration,
    ));
  }
}

/// @nodoc

class _$PlaybackState_Started implements PlaybackState_Started {
  const _$PlaybackState_Started(this.field0);

  @override
  final Duration field0;

  @override
  String toString() {
    return 'PlaybackState.started(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PlaybackState_Started &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$PlaybackState_StartedCopyWith<_$PlaybackState_Started> get copyWith =>
      __$$PlaybackState_StartedCopyWithImpl<_$PlaybackState_Started>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Duration field0) started,
    required TResult Function() play,
    required TResult Function() pause,
    required TResult Function() done,
    required TResult Function() stop,
    required TResult Function() preloadPlayed,
  }) {
    return started(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Duration field0)? started,
    TResult? Function()? play,
    TResult? Function()? pause,
    TResult? Function()? done,
    TResult? Function()? stop,
    TResult? Function()? preloadPlayed,
  }) {
    return started?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Duration field0)? started,
    TResult Function()? play,
    TResult Function()? pause,
    TResult Function()? done,
    TResult Function()? stop,
    TResult Function()? preloadPlayed,
    required TResult orElse(),
  }) {
    if (started != null) {
      return started(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PlaybackState_Started value) started,
    required TResult Function(PlaybackState_Play value) play,
    required TResult Function(PlaybackState_Pause value) pause,
    required TResult Function(PlaybackState_Done value) done,
    required TResult Function(PlaybackState_Stop value) stop,
    required TResult Function(PlaybackState_PreloadPlayed value) preloadPlayed,
  }) {
    return started(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PlaybackState_Started value)? started,
    TResult? Function(PlaybackState_Play value)? play,
    TResult? Function(PlaybackState_Pause value)? pause,
    TResult? Function(PlaybackState_Done value)? done,
    TResult? Function(PlaybackState_Stop value)? stop,
    TResult? Function(PlaybackState_PreloadPlayed value)? preloadPlayed,
  }) {
    return started?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PlaybackState_Started value)? started,
    TResult Function(PlaybackState_Play value)? play,
    TResult Function(PlaybackState_Pause value)? pause,
    TResult Function(PlaybackState_Done value)? done,
    TResult Function(PlaybackState_Stop value)? stop,
    TResult Function(PlaybackState_PreloadPlayed value)? preloadPlayed,
    required TResult orElse(),
  }) {
    if (started != null) {
      return started(this);
    }
    return orElse();
  }
}

abstract class PlaybackState_Started implements PlaybackState {
  const factory PlaybackState_Started(final Duration field0) =
      _$PlaybackState_Started;

  Duration get field0;
  @JsonKey(ignore: true)
  _$$PlaybackState_StartedCopyWith<_$PlaybackState_Started> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PlaybackState_PlayCopyWith<$Res> {
  factory _$$PlaybackState_PlayCopyWith(_$PlaybackState_Play value,
          $Res Function(_$PlaybackState_Play) then) =
      __$$PlaybackState_PlayCopyWithImpl<$Res>;
}

/// @nodoc
class __$$PlaybackState_PlayCopyWithImpl<$Res>
    extends _$PlaybackStateCopyWithImpl<$Res, _$PlaybackState_Play>
    implements _$$PlaybackState_PlayCopyWith<$Res> {
  __$$PlaybackState_PlayCopyWithImpl(
      _$PlaybackState_Play _value, $Res Function(_$PlaybackState_Play) _then)
      : super(_value, _then);
}

/// @nodoc

class _$PlaybackState_Play implements PlaybackState_Play {
  const _$PlaybackState_Play();

  @override
  String toString() {
    return 'PlaybackState.play()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$PlaybackState_Play);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Duration field0) started,
    required TResult Function() play,
    required TResult Function() pause,
    required TResult Function() done,
    required TResult Function() stop,
    required TResult Function() preloadPlayed,
  }) {
    return play();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Duration field0)? started,
    TResult? Function()? play,
    TResult? Function()? pause,
    TResult? Function()? done,
    TResult? Function()? stop,
    TResult? Function()? preloadPlayed,
  }) {
    return play?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Duration field0)? started,
    TResult Function()? play,
    TResult Function()? pause,
    TResult Function()? done,
    TResult Function()? stop,
    TResult Function()? preloadPlayed,
    required TResult orElse(),
  }) {
    if (play != null) {
      return play();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PlaybackState_Started value) started,
    required TResult Function(PlaybackState_Play value) play,
    required TResult Function(PlaybackState_Pause value) pause,
    required TResult Function(PlaybackState_Done value) done,
    required TResult Function(PlaybackState_Stop value) stop,
    required TResult Function(PlaybackState_PreloadPlayed value) preloadPlayed,
  }) {
    return play(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PlaybackState_Started value)? started,
    TResult? Function(PlaybackState_Play value)? play,
    TResult? Function(PlaybackState_Pause value)? pause,
    TResult? Function(PlaybackState_Done value)? done,
    TResult? Function(PlaybackState_Stop value)? stop,
    TResult? Function(PlaybackState_PreloadPlayed value)? preloadPlayed,
  }) {
    return play?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PlaybackState_Started value)? started,
    TResult Function(PlaybackState_Play value)? play,
    TResult Function(PlaybackState_Pause value)? pause,
    TResult Function(PlaybackState_Done value)? done,
    TResult Function(PlaybackState_Stop value)? stop,
    TResult Function(PlaybackState_PreloadPlayed value)? preloadPlayed,
    required TResult orElse(),
  }) {
    if (play != null) {
      return play(this);
    }
    return orElse();
  }
}

abstract class PlaybackState_Play implements PlaybackState {
  const factory PlaybackState_Play() = _$PlaybackState_Play;
}

/// @nodoc
abstract class _$$PlaybackState_PauseCopyWith<$Res> {
  factory _$$PlaybackState_PauseCopyWith(_$PlaybackState_Pause value,
          $Res Function(_$PlaybackState_Pause) then) =
      __$$PlaybackState_PauseCopyWithImpl<$Res>;
}

/// @nodoc
class __$$PlaybackState_PauseCopyWithImpl<$Res>
    extends _$PlaybackStateCopyWithImpl<$Res, _$PlaybackState_Pause>
    implements _$$PlaybackState_PauseCopyWith<$Res> {
  __$$PlaybackState_PauseCopyWithImpl(
      _$PlaybackState_Pause _value, $Res Function(_$PlaybackState_Pause) _then)
      : super(_value, _then);
}

/// @nodoc

class _$PlaybackState_Pause implements PlaybackState_Pause {
  const _$PlaybackState_Pause();

  @override
  String toString() {
    return 'PlaybackState.pause()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$PlaybackState_Pause);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Duration field0) started,
    required TResult Function() play,
    required TResult Function() pause,
    required TResult Function() done,
    required TResult Function() stop,
    required TResult Function() preloadPlayed,
  }) {
    return pause();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Duration field0)? started,
    TResult? Function()? play,
    TResult? Function()? pause,
    TResult? Function()? done,
    TResult? Function()? stop,
    TResult? Function()? preloadPlayed,
  }) {
    return pause?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Duration field0)? started,
    TResult Function()? play,
    TResult Function()? pause,
    TResult Function()? done,
    TResult Function()? stop,
    TResult Function()? preloadPlayed,
    required TResult orElse(),
  }) {
    if (pause != null) {
      return pause();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PlaybackState_Started value) started,
    required TResult Function(PlaybackState_Play value) play,
    required TResult Function(PlaybackState_Pause value) pause,
    required TResult Function(PlaybackState_Done value) done,
    required TResult Function(PlaybackState_Stop value) stop,
    required TResult Function(PlaybackState_PreloadPlayed value) preloadPlayed,
  }) {
    return pause(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PlaybackState_Started value)? started,
    TResult? Function(PlaybackState_Play value)? play,
    TResult? Function(PlaybackState_Pause value)? pause,
    TResult? Function(PlaybackState_Done value)? done,
    TResult? Function(PlaybackState_Stop value)? stop,
    TResult? Function(PlaybackState_PreloadPlayed value)? preloadPlayed,
  }) {
    return pause?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PlaybackState_Started value)? started,
    TResult Function(PlaybackState_Play value)? play,
    TResult Function(PlaybackState_Pause value)? pause,
    TResult Function(PlaybackState_Done value)? done,
    TResult Function(PlaybackState_Stop value)? stop,
    TResult Function(PlaybackState_PreloadPlayed value)? preloadPlayed,
    required TResult orElse(),
  }) {
    if (pause != null) {
      return pause(this);
    }
    return orElse();
  }
}

abstract class PlaybackState_Pause implements PlaybackState {
  const factory PlaybackState_Pause() = _$PlaybackState_Pause;
}

/// @nodoc
abstract class _$$PlaybackState_DoneCopyWith<$Res> {
  factory _$$PlaybackState_DoneCopyWith(_$PlaybackState_Done value,
          $Res Function(_$PlaybackState_Done) then) =
      __$$PlaybackState_DoneCopyWithImpl<$Res>;
}

/// @nodoc
class __$$PlaybackState_DoneCopyWithImpl<$Res>
    extends _$PlaybackStateCopyWithImpl<$Res, _$PlaybackState_Done>
    implements _$$PlaybackState_DoneCopyWith<$Res> {
  __$$PlaybackState_DoneCopyWithImpl(
      _$PlaybackState_Done _value, $Res Function(_$PlaybackState_Done) _then)
      : super(_value, _then);
}

/// @nodoc

class _$PlaybackState_Done implements PlaybackState_Done {
  const _$PlaybackState_Done();

  @override
  String toString() {
    return 'PlaybackState.done()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$PlaybackState_Done);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Duration field0) started,
    required TResult Function() play,
    required TResult Function() pause,
    required TResult Function() done,
    required TResult Function() stop,
    required TResult Function() preloadPlayed,
  }) {
    return done();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Duration field0)? started,
    TResult? Function()? play,
    TResult? Function()? pause,
    TResult? Function()? done,
    TResult? Function()? stop,
    TResult? Function()? preloadPlayed,
  }) {
    return done?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Duration field0)? started,
    TResult Function()? play,
    TResult Function()? pause,
    TResult Function()? done,
    TResult Function()? stop,
    TResult Function()? preloadPlayed,
    required TResult orElse(),
  }) {
    if (done != null) {
      return done();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PlaybackState_Started value) started,
    required TResult Function(PlaybackState_Play value) play,
    required TResult Function(PlaybackState_Pause value) pause,
    required TResult Function(PlaybackState_Done value) done,
    required TResult Function(PlaybackState_Stop value) stop,
    required TResult Function(PlaybackState_PreloadPlayed value) preloadPlayed,
  }) {
    return done(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PlaybackState_Started value)? started,
    TResult? Function(PlaybackState_Play value)? play,
    TResult? Function(PlaybackState_Pause value)? pause,
    TResult? Function(PlaybackState_Done value)? done,
    TResult? Function(PlaybackState_Stop value)? stop,
    TResult? Function(PlaybackState_PreloadPlayed value)? preloadPlayed,
  }) {
    return done?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PlaybackState_Started value)? started,
    TResult Function(PlaybackState_Play value)? play,
    TResult Function(PlaybackState_Pause value)? pause,
    TResult Function(PlaybackState_Done value)? done,
    TResult Function(PlaybackState_Stop value)? stop,
    TResult Function(PlaybackState_PreloadPlayed value)? preloadPlayed,
    required TResult orElse(),
  }) {
    if (done != null) {
      return done(this);
    }
    return orElse();
  }
}

abstract class PlaybackState_Done implements PlaybackState {
  const factory PlaybackState_Done() = _$PlaybackState_Done;
}

/// @nodoc
abstract class _$$PlaybackState_StopCopyWith<$Res> {
  factory _$$PlaybackState_StopCopyWith(_$PlaybackState_Stop value,
          $Res Function(_$PlaybackState_Stop) then) =
      __$$PlaybackState_StopCopyWithImpl<$Res>;
}

/// @nodoc
class __$$PlaybackState_StopCopyWithImpl<$Res>
    extends _$PlaybackStateCopyWithImpl<$Res, _$PlaybackState_Stop>
    implements _$$PlaybackState_StopCopyWith<$Res> {
  __$$PlaybackState_StopCopyWithImpl(
      _$PlaybackState_Stop _value, $Res Function(_$PlaybackState_Stop) _then)
      : super(_value, _then);
}

/// @nodoc

class _$PlaybackState_Stop implements PlaybackState_Stop {
  const _$PlaybackState_Stop();

  @override
  String toString() {
    return 'PlaybackState.stop()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$PlaybackState_Stop);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Duration field0) started,
    required TResult Function() play,
    required TResult Function() pause,
    required TResult Function() done,
    required TResult Function() stop,
    required TResult Function() preloadPlayed,
  }) {
    return stop();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Duration field0)? started,
    TResult? Function()? play,
    TResult? Function()? pause,
    TResult? Function()? done,
    TResult? Function()? stop,
    TResult? Function()? preloadPlayed,
  }) {
    return stop?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Duration field0)? started,
    TResult Function()? play,
    TResult Function()? pause,
    TResult Function()? done,
    TResult Function()? stop,
    TResult Function()? preloadPlayed,
    required TResult orElse(),
  }) {
    if (stop != null) {
      return stop();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PlaybackState_Started value) started,
    required TResult Function(PlaybackState_Play value) play,
    required TResult Function(PlaybackState_Pause value) pause,
    required TResult Function(PlaybackState_Done value) done,
    required TResult Function(PlaybackState_Stop value) stop,
    required TResult Function(PlaybackState_PreloadPlayed value) preloadPlayed,
  }) {
    return stop(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PlaybackState_Started value)? started,
    TResult? Function(PlaybackState_Play value)? play,
    TResult? Function(PlaybackState_Pause value)? pause,
    TResult? Function(PlaybackState_Done value)? done,
    TResult? Function(PlaybackState_Stop value)? stop,
    TResult? Function(PlaybackState_PreloadPlayed value)? preloadPlayed,
  }) {
    return stop?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PlaybackState_Started value)? started,
    TResult Function(PlaybackState_Play value)? play,
    TResult Function(PlaybackState_Pause value)? pause,
    TResult Function(PlaybackState_Done value)? done,
    TResult Function(PlaybackState_Stop value)? stop,
    TResult Function(PlaybackState_PreloadPlayed value)? preloadPlayed,
    required TResult orElse(),
  }) {
    if (stop != null) {
      return stop(this);
    }
    return orElse();
  }
}

abstract class PlaybackState_Stop implements PlaybackState {
  const factory PlaybackState_Stop() = _$PlaybackState_Stop;
}

/// @nodoc
abstract class _$$PlaybackState_PreloadPlayedCopyWith<$Res> {
  factory _$$PlaybackState_PreloadPlayedCopyWith(
          _$PlaybackState_PreloadPlayed value,
          $Res Function(_$PlaybackState_PreloadPlayed) then) =
      __$$PlaybackState_PreloadPlayedCopyWithImpl<$Res>;
}

/// @nodoc
class __$$PlaybackState_PreloadPlayedCopyWithImpl<$Res>
    extends _$PlaybackStateCopyWithImpl<$Res, _$PlaybackState_PreloadPlayed>
    implements _$$PlaybackState_PreloadPlayedCopyWith<$Res> {
  __$$PlaybackState_PreloadPlayedCopyWithImpl(
      _$PlaybackState_PreloadPlayed _value,
      $Res Function(_$PlaybackState_PreloadPlayed) _then)
      : super(_value, _then);
}

/// @nodoc

class _$PlaybackState_PreloadPlayed implements PlaybackState_PreloadPlayed {
  const _$PlaybackState_PreloadPlayed();

  @override
  String toString() {
    return 'PlaybackState.preloadPlayed()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PlaybackState_PreloadPlayed);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Duration field0) started,
    required TResult Function() play,
    required TResult Function() pause,
    required TResult Function() done,
    required TResult Function() stop,
    required TResult Function() preloadPlayed,
  }) {
    return preloadPlayed();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Duration field0)? started,
    TResult? Function()? play,
    TResult? Function()? pause,
    TResult? Function()? done,
    TResult? Function()? stop,
    TResult? Function()? preloadPlayed,
  }) {
    return preloadPlayed?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Duration field0)? started,
    TResult Function()? play,
    TResult Function()? pause,
    TResult Function()? done,
    TResult Function()? stop,
    TResult Function()? preloadPlayed,
    required TResult orElse(),
  }) {
    if (preloadPlayed != null) {
      return preloadPlayed();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PlaybackState_Started value) started,
    required TResult Function(PlaybackState_Play value) play,
    required TResult Function(PlaybackState_Pause value) pause,
    required TResult Function(PlaybackState_Done value) done,
    required TResult Function(PlaybackState_Stop value) stop,
    required TResult Function(PlaybackState_PreloadPlayed value) preloadPlayed,
  }) {
    return preloadPlayed(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PlaybackState_Started value)? started,
    TResult? Function(PlaybackState_Play value)? play,
    TResult? Function(PlaybackState_Pause value)? pause,
    TResult? Function(PlaybackState_Done value)? done,
    TResult? Function(PlaybackState_Stop value)? stop,
    TResult? Function(PlaybackState_PreloadPlayed value)? preloadPlayed,
  }) {
    return preloadPlayed?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PlaybackState_Started value)? started,
    TResult Function(PlaybackState_Play value)? play,
    TResult Function(PlaybackState_Pause value)? pause,
    TResult Function(PlaybackState_Done value)? done,
    TResult Function(PlaybackState_Stop value)? stop,
    TResult Function(PlaybackState_PreloadPlayed value)? preloadPlayed,
    required TResult orElse(),
  }) {
    if (preloadPlayed != null) {
      return preloadPlayed(this);
    }
    return orElse();
  }
}

abstract class PlaybackState_PreloadPlayed implements PlaybackState {
  const factory PlaybackState_PreloadPlayed() = _$PlaybackState_PreloadPlayed;
}
