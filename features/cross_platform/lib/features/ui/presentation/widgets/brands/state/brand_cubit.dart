import 'package:equatable/equatable.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:rust/rust.dart';

class BrandCubit extends Cubit<BrandCubitState> {
  BrandCubit() : super(const BrandCubitState(newBrandName: null));

  void onNewBrandNameChange(String? value) {}
}

final class BrandCubitState extends Equatable {
  const BrandCubitState({required this.newBrandName});

  final String? newBrandName;

  BrandCubitState copyWith({Option<String?>? newBrandName}) => BrandCubitState(newBrandName: newBrandName?.toNullable() ?? this.newBrandName);

  @override
  List<Object?> get props => [newBrandName];
}
