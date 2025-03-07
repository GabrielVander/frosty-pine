import 'package:flutter/material.dart';
import 'package:frosy_pine/presentation/utils/context_extensions.dart';
import 'package:frosy_pine/presentation/widgets/brands/brands_tab.dart';
import 'package:frosy_pine/presentation/widgets/home/home_tab.dart';
import 'package:frosy_pine/presentation/widgets/transactions/state/new_transaction_cubit.dart';
import 'package:frosy_pine/presentation/widgets/transactions/transactions_tab.dart';

class MainPage extends StatelessWidget {
  const MainPage({required this.newTransactionCubit, super.key});

  static const String i18nPrefix = 'main_page';
  final NewTransactionCubit newTransactionCubit;

  @override
  Widget build(BuildContext context) {
    return DefaultTabController(
      length: 3,
      child: Scaffold(
        appBar: AppBar(
          title: Text(context.i18n('$i18nPrefix.title')),
          bottom: TabBar(
            tabs: [
              Tab(text: context.i18n('$i18nPrefix.home_tab_title'), icon: const Icon(Icons.home)),
              Tab(text: context.i18n('$i18nPrefix.transactions_tab_title'), icon: const Icon(Icons.checklist)),
              Tab(text: context.i18n('$i18nPrefix.brands_tab_title'), icon: const Icon(Icons.shop)),
            ],
          ),
        ),
        body: TabBarView(
          children: [const HomeTab(), TransactionsTab(newTransactionCubit: newTransactionCubit), BrandsTab()],
        ),
      ),
    );
  }
}
