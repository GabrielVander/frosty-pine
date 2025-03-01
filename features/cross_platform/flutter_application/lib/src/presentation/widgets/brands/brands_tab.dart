import 'package:flutter/material.dart';
import 'package:flutter_application/src/presentation/utils/context_extensions.dart';

class BrandsTab extends StatelessWidget {
  BrandsTab({super.key});

  final List<String> brands = ['Nestlé', 'Nike', 'Google'];

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Expanded(
          child: ListView.separated(
            itemCount: brands.length,
            itemBuilder: (context, index) => Card(child: ListTile(title: Text(brands[index]))),
            separatorBuilder: (context, _) => const SizedBox(height: 10),
          ),
        ),
        Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            IconButton(
              onPressed:
                  () => showDialog<void>(
                    context: context,
                    builder: (BuildContext context) {
                      return AlertDialog(
                        title: Text(context.i18n('brands_tab.new_brand_dialog.title')),
                        content: Container(),
                        actions: <Widget>[
                          TextButton(
                            child: Text(context.i18n('brands_tab.new_brand_dialog.cancel_button')),
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
                    },
                  ),
              icon: const Icon(Icons.add),
              iconSize: 60,
              color: Theme.of(context).primaryColor,
            ),
          ],
        ),
      ],
    );
  }
}
