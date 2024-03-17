import Foundation

// Variables and Constants
var variableName: String = "Hello"
let constantName = "World"

// Data Types
let integer: Int = 100
let floatingPoint: Float = 10.5
let doublePrecision: Double = 20.123456
let boolean: Bool = true
let string: String = "A String"
let character: Character = "C"

// Collections
var array: [Int] = [1, 2, 3]
var dictionary: [String: Int] = ["one": 1, "two": 2]
var set: Set<Int> = [1, 2, 2, 3, 3]

// Optional
var optionalString: String? = "Optional"
optionalString = nil

// Functions
func myFunction(parameter: Int) -> Int {
    return parameter * 2
}

// Closure
let multiplyClosure: (Int, Int) -> Int = { $0 * $1 }

// Enumerations
enum Direction {
    case north, south, east, west
}

// Structs
struct Point {
    var x: Int
    var y: Int
}

// Classes
class Vehicle {
    var speed: Int = 0
    func description() -> String {
        return "Traveling at \(speed) miles per hour"
    }
}

// Inheritance
class Bicycle: Vehicle {
    var hasBasket = false
}

// Protocol
protocol Named {
    var name: String { get }
}

// Extensions
extension Int {
    func squared() -> Int {
        return self * self
    }
}

// Generics
func swapTwoValues<T>(_ a: inout T, _ b: inout T) {
    let temporaryA = a
    a = b
    b = temporaryA
}

// Error Handling
enum PrinterError: Error {
    case outOfPaper, noToner, onFire
}

// Concurrency
let queue = DispatchQueue(label: "com.example.myqueue")
queue.async {
    print("Do something asynchronously")
}

// Pattern Matching
let someValue = 42
switch someValue {
case 0..<50:
    print("Value is between 0 and 49")
default:
    print("Value is 50 or greater")
}

// Attributes
@available(iOS 10, *)
func newerFunction() {
    // Only available on iOS 10 and later
}

// Property Observers
class StepCounter {
    var totalSteps: Int = 0 {
        willSet(newTotalSteps) {
            print("About to set totalSteps to \(newTotalSteps)")
        }
        didSet {
            if totalSteps > oldValue {
                print("Added \(totalSteps - oldValue) steps")
            }
        }
    }
}

// Computed Properties
struct Rectangle {
    var width: Double
    var height: Double
    var area: Double {
        return width * height
    }
}

// Protocol Conformance & Extension
protocol ExampleProtocol {
    var simpleDescription: String { get }
    mutating func adjust()
}

extension Int: ExampleProtocol {
    var simpleDescription: String {
        return "The number \(self)"
    }
    mutating func adjust() {
        self += 42
    }
}

// Higher-order Functions
let numbers = [1, 2, 3, 4, 5]
let doubledNumbers = numbers.map { $0 * 2 }
let evenNumbers = numbers.filter { $0 % 2 == 0 }

// Defer
func processFile(filename: String) throws {
    let file = open(filename)
    defer {
        close(file)
    }
    // Work with the file
}

// Type Casting and Inspection
class MediaItem {
    var name: String
    init(name: String) {
        self.name = name
    }
}

class Movie: MediaItem {
    var director: String
    init(name: String, director: String) {
        self.director = director
        super.init(name: name)
    }
}

let items: [MediaItem] = [
    Movie(name: "Casablanca", director: "Michael Curtiz"),
    Movie(name: "Citizen Kane", director: "Orson Welles")
]

for item in items {
    if let movie = item as? Movie {
        print("Movie: '\(movie.name)', dir. \(movie.director)")
    }
}

// Pattern Matching in More Detail
let someCharacter: Character = "e"
switch someCharacter {
case "a", "e", "i", "o", "u":
    print("\(someCharacter) is a vowel")
default:
    print("\(someCharacter) is not a vowel")
}

// Type Aliases
typealias AudioSample = UInt16
var maxAmplitudeFound = AudioSample.min // UInt16.min

// Guard Statements
func greet(person: [String: String]) {
    guard let name = person["name"] else {
        return
    }
    print("Hello \(name)!")
}

// Swift’s Error Handling
do {
    try processFile(filename: "test.txt")
} catch {
    print("An error occurred: \(error)")
}

import SwiftUI

// Async/Await for Concurrency
func fetchDocument() async throws -> String {
    // Simulate fetching document from a network
    return "Document contents"
}

async func exampleAsyncFunction() {
    do {
        let document = try await fetchDocument()
        print(document)
    } catch {
        print("An error occurred")
    }
}

// Property Wrappers
@propertyWrapper
struct Capitalized {
    var wrappedValue: String {
        didSet { wrappedValue = wrappedValue.capitalized }
    }
    
    init(wrappedValue: String) {
        self.wrappedValue = wrappedValue.capitalized
    }
}

struct User {
    @Capitalized var name: String
}

// Opaque Types
protocol Shape {
    func draw() -> String
}

struct Circle: Shape {
    func draw() -> String {
        return "Drawing a circle"
    }
}

func makeShape() -> some Shape {
    Circle()
}

// SwiftUI for UI Development
struct ContentView: View {
    var body: some View {
        Text("Hello, SwiftUI!")
            .padding()
    }
}

// Using Combine for reactive programming
import Combine

class ObservableObjectExample: ObservableObject {
    @Published var counter = 0
    
    func increment() {
        counter += 1
    }
}

// Swift Package Manager manifest file example (Package.swift)
// swift-tools-version:5.3
import PackageDescription

let package = Package(
    name: "MyLibrary",
    products: [
        .library(
            name: "MyLibrary",
            targets: ["MyLibrary"]),
    ],
    dependencies: [
        .package(url: "https://github.com/apple/swift-algorithms", from: "0.0.1"),
    ],
    targets: [
        .target(
            name: "MyLibrary",
            dependencies: []),
        .testTarget(
            name: "MyLibraryTests",
            dependencies: ["MyLibrary"]),
    ]
)

// Reflectable enums with associated values
enum Measurement {
    case weight(Double)
    case age(Int)
    case height(Double)
    
    var value: Double {
        switch self {
        case .weight(let value), .height(let value):
            return value
        case .age(let value):
            return Double(value)
        }
    }
}

import Foundation

// Compiler Control Statements
#if DEBUG
print("Debug mode is enabled")
#else
print("Debug mode is not enabled")
#endif

// Deprecation and Obsoletion
@available(iOS, deprecated: 10.0, message: "Use newerMethod() instead")
func deprecatedMethod() {
    // This method is deprecated
}

@available(*, unavailable, renamed: "newerMethod")
func oldMethod() {
    // This method is not available and has been renamed
}

// Specifying API availability
@available(iOS 13, macOS 10.15, *)
func featureOnlyAvailableOnNewerOS() {
    // Use iOS 13+ or macOS 10.15+ APIs here
}

// Dynamic Member Lookup
@dynamicMemberLookup
class DynamicStruct {
    var dictionary = [String: String]()
    subscript(dynamicMember member: String) -> String {
        get { dictionary[member, default: ""] }
        set { dictionary[member] = newValue }
    }
}

let dynamic = DynamicStruct()
dynamic.hello = "world" // Uses dynamic member lookup

// Property Wrappers - More Advanced Use
@propertyWrapper
struct UserDefaultsBacked<Value> {
    let key: String
    var wrappedValue: Value? {
        get { UserDefaults.standard.object(forKey: key) as? Value }
        set { UserDefaults.standard.set(newValue, forKey: key) }
    }
}

class Settings {
    @UserDefaultsBacked(key: "firstLaunch")
    var firstLaunch: Bool?
}

// Swift's Result Type
func fetchData(completion: @escaping (Result<String, Error>) -> Void) {
    let success = true // Simulate fetch result
    if success {
        completion(.success("Data fetched successfully"))
    } else {
        completion(.failure(NSError(domain: "FetchError", code: 1, userInfo: nil)))
    }
}

// Compile-Time Diagnostic Directives
func myFunction() {
    #warning("This function needs to be refactored")
    #error("This function should not be used in production")
}

// Swift's Opaque Return Types
func makeOpaqueShape() -> some Shape {
    return Circle() // Assuming Circle conforms to the Shape protocol
}

// Swift's Function Builders
@_functionBuilder
struct HTMLBuilder {
    static func buildBlock(_ components: String...) -> String {
        return "<html>" + components.joined(separator: "") + "</html>"
    }
}

// Using the Function Builder
@HTMLBuilder func buildHTML() -> String {
    "<head></head>"
    "<body><p>Hello, world!</p></body>"
}

// Conditional Compilation for Arch, OS, and Swift Version
#if arch(x86_64) || arch(arm64)
// Compile for x86_64 and ARM64 architectures only
#endif

#if os(iOS) || os(macOS)
// Compile for iOS and macOS only
#endif

#if swift(>=5.3)
// Use Swift 5.3 features
#endif

import SwiftUI

// State Management with @State
struct CounterView: View {
    @State private var count = 0
    
    var body: some View {
        VStack {
            Text("Count: \(count)")
            Button("Increment") {
                count += 1
            }
        }
    }
}

// Binding properties with @Binding
struct ChildView: View {
    @Binding var value: Int
    
    var body: some View {
        Button("Increment") {
            value += 1
        }
    }
}

// Using @ObservedObject for more complex state management
class UserSettings: ObservableObject {
    @Published var score = 0
}

struct ContentView: View {
    @ObservedObject var settings = UserSettings()
    
    var body: some View {
        VStack {
            Text("Score: \(settings.score)")
            Button("Increase Score") {
                settings.score += 1
            }
        }
    }
}

// Environment values with @Environment
struct EnvironmentView: View {
    @Environment(\.presentationMode) var presentationMode
    
    var body: some View {
        Button("Dismiss") {
            presentationMode.wrappedValue.dismiss()
        }
    }
}

// Custom Modifiers
struct TitleStyle: ViewModifier {
    func body(content: Content) -> some View {
        content
            .font(.largeTitle)
            .foregroundColor(.blue)
    }
}

extension View {
    func titleStyle() -> some View {
        self.modifier(TitleStyle())
    }
}

// Using custom modifiers
struct TitleView: View {
    var body: some View {
        Text("Hello, World!")
            .titleStyle()
    }
}

// PreviewProvider for SwiftUI previews
struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}

// Lazy loading views with LazyVStack and LazyHStack
struct LazyLoadingView: View {
    var body: some View {
        ScrollView {
            LazyVStack {
                ForEach(1...1000, id: \.self) { index in
                    Text("Row \(index)")
                }
            }
        }
    }
}

// GeometryReader for flexible layouts
struct GeometryReaderExample: View {
    var body: some View {
        GeometryReader { geometry in
            VStack {
                Text("Top")
                    .frame(width: geometry.size.width, height: geometry.size.height / 2)
                    .background(Color.green)
                Text("Bottom")
                    .frame(width: geometry.size.width, height: geometry.size.height / 2)
                    .background(Color.blue)
            }
        }
    }
}

import Foundation

// Optionals and Optional Handling
var optionalString: String? = "Hello"
var optionalInt: Int?
if let unwrappedString = optionalString {
    print(unwrappedString)
}
optionalInt?.description // Optional chaining
func unwrapOptional(value: String?) {
    guard let unwrappedValue = value else {
        print("Value was nil")
        return
    }
    print(unwrappedValue)
}

// Enums with Associated Values
enum Barcode {
    case upc(Int, Int, Int, Int)
    case qrCode(String)
}
var productBarcode = Barcode.upc(8, 85909, 51226, 3)
productBarcode = .qrCode("ABCDEFGHIJKLMNOP")
switch productBarcode {
case let .upc(numberSystem, manufacturer, product, check):
    print("UPC: \(numberSystem), \(manufacturer), \(product), \(check).")
case let .qrCode(productCode):
    print("QR code: \(productCode).")
}

// Structs with Computed Properties
struct Point {
    var x = 0.0, y = 0.0
    var isOrigin: Bool {
        return x == 0.0 && y == 0.0
    }
}
let originPoint = Point(x: 0.0, y: 0.0)
print(originPoint.isOrigin)

// Protocols and Extensions
protocol ExampleProtocol {
    var simpleDescription: String { get }
    mutating func adjust()
}
extension Int: ExampleProtocol {
    var simpleDescription: String { return "The number \(self)" }
    mutating func adjust() { self += 42 }
}
print(7.simpleDescription)

// Generics
func swapTwoValues<T>(_ a: inout T, _ b: inout T) {
    let temporaryA = a
    a = b
    b = temporaryA
}
var someInt = 3
var anotherInt = 107
swapTwoValues(&someInt, &anotherInt)
print("someInt is now \(someInt), and anotherInt is now \(anotherInt)")

// Closures
let names = ["Chris", "Alex", "Ewa", "Barry", "Daniella"]
var reversedNames = names.sorted(by: { $0 > $1 })
print(reversedNames)

// Property Observers
class StepCounter {
    var totalSteps: Int = 0 {
        willSet(newTotalSteps) {
            print("About to set totalSteps to \(newTotalSteps)")
        }
        didSet {
            if totalSteps > oldValue  {
                print("Added \(totalSteps - oldValue) steps")
            }
        }
    }
}
var stepCounter = StepCounter()
stepCounter.totalSteps = 200

// Error Handling
enum PrinterError: Error {
    case outOfPaper, noToner, onFire
}
func send(job: Int, toPrinter printerName: String) throws -> String {
    if printerName == "Never Has Toner" {
        throw PrinterError.noToner
    }
    return "Job sent"
}

do {
    let printerResponse = try send(job: 1040, toPrinter: "Bi Sheng")
    print(printerResponse)
} catch PrinterError.onFire {
    print("I'll just put this over here, with the rest of the fire.")
} catch let printerError as PrinterError {
    print("Printer error: \(printerError).")
} catch {
    print(error)
}

import Foundation

// Static properties and methods
class SomeClass {
    static var staticProperty: String = "Static Property"
    static func staticMethod() {
        print("Static method on SomeClass called.")
    }
}

SomeClass.staticMethod()
print(SomeClass.staticProperty)

// Extensions with static properties
extension SomeClass {
    static var extensionStaticProperty: String = "Extended Static Property"
}

print(SomeClass.extensionStaticProperty)

// Using static properties in protocol extensions
protocol Identifiable {
    static var identifier: String { get }
}

extension Identifiable {
    static var identifier: String {
        return "ID:\(Self.self)"
    }
}

class MyIdentifiableClass: Identifiable {}

print(MyIdentifiableClass.identifier)

// Enums with methods
enum Beverage: CaseIterable {
    case coffee, tea, juice
    func describe() -> String {
        switch self {
        case .coffee:
            return "Hot and caffeinated."
        case .tea:
            return "A variety of options."
        case .juice:
            return "Fruit derived."
        }
    }
}

let myBeverage = Beverage.coffee
print(myBeverage.describe())

// Pattern matching with tuples
let somePoint = (1, 1)
switch somePoint {
case (0, 0):
    print("At the origin")
case (_, 0):
    print("On the x-axis")
case (0, _):
    print("On the y-axis")
case (-2...2, -2...2):
    print("Inside the box")
default:
    print("Outside of the box")
}

// Type properties and methods
struct Math {
    static let pi = 3.14159
    static func square(of number: Int) -> Int {
        return number * number
    }
}

print(Math.pi)
print(Math.square(of: 5))

// Subscripts
struct TimesTable {
    let multiplier: Int
    subscript(index: Int) -> Int {
        return multiplier * index
    }
}

let threeTimesTable = TimesTable(multiplier: 3)
print("6 times 3 is \(threeTimesTable[6])")

// Advanced pattern matching
let anotherPoint = (2, 0)
switch anotherPoint {
case let (x, y) where x == y:
    print("(\(x), \(y)) is on the line x == y")
case let (x, y) where x == -y:
    print("(\(x), \(y)) is on the line x == -y")
case let (x, y):
    print("Just some arbitrary point: (\(x), \(y))")
}

import Foundation
import Darwin // Or `import Glibc` on Linux for UNIX-like environments

// Unsafe Mutable and Immutable Pointers
var number: Int = 42
withUnsafeMutablePointer(to: &number) { ptr in
    ptr.pointee = 24
}
withUnsafePointer(to: &number) { ptr in
    print(ptr.pointee) // Prints 24
}

// Working with Arrays and Unsafe Buffer Pointers
let numbers = [1, 2, 3, 4, 5]
numbers.withUnsafeBufferPointer { buffer in
    for i in 0..<buffer.count {
        print(buffer[i])
    }
}

// Interoperability with C
print(sqrt(2.0)) // Using the C standard library's sqrt function

var tv = timeval(tv_sec: 5, tv_usec: 0)
withUnsafePointer(to: &tv) { t in
    // This pointer `t` can be passed to C functions expecting a `timeval` pointer
}

// Memory Management and Avoiding Retain Cycles
class Parent {
    var child: Child?
    deinit { print("Parent deinitialized") }
}

class Child {
    weak var parent: Parent?
    deinit { print("Child deinitialized") }
}

var parent: Parent? = Parent()
var child: Child? = Child()

parent!.child = child
child!.parent = parent

parent = nil // child is automatically deallocated when parent is set to nil due to the weak reference

// Demonstrates the child being deallocated automatically due to the weak reference, preventing a retain cycle
