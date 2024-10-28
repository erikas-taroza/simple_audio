// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'api.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

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
abstract class _$$Error_NetworkStreamImplCopyWith<$Res>
    implements $ErrorCopyWith<$Res> {
  factory _$$Error_NetworkStreamImplCopyWith(_$Error_NetworkStreamImpl value,
          $Res Function(_$Error_NetworkStreamImpl) then) =
      __$$Error_NetworkStreamImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String field0});
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
    Object? field0 = null,
  }) {
    return _then(_$Error_NetworkStreamImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$Error_NetworkStreamImpl extends Error_NetworkStream {
  const _$Error_NetworkStreamImpl(this.field0) : super._();

  @override
  final String field0;

  @override
  String toString() {
    return 'Error.networkStream(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Error_NetworkStreamImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Error_NetworkStreamImplCopyWith<_$Error_NetworkStreamImpl> get copyWith =>
      __$$Error_NetworkStreamImplCopyWithImpl<_$Error_NetworkStreamImpl>(
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

abstract class Error_NetworkStream extends Error {
  const factory Error_NetworkStream(final String field0) =
      _$Error_NetworkStreamImpl;
  const Error_NetworkStream._() : super._();

  @override
  String get field0;
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
  $Res call({String field0});
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
    Object? field0 = null,
  }) {
    return _then(_$Error_DecodeImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$Error_DecodeImpl extends Error_Decode {
  const _$Error_DecodeImpl(this.field0) : super._();

  @override
  final String field0;

  @override
  String toString() {
    return 'Error.decode(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Error_DecodeImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Error_DecodeImplCopyWith<_$Error_DecodeImpl> get copyWith =>
      __$$Error_DecodeImplCopyWithImpl<_$Error_DecodeImpl>(this, _$identity);

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

abstract class Error_Decode extends Error {
  const factory Error_Decode(final String field0) = _$Error_DecodeImpl;
  const Error_Decode._() : super._();

  @override
  String get field0;
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
  $Res call({String field0});
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
    Object? field0 = null,
  }) {
    return _then(_$Error_OpenImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$Error_OpenImpl extends Error_Open {
  const _$Error_OpenImpl(this.field0) : super._();

  @override
  final String field0;

  @override
  String toString() {
    return 'Error.open(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Error_OpenImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Error_OpenImplCopyWith<_$Error_OpenImpl> get copyWith =>
      __$$Error_OpenImplCopyWithImpl<_$Error_OpenImpl>(this, _$identity);

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

abstract class Error_Open extends Error {
  const factory Error_Open(final String field0) = _$Error_OpenImpl;
  const Error_Open._() : super._();

  @override
  String get field0;
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
  $Res call({String field0});
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
    Object? field0 = null,
  }) {
    return _then(_$Error_PreloadImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$Error_PreloadImpl extends Error_Preload {
  const _$Error_PreloadImpl(this.field0) : super._();

  @override
  final String field0;

  @override
  String toString() {
    return 'Error.preload(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Error_PreloadImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Error_PreloadImplCopyWith<_$Error_PreloadImpl> get copyWith =>
      __$$Error_PreloadImplCopyWithImpl<_$Error_PreloadImpl>(this, _$identity);

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

abstract class Error_Preload extends Error {
  const factory Error_Preload(final String field0) = _$Error_PreloadImpl;
  const Error_Preload._() : super._();

  @override
  String get field0;
  @override
  @JsonKey(ignore: true)
  _$$Error_PreloadImplCopyWith<_$Error_PreloadImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
