
/// A Swift expression.
#[derive(Debug, Clone)]
pub enum Expression {
    /// Corresponds to `self` in Swift. Example: `self`
    SelfExpression,
    /// Corresponds to `super` in Swift. Example: `super.init()`
    SuperExpression,
    /// Corresponds to identifiers in Swift. Example: `myVariable`
    Identifier(expression::Identifier),
    /// Corresponds to literals in Swift. Example: `5`, `"Hello"`, `true`
    Literal(expression::Literal),
    /// Corresponds to binary expressions in Swift. Example: `a + b`
    BinaryExpression(Box<expression::BinaryExpression>),
    /// Corresponds to unary expressions in Swift. Example: `-a`, `!flag`
    UnaryExpression(Box<expression::UnaryExpression>),
    /// Corresponds to function call expressions in Swift. Example: `print("Hello")`
    CallExpression(Box<expression::CallExpression>),
    /// Corresponds to closure expressions in Swift. Example: `{ print("Hello") }`
    Closure(Box<expression::Closure>),
    /// Corresponds to subscript expressions in Swift. Example: `array[0]`
    Subscript(Box<expression::SubscriptExpression>),
    /// Corresponds to conditional expressions in Swift. Example: `a > b ? a : b`
    Conditional(Box<expression::InlineConditionalExpression>),
    /// Corresponds to tuple expressions in Swift. Example: `(1, "Hello")`
    Tuple(Box<expression::TupleExpression>),
    /// Corresponds to array expressions in Swift. Example: `[1, 2, 3]`
    Array(Box<expression::ArrayExpression>),
    /// Corresponds to dictionary expressions in Swift. Example: `["key": "value"]`
    Dictionary(Box<expression::DictionaryExpression>),
    /// Corresponds to member access expressions in Swift. Example: `object.property`
    MemberAccess(Box<expression::MemberAccessExpression>),
    /// Corresponds to type casting expressions in Swift. Example: `object as? MyClass`
    TypeCasting(Box<expression::TypeCastingExpression>),
    /// Corresponds to pattern match expressions in Swift. Example: `case .some(let x)`
    PatternMatch(Box<expression::PatternMatchExpression>),
    /// Corresponds to key path expressions in Swift. Example: `\Person.name`
    KeyPath(expression::KeyPathExpression),
    /// Corresponds to assignment expressions in Swift. Example: `a = b`
    Assignment(Box<expression::AssignmentExpression>),
}

pub mod expression {
    use super::{Expression, SwiftType};

    #[derive(Debug, Clone)]
    pub struct Identifier {
        pub name: String,
    }

    #[derive(Debug, Clone)]
    pub struct InfixIdentifier {
        pub symbol: String,
    }

    #[derive(Debug, Clone)]
    pub struct UnaryIdentifier {
        pub symbol: String,
    }

    /// Represents a literal in Swift. Examples: `5`, `3.14`, `true`, `"Hello"`, `'a'`
    #[derive(Debug, Clone)]
    pub enum Literal {
        Integer(i64),
        Float(f64),
        Bool(bool),
        String(String),
        Character(char),
        /// Represents `nil`.
        Nil,
    }

    /// Represents a binary expression in Swift. Example: `a + b`
    #[derive(Debug, Clone)]
    pub struct BinaryExpression {
        pub left: Box<Expression>,
        pub operator: InfixIdentifier,
        pub right: Box<Expression>,
    }

    /// Represents a unary expression in Swift. Examples: `-a`, `!flag`
    #[derive(Debug, Clone)]
    pub struct UnaryExpression {
        pub operator: UnaryIdentifier,
        pub operand: Box<Expression>,
    }

    #[derive(Debug, Clone)]
    pub struct CallExpression {
        /// The expression that results in a callable entity.
        pub callee: Box<Expression>,
        /// The arguments passed to the call, excluding trailing closures.
        pub arguments: Vec<Argument>,
        /// Optional generic type arguments for the call.
        pub generic_type_arguments: Option<Vec<SwiftType>>,
        /// A vector of trailing closures.
        pub trailing_closures: Vec<TrailingClosure>,
    }

    #[derive(Debug, Clone)]
    pub struct TrailingClosure {
        /// Optional label for the closure, supporting labeled trailing closures introduced in Swift 5.3.
        pub label: Option<String>,
        /// The closure expression.
        pub closure: Expression,
    }

    #[derive(Debug, Clone)]
    pub struct Argument {
        /// The label of the argument, if any.
        pub label: Option<String>,
        /// The argument value.
        pub value: Expression,
        /// True if this argument is part of a variadic parameter.
        pub is_variadic: bool,
        /// True if this argument is passed as inout.
        pub is_inout: bool,
    }


    #[derive(Debug, Clone)]
    pub struct Closure {
        pub parameters: Vec<ClosureParameter>, // Closure parameters, possibly with types.
        pub return_type: Option<Box<SwiftType>>, // Optional return type of the closure.
        pub is_escaping: bool, // True if the closure is marked with `@escaping`.
        pub body: Vec<super::Statement>, // The body of the closure as a sequence of statements.
    }

    #[derive(Debug, Clone)]
    pub struct ClosureParameter {
        /// The name of the parameter.
        pub name: String,
        /// Optional type annotation for the parameter.
        pub type_annotation: Option<SwiftType>,
    }


    /// Represents an assignment expression in Swift. Example: `a = b`
    #[derive(Debug, Clone)]
    pub struct AssignmentExpression {
        pub target: Box<Expression>,
        pub value: Box<Expression>,
    }

    /// Represents a subscript expression in Swift. Example: `array[0]`
    #[derive(Debug, Clone)]
    pub struct SubscriptExpression {
        pub target: Box<Expression>,
        pub index: Box<Expression>,
    }

    /// Represents a conditional expression in Swift. Example: `a > b ? a : b`
    #[derive(Debug, Clone)]
    pub struct InlineConditionalExpression {
        pub condition: Box<Expression>,
        pub true_expression: Box<Expression>,
        pub false_expression: Box<Expression>,
    }

    /// Represents a tuple expression in Swift. Example: `(1, "Hello")`
    #[derive(Debug, Clone)]
    pub struct TupleExpression {
        pub elements: Vec<Expression>,
    }

    /// Represents an array expression in Swift. Example: `[1, 2, 3]`
    #[derive(Debug, Clone)]
    pub struct ArrayExpression {
        pub elements: Vec<Expression>,
    }

    /// Represents a dictionary expression in Swift. Example: `["key": "value"]`
    #[derive(Debug, Clone)]
    pub struct DictionaryExpression {
        pub elements: Vec<(Expression, Expression)>,
    }

    /// Represents a member access expression in Swift. Example: `object.property`
    #[derive(Debug, Clone)]
    pub struct MemberAccessExpression {
        pub target: Box<Expression>,
        pub member: String,
    }

    /// Represents a type casting expression in Swift. Example: `object as? MyClass`
    #[derive(Debug, Clone)]
    pub struct TypeCastingExpression {
        pub expression: Box<Expression>,
        pub target_type: SwiftType,
    }

    /// Represents a pattern match expression in Swift. Example: `case .some(let x)`
    #[derive(Debug, Clone)]
    pub struct PatternMatchExpression {
        pub pattern: Box<Expression>,
        pub expression: Box<Expression>,
    }

    /// Represents a key path expression in Swift. Example: `\Person.name`
    #[derive(Debug, Clone)]
    pub struct KeyPathExpression {
        pub type_name: Option<String>,
        pub path: Vec<String>,
    }
}



/// A Swift statement.
#[derive(Debug, Clone)]
pub enum Statement {
    /// Corresponds to `break` statement in Swift.
    Break(statement::BreakStatement),
    /// Corresponds to `continue` statement in Swift.
    Continue(statement::ContinueStatement),
    /// Corresponds to expression statements in Swift.
    Expression(Box<Expression>),
    /// Corresponds to declaration statements in Swift.
    Declaration(Box<Declaration>),
    /// Corresponds to `return` statement in Swift.
    Return(Box<statement::ReturnStatement>),
    /// Corresponds to `if` statement in Swift.
    If(Box<statement::IfStatement>),
    /// Corresponds to `for` loop in Swift.
    ForLoop(Box<statement::ForLoopStatement>),
    /// Corresponds to `while` loop in Swift.
    WhileLoop(Box<statement::WhileLoopStatement>),
    /// Corresponds to `repeat-while` loop in Swift.
    RepeatWhileLoop(Box<statement::RepeatWhileLoopStatement>),
    /// Corresponds to `switch` statement in Swift.
    Switch(Box<statement::SwitchStatement>),
    /// Corresponds to `guard` statement in Swift.
    Guard(Box<statement::GuardStatement>),
    /// Corresponds to `throw` statement in Swift.
    Throw(Box<statement::ThrowStatement>),
    /// Corresponds to `do-catch` statement in Swift.
    DoCatch(Box<statement::DoCatchStatement>),
    /// Corresponds to assignment statement in Swift.
    Assignment(statement::AssignmentStatement),
}


/// Represents a sequence of Swift statements. 
#[derive(Debug, Clone)]
pub struct StatementSequence(Vec<Statement>);

pub mod statement {
    use super::{Expression, StatementSequence};

    /// Represents a `break` statement in Swift. Example: `break`
    #[derive(Debug, Clone)]
    pub struct BreakStatement {
        pub label: Option<String>
    }

    /// Represents a `continue` statement in Swift. Example: `continue`
    #[derive(Debug, Clone)]
    pub struct ContinueStatement {
        pub label: Option<String>
    }

    /// Represents a `return` statement in Swift. Example: `return a`
    #[derive(Debug, Clone)]
    pub struct ReturnStatement {
        pub expression: Option<Box<Expression>>,
    }

    /// Represents an `if` statement in Swift. Example: `if a > b { ... }`
    #[derive(Debug, Clone)]
    pub struct IfStatement {
        pub condition: Box<Expression>,
        pub body: StatementSequence,
        pub else_body: Option<StatementSequence>,
    }

    /// Represents a `for` loop in Swift. Example: `for i in 1...5 { ... }`
    #[derive(Debug, Clone)]
    pub struct ForLoopStatement {
        pub variable: String,
        pub range: (Box<Expression>, Box<Expression>),
        pub body: StatementSequence,
    }

    /// Represents a `while` loop in Swift. Example: `while a > b { ... }`
    #[derive(Debug, Clone)]
    pub struct WhileLoopStatement {
        pub condition: Box<Expression>,
        pub body: StatementSequence,
    }

    /// Represents a `repeat-while` loop in Swift. Example: `repeat { ... } while a > b`
    #[derive(Debug, Clone)]
    pub struct RepeatWhileLoopStatement {
        pub body: StatementSequence,
        pub condition: Box<Expression>,
    }

    /// Represents a `switch` statement in Swift.
    #[derive(Debug, Clone)]
    pub struct SwitchStatement {
        pub expression: Box<Expression>,
        pub cases: Vec<Case>,
        pub default_case: Option<StatementSequence>, // Optional; some `switch` statements might not have a `default` case.
    }

    /// Represents a case in a `switch` statement, which can include multiple patterns and an optional guard.
    #[derive(Debug, Clone)]
    pub struct Case {
        pub patterns: Vec<Pattern>, // A case can match against multiple patterns.
        pub guard_expression: Option<Box<Expression>>, // An optional guard condition for the case.
        pub body: StatementSequence,
    }

    /// Represents a pattern in a `switch` case. This is a simplified representation.
    #[derive(Debug, Clone)]
    pub enum Pattern {
        Literal(LiteralPattern),
        Identifier(super::expression::Identifier),
        Tuple(TuplePattern),
        EnumCase(EnumCasePattern),
        Wildcard,
        /// Using the `is` syntax.
        TypePattern(TypePattern),
        // Additional patterns can be added here as needed.
    }

    #[derive(Debug, Clone)]
    pub struct TypePattern {
        // Represents the type to match against.
        pub ty: super::SwiftType,
    }

    #[derive(Debug, Clone)]
    pub struct LiteralPattern {
        // Using the `Literal` variant of `Expression`.
        pub value: super::expression::Literal,
    }

    #[derive(Debug, Clone)]
    pub struct TuplePattern {
        // Each element in the tuple can be a pattern.
        pub elements: Vec<Pattern>,
    }

    #[derive(Debug, Clone)]
    pub struct EnumCasePattern {
        // Name of the enum.
        pub enum_name: Option<String>,
        // Specific case of the enum to match.
        pub case_name: String,
        // Patterns for associated values, if any.
        pub associated_values: Vec<Pattern>,
    }

    /// Represents a `guard` statement in Swift. Example: `guard let a = optional else { return }`
    #[derive(Debug, Clone)]
    pub struct GuardStatement {
        pub condition: Box<Expression>,
        pub body: StatementSequence,
    }

    /// Represents a `throw` statement in Swift. Example: `throw MyError()`
    #[derive(Debug, Clone)]
    pub struct ThrowStatement {
        pub expression: Box<Expression>,
    }

    /// Represents a `do-catch` statement in Swift. Example: `do { try function() } catch { ... }`
    #[derive(Debug, Clone)]
    pub struct DoCatchStatement {
        pub body: StatementSequence,
        pub catch_body: Vec<super::Statement>,
    }

    /// Represents an assignment statement in Swift. Example: `a = b`
    #[derive(Debug, Clone)]
    pub struct AssignmentStatement {
        /// Typically an Identifier or a MemberAccess expression
        pub target: Box<Expression>,
        pub value: Box<Expression>,
    }
}


/// A Swift type.
#[derive(Debug, Clone)]
pub enum SwiftType {
    /// Represents an integer type in Swift. Example: `Int`
    Integer,
    /// Represents a floating point type in Swift. Example: `Double`
    Float,
    /// Represents a boolean type in Swift. Example: `Bool`
    Bool,
    /// Represents a string type in Swift. Example: `String`
    String,
    /// Represents a character type in Swift. Example: `Character`
    Character,
    /// Represents an optional type in Swift. Example: `Int?`
    Optional(Box<SwiftType>),
    /// Represents an array type in Swift. Example: `[Int]`
    Array(Box<SwiftType>),
    /// Represents a dictionary type in Swift. Example: `[String: Int]`
    Dictionary(Box<SwiftType>, Box<SwiftType>),
    /// Represents a tuple type in Swift. Example: `(Int, String)`
    Tuple(Vec<SwiftType>),
    /// Represents a function type in Swift. Example: `(Int, String) -> Bool`
    Function(Vec<SwiftType>, Box<SwiftType>),
    /// Represents a custom type in Swift. Example: `MyClass`
    Custom(String),
}

/// A Swift declaration.
#[derive(Debug, Clone)]
pub enum Declaration {
    /// Corresponds to function declarations in Swift. Example: `func myFunction() { ... }`
    Function(Box<declaration::FunDeclaration>),
    /// Corresponds to variable declarations in Swift. Example: `var a = 5`
    Var(Box<declaration::VarDeclaration>),
    /// Corresponds to constant declarations in Swift. Example: `let a = 5`
    Let(Box<declaration::LetDeclaration>),
    /// Corresponds to struct declarations in Swift. Example: `struct MyStruct { ... }`
    Struct(Box<declaration::StructDeclaration>),
    /// Corresponds to enum declarations in Swift. Example: `enum MyEnum { case a, b, c }`
    Enum(Box<declaration::EnumDeclaration>),
    /// Corresponds to class declarations in Swift. Example: `class MyClass { ... }`
    Class(Box<declaration::ClassDeclaration>),
    /// Corresponds to protocol declarations in Swift. Example: `protocol MyProtocol { ... }`
    Protocol(Box<declaration::ProtocolDeclaration>),
    /// Corresponds to extension declarations in Swift. Example: `extension MyClass { ... }`
    Extension(Box<declaration::ExtensionDeclaration>),
    /// Corresponds to type alias declarations in Swift. Example: `typealias MyType = Int`
    TypeAlias(Box<declaration::TypeAliasDeclaration>),
    /// Corresponds to import declarations in Swift. Example: `import Foundation`
    Import(Box<declaration::ImportDeclaration>),
    /// Corresponds to initializer declarations in Swift. Example: `init() { ... }`
    Initializer(Box<declaration::InitializerDeclaration>),
    /// Corresponds to deinitializer declarations in Swift. Example: `deinit { ... }`
    Deinitializer(Box<declaration::DeinitializerDeclaration>),
}

pub mod declaration {
    use super::{Expression, SwiftType, StatementSequence};

    /// Represents a Swift function, including support for generics, different types of parameters, access control, and more.
    /// 
    /// # Examples
    ///
    /// Swift:
    /// ```swift
    /// public func greet(person: String, loudly: Bool = false) -> String {
    ///     let greeting = "Hello, \(person)!"
    ///     return loudly ? greeting.uppercased() : greeting
    /// }
    /// ```
    #[derive(Debug, Clone)]
    pub struct FunDeclaration {
        pub name: String,
        pub generics: Option<GenericsDeclaration>,
        pub parameters: Vec<FunctionParameter>,
        pub return_type: Option<SwiftType>,
        pub is_throwing: bool, // True if the function can throw an error, false otherwise.
        pub access_control: AccessControl, // The access level of the function.
        pub body: Option<StatementSequence>, // Optional body; for protocol method requirements, this may be None.
    }



    /// Represents generic type parameters and their constraints in a function.
    /// Swift code example:
    /// ```
    /// func add<T: Numeric>(a: T, b: T) -> T { ... }
    /// ```
    #[derive(Debug, Clone)]
    pub struct GenericsDeclaration {
        pub type_parameters: Vec<TypeParameter>,
    }

    /// Represents a single generic type parameter and its optional constraint.
    /// For instance, `T: Numeric` in `func add<T: Numeric>(a: T, b: T) -> T { ... }`
    #[derive(Debug, Clone)]
    pub struct TypeParameter {
        pub name: String,
        pub constraint: Option<SwiftType>, // Simplified; real-world might require a more complex representation.
    }

    /// Represents a function parameter in Swift, including support for labels, default values, and variadic parameters.
    #[derive(Debug, Clone)]
    pub struct FunctionParameter {
        /// // External parameter name, if any.
        pub label: Option<String>,
        /// Internal parameter name.
        pub internal_name: String,
        /// The type of the parameter.
        pub ty: SwiftType,
        /// The default value of the parameter, represented as a String.
        pub default_value: Option<String>,
        /// True if the parameter is variadic, false otherwise.
        pub is_variadic: bool,
        /// True if the parameter is an inout parameter, allowing modification of passed argument.
        pub is_inout: bool,
    }


    
    /// Represents a struct declaration in Swift, including support for generics, protocol conformance, and initializers.
    /// Swift code example:
    /// ```
    /// struct Point<T: Numeric>: CustomStringConvertible {
    ///     var x: T, y: T
    ///
    ///     var description: String {
    ///         return "(\(x), \(y))"
    ///     }
    ///
    ///     init(x: T, y: T) {
    ///         self.x = x
    ///         self.y = y
    ///     }
    /// }
    /// ```
    #[derive(Debug, Clone)]
    pub struct StructDeclaration {
        pub name: String,
        pub generics: Option<GenericsDeclaration>,
        /// Protocol names to which the struct conforms.
        pub conformances: Vec<String>,
        /// Includes variables and constants.
        pub properties: Vec<VariablePropertyDeclaration>,
        /// Includes functions and computed properties.
        pub methods: Vec<FunDeclaration>,
        /// Initializers.
        pub initializers: Vec<InitializerDeclaration>,
    }
    
    /// Represents an enum declaration in Swift, including associated values, generics, and raw values.
    /// Swift code example:
    /// ```
    /// enum Barcode {
    ///     case upc(Int, Int, Int, Int)
    ///     case qrCode(String)
    /// }
    ///
    /// enum Planet: Int {
    ///     case mercury = 1, venus, earth, mars, jupiter, saturn, uranus, neptune
    /// }
    ///
    /// enum OptionalValue<Wrapped> {
    ///     case none
    ///     case some(Wrapped)
    /// }
    /// ```
    #[derive(Debug, Clone)]
    pub struct EnumDeclaration {
        pub name: String,
        pub generics: Option<GenericsDeclaration>,
        pub cases: Vec<EnumCase>,
        /// // For enums with raw values
        pub raw_type: Option<SwiftType>,
    }

    /// Represents a single case in an enum. Enum cases in Swift can have associated values.
    #[derive(Debug, Clone)]
    pub struct EnumCase {
        pub name: String,
        /// Empty for cases without associated values
        pub associated_values: Vec<EnumAssociatedValue>,
        /// For enums with raw values, otherwise None
        pub raw_value: Option<Expression>,
    }

    /// Represents an associated value for an enum case, potentially with a label.
    #[derive(Debug, Clone)]
    pub struct EnumAssociatedValue {
        /// Optional label for the associated value
        pub label: Option<String>,
        /// The type of the associated value
        pub ty: SwiftType,
    }

    
    /// Represents a class declaration in Swift, including generics, inheritance, protocol conformance, initializers, and deinitializers.
    /// Swift code example:
    /// ```
    /// class Vehicle {
    ///     var numberOfWheels: Int
    ///     var description: String {
    ///         return "\(numberOfWheels) wheel(s)"
    ///     }
    ///     init() {
    ///         numberOfWheels = 0
    ///     }
    /// }
    ///
    /// class Bicycle: Vehicle {
    ///     override init() {
    ///         super.init()
    ///         numberOfWheels = 2
    ///     }
    /// }
    ///
    /// class Hoverboard: Vehicle, CustomStringConvertible {
    ///     var color: String
    ///     override var description: String {
    ///         return "\(super.description) in the color \(color)"
    ///     }
    ///     init(color: String) {
    ///         self.color = color
    ///         super.init()
    ///         numberOfWheels = 0
    ///     }
    /// }
    /// ```
    #[derive(Debug, Clone)]
    pub struct ClassDeclaration {
        pub name: String,
        pub generics: Option<GenericsDeclaration>,
        /// Optional superclass name for inheritance.
        pub superclass: Option<String>,
        /// Protocols the class conforms to.
        pub conformances: Vec<String>,
        /// Includes variables and constants.
        pub properties: Vec<VariablePropertyDeclaration>,
        /// Includes functions, computed properties, and overrides.
        pub methods: Vec<FunDeclaration>,
        /// Initializers.
        pub initializers: Vec<InitializerDeclaration>,
        /// Optional deinitializer.
        pub deinitializer: Option<DeinitializerDeclaration>,
    }

    
    /// Represents a protocol declaration in Swift, including method requirements, property requirements, initializer requirements, and protocol inheritance.
    /// Swift code example:
    /// ```
    /// protocol FullyNamed {
    ///     var fullName: String { get }
    /// }
    ///
    /// protocol RandomNumberGenerator {
    ///     func random() -> Double
    /// }
    ///
    /// protocol Named {
    ///     init(name: String)
    /// }
    ///
    /// protocol InheritingProtocol: SomeProtocol, AnotherProtocol {
    ///     // protocol definition goes here
    /// }
    /// ```
    #[derive(Debug, Clone)]
    pub struct ProtocolDeclaration {
        pub name: String,
        /// Names of inherited protocols
        pub inherited_protocols: Vec<String>,
        pub property_requirements: Vec<PropertyRequirement>,
        pub method_requirements: Vec<MethodRequirement>,
        pub initializer_requirements: Vec<InitializerRequirement>,
    }

    /// Represents a property requirement in a Swift protocol.
    #[derive(Debug, Clone)]
    pub struct PropertyRequirement {
        pub name: String,
        pub ty: SwiftType,
        /// True if the property is read-only (`get`), false if read-write (`get` and `set`).
        pub is_read_only: bool,
    }

    /// Represents a method requirement in a Swift protocol.
    #[derive(Debug, Clone)]
    pub struct MethodRequirement {
        pub name: String,
        pub parameters: Vec<FunctionParameter>,
        pub return_type: Option<SwiftType>,
        /// True for mutating methods in value types.
        pub is_mutating: bool,
    }

    /// Represents an initializer requirement in a Swift protocol.
    #[derive(Debug, Clone)]
    pub struct InitializerRequirement {
        pub parameters: Vec<FunctionParameter>,
    }
    
    /// Represents an extension of a Swift class, struct, enum, or protocol,
    /// including the addition of computed properties, methods, initializers,
    /// and protocol conformances.
    /// 
    /// Swift code example:
    /// ```
    /// extension Double {
    ///     var km: Double { return self * 1_000.0 }
    ///     var m: Double { return self }
    ///     var cm: Double { return self / 100.0 }
    ///     var mm: Double { return self / 1_000.0 }
    ///     var ft: Double { return self / 3.28084 }
    ///
    ///     func roundToPlaces(places: Int) -> Double {
    ///         let divisor = pow(10.0, Double(places))
    ///         return (self * divisor).rounded() / divisor
    ///     }
    /// }
    ///
    /// extension SomeType: SomeProtocol, AnotherProtocol {
    ///     // implementation of protocol requirements goes here
    /// }
    /// ```
    #[derive(Debug, Clone)]
    pub struct ExtensionDeclaration {
        /// The type being extended
        pub type_name: String,
        /// Protocols the extension conforms to
        pub conformances: Vec<String>,
        /// Computed properties added by the extension
        pub properties: Vec<VarDeclaration>,
        /// Methods added by the extension
        pub methods: Vec<FunDeclaration>,
        /// Initializers added by the extension
        pub initializers: Vec<InitializerDeclaration>,
    }

    
    /// Represents a type alias declaration in Swift.
    /// A type alias allows you to provide a new name for an existing type.
    /// Swift code example:
    /// ```
    /// typealias Point = (Int, Int)
    /// typealias CompletionHandler = (Result<String, Error>) -> Void
    /// ```
    #[derive(Debug, Clone)]
    pub struct TypeAliasDeclaration {
        /// The new name for the type.
        pub name: String,
        /// The existing type that is being aliased.
        pub target: SwiftType,
    }
    
    /// Represents different kinds of symbols that can be imported from a module.
    #[derive(Debug, Clone)]
    pub enum ImportSymbol {
        EntireModule,
        Class(String),
        Struct(String),
        Enum(String),
        Protocol(String),
        Function(String),
        Variable(String),
        // Extend this as needed to cover other symbol types.
    }

    /// Represents an import declaration in Swift, capable of importing specific symbols or entire modules.
    #[derive(Debug, Clone)]
    pub struct ImportDeclaration {
        pub module: String,
        pub symbol: ImportSymbol,
    }

    
    /// Represents a constant declaration in Swift. Example: `let a: Int = 5`
    #[derive(Debug, Clone)]
    pub struct LetDeclaration {
        pub name: String,
        pub ty: Option<SwiftType>,
        pub initial_value: Option<Expression>,
    }

    /// Represents a variable declaration in Swift. Example: `var a: Int = 5`
    #[derive(Debug, Clone)]
    pub struct VarDeclaration {
        pub name: String,
        pub ty: Option<SwiftType>,
        pub initial_value: Option<Expression>,
    }

    /// Represents a variable declaration, specifically for computed properties in this context.
    /// Computed properties in extensions can't store a value; they must provide a getter and optionally a setter.
    #[derive(Debug, Clone)]
    pub struct VariablePropertyDeclaration {
        pub name: String,
        pub ty: SwiftType,
        // The getter function for the computed property
        pub getter: FunDeclaration,
        // The setter function for the computed property, if any
        pub setter: Option<FunDeclaration>,
    }

    /// Represents an initializer in Swift, including support for parameters, generics, and access control.
    #[derive(Debug, Clone)]
    pub struct InitializerDeclaration {
        pub generics: Option<GenericsDeclaration>,
        pub parameters: Vec<FunctionParameter>,
        pub body: StatementSequence,
        // True for failable initializers (`init?`), false otherwise.
        pub is_failable: bool,
        // True for convenience initializers, false for designated initializers.
        pub is_convenience: bool,
        // The access level of the initializer.
        pub access_control: AccessControl,
    }


    /// Represents a deinitializer declaration in Swift. Example: `deinit { print("Deinitialized") }`
    #[derive(Debug, Clone)]
    pub struct DeinitializerDeclaration {
        pub body: StatementSequence,
    }

    /// Represents the accessibility level of the initializer, corresponding to Swift's access control keywords.
    #[derive(Debug, Clone, PartialEq)]
    pub enum AccessControl {
        Public,
        Internal,
        FilePrivate,
        Private,
    }
}
