import 'dart:async';
import 'package:frosty_pine/domain/entities/brand.dart';
import 'package:frosty_pine/domain/entities/category.dart';
import 'package:frosty_pine/domain/entities/failures.dart';
import 'package:frosty_pine/domain/entities/product.dart';
import 'package:frosty_pine/domain/repositories/brand_repository.dart';
import 'package:frosty_pine/domain/repositories/category_repository.dart';
import 'package:frosty_pine/domain/repositories/product_repository.dart';
import 'package:rust/rust.dart';

class RetrieveAvailableProductsUseCase {
  RetrieveAvailableProductsUseCase({
    required ProductRepository productRepository,
    required BrandRepository brandRepository,
    required CategoryRepository categoryRepository,
  }) : _productRepository = productRepository,
       _brandRepository = brandRepository,
       _categoryRepository = categoryRepository;

  final ProductRepository _productRepository;
  final BrandRepository _brandRepository;
  final CategoryRepository _categoryRepository;

  Future<Result<List<Product>, RetrieveAvailableProductsUseCaseFailure>> execute() async => _fetchLazyProducts().andThen<List<Product>>(_toProducts);

  Future<Result<List<Product>, RetrieveAvailableProductsUseCaseFailure>> call() => execute();

  FutureResult<List<LazyProduct>, RetrieveAvailableProductsUseCaseFailure> _fetchLazyProducts() {
    return _productRepository.fetchAllLazy().mapErr<RetrieveAvailableProductsUseCaseFailure>(RetrieveAvailableProductsUseCaseFailure.fromFailure);
  }

  FutureOr<Result<List<Product>, RetrieveAvailableProductsUseCaseFailure>> _toProducts(List<LazyProduct> lazyProducts) async =>
      lazyProducts.map<Future<Result<Product, RetrieveAvailableProductsUseCaseFailure>>>(_toProduct).toResultEager();

  Future<Result<Product, RetrieveAvailableProductsUseCaseFailure>> _toProduct(LazyProduct lazyProduct) async {
    return _fetchBrand(lazyProduct.brandId).andThen<Product>(
      (Brand brand) async => _fetchCategory(lazyProduct.categoryId).map<Product>((Category category) => lazyProduct.toProduct(brand, category)),
    );
  }

  Future<Result<Brand, RetrieveAvailableProductsUseCaseFailure>> _fetchBrand(String brandId) async =>
      (await _brandRepository.fetchSingleById(brandId)).mapErr(RetrieveAvailableProductsUseCaseFailure.fromFailure);

  Future<Result<Category, RetrieveAvailableProductsUseCaseFailure>> _fetchCategory(String categoryId) async =>
      (await _categoryRepository.fetchSingleById(categoryId)).mapErr(RetrieveAvailableProductsUseCaseFailure.fromFailure);
}

sealed class RetrieveAvailableProductsUseCaseFailure extends Failure {
  const RetrieveAvailableProductsUseCaseFailure({required super.message, required super.details});

  static RetrieveAvailableProductsUseCaseFailure fromFailure(Failure failure) =>
      RetrieveAvailableProductsUseCaseGenericFailure(message: 'Something went wrong', details: '${failure.message}\n\n${failure.details}');
}

final class RetrieveAvailableProductsUseCaseGenericFailure extends RetrieveAvailableProductsUseCaseFailure {
  const RetrieveAvailableProductsUseCaseGenericFailure({required super.message, required super.details});
}
