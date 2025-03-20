import 'package:frosy_pine/features/core/domain/entities/product.dart';

final class Item {
  Item({
    required this.id,
    required this.product,
    required this.unit,
    required this.unitaryPrice,
  });

  final String id;
  final Product product;
  final Unit unit;
  final double unitaryPrice;

  double calculateFullPrice() => switch (unit) {
    NoneUnit() => unitaryPrice,
    QuantityUnit(amount: final int amount) => amount * unitaryPrice,
    KilogramsUnit(weight: final double weight) => weight * unitaryPrice,
    LitersUnit(volume: final double volume) => volume * unitaryPrice,
  };
}

sealed class Unit {}

final class NoneUnit extends Unit {}

final class QuantityUnit extends Unit {
  QuantityUnit({required this.amount});

  final int amount;
}

final class KilogramsUnit extends Unit {
  KilogramsUnit({required this.weight});

  final double weight;
}

final class LitersUnit extends Unit {
  LitersUnit({required this.volume});

  final double volume;
}
