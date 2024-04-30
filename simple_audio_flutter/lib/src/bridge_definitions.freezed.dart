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
