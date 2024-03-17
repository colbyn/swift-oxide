// Variable and Constant Declarations
var myVariable: Int = 10
let myConstant: String = "Hello, Swift!"

// Basic Data Types
var integer: Int = 42
var floatingPoint: Double = 3.14
var boolean: Bool = true
var string: String = "A string"

// Arrays and Dictionaries
var array: [Int] = [1, 2, 3]
var dictionary: [String: Int] = ["one": 1, "two": 2]

// Control Flow
if boolean {
    print("It's true!")
} else {
    print("It's false!")
}

for value in array {
    print(value)
}

while integer > 0 {
    integer -= 1
}

// Functions
func sayHello(to name: String) -> String {
    return "Hello, \(name)!"
}

// Calling a function
print(sayHello(to: "Swift"))

// Enumerations
enum CompassPoint {
    case north
    case south
    case east
    case west
}

var direction = CompassPoint.west

// Structures
struct Point {
    var x: Int
    var y: Int
}

var point = Point(x: 10, y: 20)

// Classes
class Vehicle {
    var numberOfWheels = 0
    func description() -> String {
        return "\(numberOfWheels) wheels"
    }
}

// Subclassing and Overriding
class Bicycle: Vehicle {
    override init() {
        super.init()
        numberOfWheels = 2
    }
}

// Protocols
protocol ExampleProtocol {
    var simpleDescription: String { get }
    mutating func adjust()
}

// Extensions
extension Int: ExampleProtocol {
    var simpleDescription: String { return "The number \(self)" }
    mutating func adjust() {
        self += 42
    }
}

// Generics
func swapTwoValues<T>(_ a: inout T, _ b: inout T) {
    let temporaryA = a
    a = b
    b = temporaryA
}

//
// # Optionals
//

var optionalString: String? = "Hello"
print(optionalString == nil) // false

var optionalName: String? = "John Appleseed"
var greeting = "Hello!"
if let name = optionalName {
    greeting = "Hello, \(name)"
}
// Optional binding is used to check if optionalName contains a value

//
// # Error Handling
//
enum PrinterError: Error {
    case outOfPaper
    case noToner
    case onFire
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
} catch {
    print(error)
}

//
// # Closures
//
let names = ["Chris", "Alex", "Ewa", "Barry", "Daniella"]

var reversedNames = names.sorted(by: { (s1: String, s2: String) -> Bool in
    return s1 > s2
})
// Example of a closure that sorts an array in reverse order

//
// # Protocol-Oriented Programming
//
protocol Identifiable {
    var id: String { get }
}

struct User: Identifiable {
    var id: String
}

func displayID(thing: Identifiable) {
    print("My ID is \(thing.id)")
}

let user = User(id: "12345")
displayID(thing: user)

//
// # Generics
//
func swapTwoValues<T>(_ a: inout T, _ b: inout T) {
    let temporaryA = a
    a = b
    b = temporaryA
}

var int1 = 100
var int2 = 200
swapTwoValues(&int1, &int2)
print("int1 is now \(int1), int2 is now \(int2)")

var str1 = "hello"
var str2 = "world"
swapTwoValues(&str1, &str2)
print("str1 is now \(str1), str2 is now \(str2)")

//
// # Asynchronous Programming with Async/Await
//
func fetchDocument() async throws -> String {
    // Imagine this function fetches a document asynchronously
    return "Document contents"
}

func printDocument() async {
    do {
        let document = try await fetchDocument()
        print(document)
    } catch {
        print("Unable to fetch the document.")
    }
}
