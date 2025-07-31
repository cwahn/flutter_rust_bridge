import 'package:frb_example_pure_dart/src/rust/api/generics.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('Generic Types', () {
    test('Container<String>', () async {
      final container = await createStringContainer();
      expect(container.value, equals('hello'));
      expect(container.count, equals(1));
      
      final result = await processStringContainer(container);
      expect(result, equals(6)); // count(1) + length('hello'=5)
    });

    test('Container<i32>', () async {
      final container = await createIntContainer();
      expect(container.value, equals(42));
      expect(container.count, equals(1));
    });

    test('Pair<i32, String>', () async {
      final pair = await createPair();
      expect(pair.first, equals(10));
      expect(pair.second, equals('world'));
    });

    test('Option<String>', () async {
      final someResult = await handleStringOption(Option.some('test'));
      expect(someResult, equals('test'));
      
      final noneResult = await handleStringOption(Option.none());
      expect(noneResult, equals('empty'));
    });

    test('Either<i32, String>', () async {
      final leftResult = await handleIntEither(Either.left(42));
      expect(leftResult, equals('number: 42'));
      
      final rightResult = await handleIntEither(Either.right('hello'));
      expect(rightResult, equals('text: hello'));
    });

    test('Nested generics: Container<Option<String>>', () async {
      final container = await createNestedContainer();
      expect(container.count, equals(1));
      
      final result = await processNestedContainer(container);
      expect(result, equals('nested'));
    });
  });
}
