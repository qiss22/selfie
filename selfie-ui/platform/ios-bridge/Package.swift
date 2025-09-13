// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "SelfieUIBridge",
    platforms: [.iOS(.v14)],
    products: [
        .library(name: "SelfieUIBridge", targets: ["SelfieUIBridge"]),
    ],
    targets: [
        .target(name: "SelfieUIBridge"),
    ]
)