import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:frosty_pine/framework/ui/utils/context_extensions.dart';
import 'package:frosty_pine/framework/ui/widgets/brands/state/brand_cubit.dart';

class BrandsTab extends StatelessWidget {
  const BrandsTab({required this.cubit, super.key});

  final BrandCubit cubit;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Expanded(child: BrandListDisplay(cubit: cubit)),
        Row(mainAxisAlignment: MainAxisAlignment.center, children: [AddNewBrandButton(cubit: cubit)]),
      ],
    );
  }
}

class AddNewBrandButton extends StatelessWidget {
  const AddNewBrandButton({required this.cubit, super.key});

  final BrandCubit cubit;

  @override
  Widget build(BuildContext context) {
    return IconButton(
      onPressed: () => showDialog<void>(context: context, builder: (BuildContext context) => NewBrandDialog(cubit: cubit)),
      icon: const Icon(Icons.add),
      iconSize: 60,
      color: Theme.of(context).primaryColor,
    );
  }
}

class NewBrandDialog extends StatelessWidget {
  const NewBrandDialog({required this.cubit, super.key});

  final BrandCubit cubit;

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text(context.i18n('brands_tab.new_brand_dialog.title')),
      content: BlocBuilder(
        bloc: cubit,
        builder:
            (BuildContext context, BrandCubitState state) => TextField(
              controller: TextEditingController.fromValue(
                TextEditingValue(text: state.newBrandName ?? '', selection: TextSelection.fromPosition(TextPosition(offset: state.newBrandName?.length ?? 0))),
              ),
              onChanged: cubit.onNewBrandNameChange,
            ),
      ),
      actions: <Widget>[
        TextButton(
          child: Text(context.i18n('brands_tab.new_brand_dialog.cancel_button')),
          onPressed: () {
            Navigator.of(context).pop();
          },
        ),
        ElevatedButton(onPressed: cubit.createBrand, child: Text(context.i18n('brands_tab.new_brand_dialog.save_button'))),
      ],
    );
  }
}

class BrandListDisplay extends StatelessWidget {
  const BrandListDisplay({required this.cubit, super.key});

  final BrandCubit cubit;

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<BrandCubit, BrandCubitState>(
      bloc: cubit,
      builder: (BuildContext context, BrandCubitState state) {
        if (state.brands.isEmpty) {
          return Container();
        }

        return ListView.separated(
          itemCount: state.brands.length,
          itemBuilder: (_, index) => Card(child: ListTile(title: Text(state.brands[index]))),
          separatorBuilder: (_, _) => const SizedBox(height: 10),
        );
      },
    );
  }
}
