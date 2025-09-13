import SwiftUI
import SharedCode  // KMP framework

@main
struct SelfieApp: App {{
    var body: some Scene {{
        WindowGroup {{
            ContentView()
        }}
    }}
}}

struct ContentView: View {{
    var body: some View {{
        Text("Hello, iOS from KMP!")
    }}
}}