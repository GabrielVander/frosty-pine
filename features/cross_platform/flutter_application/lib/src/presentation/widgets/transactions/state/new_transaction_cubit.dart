import 'package:equatable/equatable.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

class NewTransactionCubit extends Cubit<NewTransactionCubitState> {
  NewTransactionCubit() : super(const NewTransactionCubitState(date: '', storeName: ''));

  void setDate(DateTime value) {
    emit(state.copyWith(date: value.toString()));
  }
}

@immutable
final class NewTransactionCubitState extends Equatable {
  const NewTransactionCubitState({required this.storeName, required this.date});

  final String storeName;
  final String date;

  @override
  List<Object?> get props => [storeName, date];

  NewTransactionCubitState copyWith({String? storeName, String? date}) =>
      NewTransactionCubitState(storeName: storeName ?? this.storeName, date: date ?? this.date);
}
