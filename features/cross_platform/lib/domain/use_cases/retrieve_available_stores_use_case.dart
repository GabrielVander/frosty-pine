import 'package:frosty_pine/domain/entities/failures.dart';
import 'package:frosty_pine/domain/entities/store.dart';
import 'package:frosty_pine/domain/repositories/store_repository.dart';
import 'package:rust/rust.dart';

class RetrieveAvailableStoresUseCase {
  RetrieveAvailableStoresUseCase({required this.storeRepository});

  final StoreRepository storeRepository;

  Future<Result<List<Store>, RetrieveAvailableStoresUseCaseFailure>> execute() async {
    return (await storeRepository.retrieveAllStores()).mapErr<RetrieveAvailableStoresUseCaseFailure>(
      RetrieveAvailableStoresUseCaseFailure.fromStoreRepositoryFailure,
    );
  }

  Future<Result<List<Store>, RetrieveAvailableStoresUseCaseFailure>> call() => execute();
}

sealed class RetrieveAvailableStoresUseCaseFailure extends Failure {
  const RetrieveAvailableStoresUseCaseFailure({required super.message, required super.details});

  static RetrieveAvailableStoresUseCaseFailure fromStoreRepositoryFailure(StoreRepositoryFailure failure) => switch (failure) {
    StoreRepositoryUnexpectedFailure(message: final String? message, details: final String? details) => RetrieveAvailableStoresUseCaseGenericFailure(
      message: 'Something went wrong',
      details: '$message\n\n$details',
    ),
  };
}

final class RetrieveAvailableStoresUseCaseGenericFailure extends RetrieveAvailableStoresUseCaseFailure {
  const RetrieveAvailableStoresUseCaseGenericFailure({required super.message, required super.details});
}
