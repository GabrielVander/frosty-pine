import 'package:equatable/equatable.dart';
import 'package:flutter/material.dart';

@immutable
abstract class Failure extends Equatable {
  const Failure({required this.message, required this.details});

  final String? message;
  final String? details;

  @override
  List<Object?> get props => [message, details];
}

@immutable
abstract class DisplayableFailure extends Failure {
  const DisplayableFailure({required super.message, required super.details, required this.i18nKey});

  final String? i18nKey;

  @override
  List<Object?> get props => [message, details, i18nKey];
}
