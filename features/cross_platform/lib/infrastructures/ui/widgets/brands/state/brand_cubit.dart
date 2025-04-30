import 'package:equatable/equatable.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:frosty_pine/adapters/presenters/flutter_presenter.dart';
import 'package:rust/rust.dart';

class BrandCubit extends Cubit<BrandCubitState> {
  BrandCubit({required this.presenter}) : super(BrandCubitState(newBrandName: null, brands: List<String>.empty()));

  final BrandFlutterPresenter presenter;

  Future<void> createBrand() async {
    await presenter.addNewBrand(name: state.newBrandName ?? '');
    await updateBrands();
  }

  void onNewBrandNameChange(String? value) {
    emit(state.copyWith(newBrandName: Some(value)));
  }

  Future<void> updateBrands() async {
    // final List<BrandDisplayModel> brands = await brandPresenter.retrieveAllBrands();
    // emit(state.copyWith(brands: brands.map((b) => b.name).toList()));
  }
}

final class BrandCubitState extends Equatable {
  const BrandCubitState({required this.newBrandName, required this.brands});

  final String? newBrandName;
  final List<String> brands;

  @override
  List<Object?> get props => [newBrandName, brands];

  BrandCubitState copyWith({Option<String?>? newBrandName, List<String>? brands}) =>
      BrandCubitState(brands: brands ?? this.brands, newBrandName: newBrandName?.toNullable() ?? this.newBrandName);

  @override
  String toString() => 'BrandCubitState(brands: $brands, newBrandName: $newBrandName)';
}
