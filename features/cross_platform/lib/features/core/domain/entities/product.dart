import 'package:frosy_pine/features/core/domain/entities/brand.dart';
import 'package:frosy_pine/features/core/domain/entities/category.dart';

final class Product {
  Product({
    required this.id,
    required this.name,
    required this.brand,
    required this.category,
  });

  final String id;
  final String name;
  final Brand brand;
  final Category category;
}

final class LazyProduct {
  LazyProduct({
    required this.id,
    required this.name,
    required this.brandId,
    required this.categoryId,
  });

  final String id;
  final String name;
  final String brandId;
  final String categoryId;

  Product toProduct(Brand brand, Category category) =>
      Product(id: id, name: name, brand: brand, category: category);
}
