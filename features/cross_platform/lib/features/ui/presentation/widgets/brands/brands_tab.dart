import 'package:flutter/material.dart';
import 'package:frosy_pine/features/ui/presentation/utils/context_extensions.dart';

class BrandsTab extends StatelessWidget {
  BrandsTab({super.key});

  final List<String> brands = ['Nestlé', 'Nike', 'Google'];

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Expanded(child: BrandListDisplay(brands: brands)),
        const Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [AddNewBrandButton()],
        ),
      ],
    );
  }
}

class AddNewBrandButton extends StatelessWidget {
  const AddNewBrandButton({super.key});

  @override
  Widget build(BuildContext context) {
    return IconButton(
      onPressed:
          () => showDialog<void>(
            context: context,
            builder: (BuildContext context) => const NewBrandDialog(),
          ),
      icon: const Icon(Icons.add),
      iconSize: 60,
      color: Theme.of(context).primaryColor,
    );
  }
}

class NewBrandDialog extends StatelessWidget {
  const NewBrandDialog({super.key});

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text(context.i18n('brands_tab.new_brand_dialog.title')),
      content: Container(),
      actions: <Widget>[
        TextButton(
          child: Text(
            context.i18n('brands_tab.new_brand_dialog.cancel_button'),
          ),
          onPressed: () {
            Navigator.of(context).pop();
          },
        ),
        ElevatedButton(
          child: Text(context.i18n('brands_tab.new_brand_dialog.save_button')),
          onPressed: () {
            Navigator.of(context).pop();
          },
        ),
      ],
    );
  }
}

class BrandListDisplay extends StatelessWidget {
  const BrandListDisplay({required this.brands, super.key});

  final List<String> brands;

  @override
  Widget build(BuildContext context) {
    return ListView.separated(
      itemCount: brands.length,
      itemBuilder:
          (_, index) => Card(child: ListTile(title: Text(brands[index]))),
      separatorBuilder: (_, _) => const SizedBox(height: 10),
    );
  }
}
