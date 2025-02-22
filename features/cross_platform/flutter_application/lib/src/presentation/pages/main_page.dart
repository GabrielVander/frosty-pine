import 'package:flutter/material.dart';
import 'package:flutter_application/src/presentation/glue/api/simple.dart';

class MainPage extends StatelessWidget {
  const MainPage({super.key});

  @override
  Widget build(BuildContext context) {
    return DefaultTabController(
      length: 1,
      child: Scaffold(
        appBar: AppBar(
          title: const Text('flutter_rust_bridge quickstart'),
          bottom: const TabBar(tabs: [Tab(text: 'Home', icon: Icon(Icons.home))]),
        ),
        body: TabBarView(
          children: [Center(child: Text('Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`'))],
        ),
      ),
    );
  }
}
