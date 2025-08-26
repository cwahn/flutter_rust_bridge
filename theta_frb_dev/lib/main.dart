import 'package:flutter/material.dart' as flt;
import 'package:theta_frb_dev/src/rust/api/simple.dart';
import 'package:theta_frb_dev/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return flt.MaterialApp(home: const CounterTestPage());
  }
}

class CounterTestPage extends StatefulWidget {
  const CounterTestPage({super.key});

  @override
  State<CounterTestPage> createState() => _CounterTestPageState();
}

class _CounterTestPageState extends State<CounterTestPage> {
  @override
  Widget build(BuildContext context) {
    return flt.Scaffold(
      appBar: flt.AppBar(
        title: const flt.Text('ActorRef<Counter> Tests'),
        backgroundColor: flt.Colors.blue[100],
      ),
      body: flt.Padding(
        padding: const flt.EdgeInsets.all(20.0),
        child: flt.Column(
          children: [
            const flt.Text(
              'ActorRef<Counter> Test Interface',
              style: flt.TextStyle(
                fontSize: 24,
                fontWeight: flt.FontWeight.bold,
              ),
            ),
            const flt.SizedBox(height: 20),
            flt.FutureBuilder<ActorRef<Counter>>(
              future: ActorRef<Counter>.connect("counter"),
              builder: (context, snapshot) {
                if (snapshot.connectionState == flt.ConnectionState.waiting) {
                  return const flt.Center(
                    child: flt.Column(
                      children: [
                        flt.CircularProgressIndicator(),
                        flt.SizedBox(height: 16),
                        flt.Text('Connecting to counter...'),
                      ],
                    ),
                  );
                } else if (snapshot.hasError) {
                  return flt.Container(
                    padding: const flt.EdgeInsets.all(16),
                    decoration: flt.BoxDecoration(
                      color: flt.Colors.red[50],
                      borderRadius: flt.BorderRadius.circular(8),
                      border: flt.Border.all(color: flt.Colors.red[200]!),
                    ),
                    child: flt.Text(
                      'Failed to connect to counter: ${snapshot.error}',
                      style: const flt.TextStyle(
                        fontSize: 16,
                        fontWeight: flt.FontWeight.w500,
                        color: flt.Colors.red,
                      ),
                      textAlign: flt.TextAlign.center,
                    ),
                  );
                } else {
                  // return CounterWidget(ActorRef<counter>: snapshot.data!);
                  // Two same CounterWidget
                  return flt.Row(
                    children: [
                      CounterWidget(ActorRef<counter>: snapshot.data!),
                      CounterWidget(ActorRef<counter>: snapshot.data!),
                    ],
                  );
                }
              },
            ),
          ],
        ),
      ),
    );
  }
}

class CounterWidget extends StatelessWidget {
  final ActorRef<Counter> ActorRef<counter>;

  const CounterWidget({super.key, required this.ActorRef<counter>});

  void _increment() {
    print('[DEBUG] Increment button pressed');
    ActorRef<counter>.inc(msg: Inc(amount: 1));
  }

  void _decrement() {
    print('[DEBUG] Decrement button pressed');
    ActorRef<counter>.dec(msg: Dec(amount: 1));
  }

  void _performBulkTest() {
    // Perform a series of operations
    for (int i = 0; i < 5; i++) {
      ActorRef<counter>.inc(msg: Inc(amount: 2));
    }
    ActorRef<counter>.dec(msg: Dec(amount: 3));
  }

  @override
  Widget build(BuildContext context) {
    return flt.Column(
      children: [
        // Status Container
        flt.Container(
          padding: const flt.EdgeInsets.all(16),
          decoration: flt.BoxDecoration(
            color: flt.Colors.blue[50],
            borderRadius: flt.BorderRadius.circular(8),
            border: flt.Border.all(color: flt.Colors.blue[200]!),
          ),
          child: flt.Text(
            'Counter connected! Stream ready with snapshot: ${ActorRef<counter>.state.value}',
            style: const flt.TextStyle(
              fontSize: 16,
              fontWeight: flt.FontWeight.w500,
            ),
            textAlign: flt.TextAlign.center,
          ),
        ),
        const flt.SizedBox(height: 20),
        // Current Counter Value Display using StreamBuilder
        flt.StreamBuilder<CounterView>(
          stream: ActorRef<counter>.stream,
          initialData: ActorRef<counter>.state, // Use the prepared snapshot
          builder: (context, streamSnapshot) {
            print(
              '[DEBUG] StreamBuilder called: ${streamSnapshot.connectionState}, hasData: ${streamSnapshot.hasData}, error: ${streamSnapshot.error}',
            );

            String displayValue;
            flt.Color? textColor;

            if (streamSnapshot.hasError) {
              displayValue = 'Stream Error';
              textColor = flt.Colors.red[600];
            } else if (streamSnapshot.hasData) {
              displayValue = '${streamSnapshot.data!.value}';
              textColor = flt.Colors.green[800];
            } else {
              displayValue = 'No data';
              textColor = flt.Colors.grey[600];
            }

            return flt.Container(
              padding: const flt.EdgeInsets.all(20),
              decoration: flt.BoxDecoration(
                color: flt.Colors.green[50],
                borderRadius: flt.BorderRadius.circular(12),
                border: flt.Border.all(color: flt.Colors.green[300]!, width: 2),
              ),
              child: flt.Column(
                children: [
                  const flt.Text(
                    'Current Counter Value',
                    style: flt.TextStyle(
                      fontSize: 18,
                      fontWeight: flt.FontWeight.bold,
                    ),
                  ),
                  const flt.SizedBox(height: 10),
                  flt.Text(
                    displayValue,
                    style: flt.TextStyle(
                      fontSize: 32,
                      fontWeight: flt.FontWeight.bold,
                      color: textColor,
                    ),
                  ),
                ],
              ),
            );
          },
        ),
        const flt.SizedBox(height: 20),
        flt.Row(
          mainAxisAlignment: flt.MainAxisAlignment.spaceEvenly,
          children: [
            flt.ElevatedButton.icon(
              onPressed: _increment,
              icon: const flt.Icon(flt.Icons.add),
              label: const flt.Text('Inc +1'),
              style: flt.ElevatedButton.styleFrom(
                backgroundColor: flt.Colors.green[200],
                foregroundColor: flt.Colors.green[800],
              ),
            ),
            flt.ElevatedButton.icon(
              onPressed: _decrement,
              icon: const flt.Icon(flt.Icons.remove),
              label: const flt.Text('Dec -1'),
              style: flt.ElevatedButton.styleFrom(
                backgroundColor: flt.Colors.red[200],
                foregroundColor: flt.Colors.red[800],
              ),
            ),
            flt.ElevatedButton.icon(
              onPressed: _performBulkTest,
              icon: const flt.Icon(flt.Icons.flash_on),
              label: const flt.Text('Bulk Test'),
              style: flt.ElevatedButton.styleFrom(
                backgroundColor: flt.Colors.orange[200],
                foregroundColor: flt.Colors.orange[800],
              ),
            ),
          ],
        ),
        const flt.SizedBox(height: 20),
        const flt.Text(
          'Note: Using the new connect() API with prepared snapshot and cached stream.',
          style: flt.TextStyle(
            fontSize: 12,
            fontStyle: flt.FontStyle.italic,
            color: flt.Colors.grey,
          ),
          textAlign: flt.TextAlign.center,
        ),
      ],
    );
  }
}
