import 'package:equatable/equatable.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:frosy_pine/src/rust/api/brand_controller.dart';
import 'package:rust/rust.dart';

class BrandCubit extends Cubit<BrandCubitState> {
  BrandCubit({required this.brandController}) : super(const BrandCubitState(newBrandName: null));

  final BrandController brandController;

  void onNewBrandNameChange(String? value) {
    emit(state.copyWith(newBrandName: Some(value)));
  }

  Future<void> createBrand() async {}
}

final class BrandCubitState extends Equatable {
  const BrandCubitState({required this.newBrandName});

  final String? newBrandName;

  BrandCubitState copyWith({Option<String?>? newBrandName}) => BrandCubitState(newBrandName: newBrandName?.toNullable() ?? this.newBrandName);

  @override
  List<Object?> get props => [newBrandName];
}
