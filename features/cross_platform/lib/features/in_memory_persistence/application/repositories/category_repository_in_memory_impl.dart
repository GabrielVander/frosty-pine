import 'package:frosty_pine/features/core/domain/entities/category.dart';
import 'package:frosty_pine/features/core/domain/repositories/category_repository.dart';
import 'package:rust/rust.dart';

class CategoryRepositoryInMemoryImpl implements CategoryRepository {
  CategoryRepositoryInMemoryImpl({required this.data});

  final Map<String, String> data;

  @override
  Future<Result<Category, CategoryRepositoryFailure>> fetchSingleById(String id) async {
    final String? brandName = data[id];

    if (brandName == null) {
      return Err<Category, CategoryRepositoryFailure>(
        CategoryRepositoryNotFoundFailure(message: 'Category not found', details: 'Unable to find category with id $id'),
      );
    }

    return Ok<Category, CategoryRepositoryFailure>(Category(id: id, name: brandName));
  }
}
