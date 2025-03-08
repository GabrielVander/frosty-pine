import 'package:frosy_pine/features/core/domain/entities/brand.dart';
import 'package:frosy_pine/features/core/domain/repositories/brand_repository.dart';
import 'package:rust/rust.dart';

class BrandRepositoryInMemoryImpl implements BrandRepository {
  BrandRepositoryInMemoryImpl({required this.data});

  final Map<String, String> data;

  @override
  Future<Result<Brand, BrandRepositoryFailure>> fetchSingleById(String id) async {
    final String? brandName = data[id];

    if (brandName == null) {
      return Err<Brand, BrandRepositoryFailure>(BrandRepositoryNotFoundFailure(message: 'Brand not found', details: 'Unable to find brand with id $id'));
    }

    return Ok<Brand, BrandRepositoryFailure>(Brand(id: id, name: brandName));
  }
}
