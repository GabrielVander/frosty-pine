import 'package:frosy_pine/features/core/domain/entities/store.dart';
import 'package:frosy_pine/features/core/domain/repositories/store_repository.dart';
import 'package:rust/rust.dart';

class StoreRepositoryInMemoryImpl implements StoreRepository {
  StoreRepositoryInMemoryImpl({required this.data});

  final Map<String, String> data;

  @override
  Future<Result<List<Store>, StoreRepositoryFailure>> retrieveAllStores() async =>
      Ok<List<Store>, StoreRepositoryFailure>(data.entries.map<Store>(_toStoreEntity).toList());

  Store _toStoreEntity(MapEntry<String, String> entry) => Store(id: entry.key, name: entry.value);
}
