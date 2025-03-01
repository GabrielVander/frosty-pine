import 'package:flutter/material.dart';
import 'package:flutter_application/src/presentation/glue/api/simple.dart';
import 'package:flutter_application/src/presentation/utils/context_extensions.dart';
import 'package:flutter_application/src/presentation/widgets/brands/brands_tab.dart';

class MainPage extends StatelessWidget {
  const MainPage({super.key});

  @override
  Widget build(BuildContext context) {
    return DefaultTabController(
      length: 2,
      child: Scaffold(
        appBar: AppBar(
          title: Text(context.i18n('main_page.title')),
          bottom: TabBar(
            tabs: [
              Tab(text: context.i18n('main_page.home_tab_title'), icon: const Icon(Icons.home)),
              Tab(text: context.i18n('main_page.brands_tab_title'), icon: const Icon(Icons.shop)),
            ],
          ),
        ),
        body: TabBarView(
          children: [
            Center(child: Text('Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`')),
            BrandsTab(),
          ],
        ),
      ),
    );
  }
}
