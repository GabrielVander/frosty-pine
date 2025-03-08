import 'package:frosy_pine/features/core/domain/entities/brand.dart';
import 'package:frosy_pine/features/core/domain/entities/category.dart';

final class Product {
  Product({required this.id, required this.name, required this.brand, required this.category});

  final String id;
  final String name;
  final Brand brand;
  final Category category;
}
