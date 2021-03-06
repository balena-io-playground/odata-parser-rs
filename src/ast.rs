//FIXME
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::rc::Rc;
use std::str::FromStr;

use super::schema;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ODataURI<'a> {
    pub service_root: &'a str,
    pub relative_uri: Option<RelativeURI<'a>>,
}

#[derive(Debug, Clone)]
pub struct ServiceRoot<'a> {
    pub data: &'a str,
}

#[derive(Debug, Clone)]
pub enum RelativeURI<'a> {
    Batch(Option<Vec<QueryOption<'a>>>),
    Entity,
    Metadata,
    Resource(ResourceQuery<'a>),
}

// pub struct Output<'a>(OutputCardinality, OutputKind<'a>);
//
// mod output {
// 	pub struct EntityCollection {
// 		select: Option<Select>,
// 		filter: Option<Filter>,
// 		expand: Option<Expand>,
// 	}
// }
//
// pub enum OutputCardinality {
// 	Collection,
// 	Single
// }
//
// pub enum OutputKind<'a> {
// 	Entity(&'a schema::ty::Entity),
// 	Complex(&'a schema::ty::Complex),
// 	Primitive(&'a schema::ty::Primitive),
// 	Collection(&'a OutputKind)
// }
//
// impl OutputKind {
// 	fn options(&self) -> impl Parser {
// 		match self {
// 			Self::Entity(_) => alt(
//
// 		}
// 	}
// }

// pub enum CollectionNavigationOption {
// 	BoundOperation,                               // -> ()
// 	Count,                                        // -> ()
// 	Each,                                         // -> boundOperation
// 	Filter(ParameterAlias<'a>),                   // -> collectionNavigation
// 	KeyPredicate(KeyPredicate<'a>),               // -> singleNavigation
// }
//
// struct EntitySet {
// 	col_nav: {
// 		filter: Option<Alias>,
// 		enum {
// 			Key,
// 			Each,
// 			Count,
// 			Ref,
// 		}
// 	}
// }
//
// struct Singleton {
//
// }
//
// pub enum SingleNavigationOption {
// 	Property(&'a schema::property::Property),     // -> single,collection,complexCollection,complex,primitiveCollection,primitive,boundOperation
// 	Ref,                                          // -> ()
// 	Value,
// }

#[derive(Debug, Clone)]
pub enum PathSegment<'a> {
    EntitySet(&'a schema::EntitySet<'a>), // -> collectionNavigation
    Singleton,                            // -> singleNavigation
    Action,                               // -> ()
    Function, // -> collection,single,complexCollection,complex,primitiveCollection,primitive,() navigation
    Crossjoin, // -> ()
    All,      // -> ()
    Cast,
    BoundOperation,                               // -> ()
    Count,                                        // -> ()
    Each,                                         // -> boundOperation
    Filter(ParameterAlias<'a>),                   // -> collectionNavigation
    KeyPredicate(KeyPredicate<'a>),               // -> singleNavigation
    Property(&'a schema::property::Property<'a>), // -> single,collection,complexCollection,complex,primitiveCollection,primitive,boundOperation
    Ref,                                          // -> ()
    Value,
    OrdinalIndex(i64),
}

#[derive(Debug, Clone)]
pub struct KeyPredicate<'a> {
    pub values: Vec<(KeyProperty<'a>)>,
}

#[derive(Debug, Clone)]
pub struct KeyProperty<'a> {
    pub property: &'a schema::property::Property<'a>,
    pub value: KeyValue<'a>,
}

#[derive(Debug, Clone)]
pub enum KeyValue<'a> {
    ParameterAlias(ParameterAlias<'a>),
    Value(&'a str),
}

#[derive(Debug, Clone)]
pub struct PrimitiveValue<'a> {
    kind: schema::ty::Primitive,
    value: &'a str,
}

#[derive(Debug, Clone)]
pub struct ParameterAlias<'a> {
    pub name: &'a str,
}

#[derive(Debug, Clone)]
pub enum FormatKind<'a> {
    JSON,
    Atom,
    XML,
    Custom(&'a str),
}

#[derive(Debug, Clone)]
pub enum QueryOption<'a> {
    Compute,
    DeltaToken,
    Expand,
    Filter(Rc<Expr<'a>>),
    Id,
    InlineCount,
    OrderBy,
    SchemaVersion,
    Search,
    Select,
    Skip,
    SkipToken,
    Top,
    Index,
    Alias,
    Name,
    Format(FormatKind<'a>),
    Custom(&'a str),
}

#[derive(Debug, Clone, Copy)]
pub enum UnOp {
    /// The `not` operator for logical inversion
    Not,
    /// The `-` operator for negation
    Neg,
}

impl UnOp {
    pub fn precedence(&self) -> u8 {
        match self {
            UnOp::Not => 6,
            UnOp::Neg => 6,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    /// The `add` operator (addition)
    Add,
    /// The `sub` operator (subtraction)
    Sub,
    /// The `mul` operator (multiplication)
    Mul,
    /// The `div` operator (division)
    Div,
    /// The `divby` operator (decimal division)
    DivBy,
    /// The `mod` operator (modulus)
    Mod,
    /// The `and` operator (logical and)
    And,
    /// The `or` operator (logical or)
    Or,
    /// The `eq` operator (equality)
    Eq,
    /// The `lt` operator (less than)
    Lt,
    /// The `le` operator (less than or equal to)
    Le,
    /// The `ne` operator (not equal to)
    Ne,
    /// The `ge` operator (greater than or equal to)
    Ge,
    /// The `gt` operator (greater than)
    Gt,
    /// The `has` operator (enumeration flags)
    Has,
    /// The `in` operator (is member of)
    In,
}

impl BinOp {
    pub fn precedence(&self) -> u8 {
        match self {
            BinOp::Or => 0,
            BinOp::And => 1,
            BinOp::Eq => 2,
            BinOp::Ne => 2,
            BinOp::Lt => 3,
            BinOp::Le => 3,
            BinOp::Ge => 3,
            BinOp::Gt => 3,
            BinOp::Add => 4,
            BinOp::Sub => 4,
            BinOp::Mul => 5,
            BinOp::Div => 5,
            BinOp::DivBy => 5,
            BinOp::Mod => 5,
            BinOp::Has => 7,
            BinOp::In => 7,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IntTy {
    U8,
    I8,
    I16,
    I32,
    I64,
}

#[derive(Debug, Clone, Copy)]
pub enum FloatTy {
    F32,
    F64,
}

#[derive(Debug, Clone)]
pub enum Lit {
    Null,
    Binary(String),
    Boolean(bool),
    Date(i16, u8, u8),
    DateTimeOffset(i16, u8, u8, u8, u8),
    Decimal,
    Float(f64, FloatTy),
    Duration,
    Enum,
    Guid(Uuid),
    Int(i64, IntTy),
    Str(String),
    TimeOfDay,
    GeographyPoint,
    GeographyLineString,
    GeographyPolygon,
    GeographyMultiPoint,
    GeographyMultiLineString,
    GeographyMultiPolygon,
    GeographyCollection,
    GeometryPoint,
    GeometryLineString,
    GeometryPolygon,
    GeometryMultiPoint,
    GeometryMultiLineString,
    GeometryMultiPolygon,
    GeometryCollection,
    Unimplemented,
}

#[derive(Debug, Clone)]
pub struct Filter<'a> {
    expr: Expr<'a>,
}

pub type NodeId = u32;

#[derive(Clone)]
pub struct Expr<'a> {
    pub id: NodeId,
    // The AST node of the expression
    pub node: ExprKind<'a>,
    pub ty: Ty<'a>,
}

impl std::fmt::Debug for Expr<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.debug_struct("Expr")
            .field("id", &self.id)
            .field("node", &self.node)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub enum ExprKind<'a> {
    Call(Rc<Expr<'a>>, Vec<Rc<Expr<'a>>>),
    Lit(Lit),
    Binary(BinOp, Rc<Expr<'a>>, Rc<Expr<'a>>),
    Unary(UnOp, Rc<Expr<'a>>),
    List(Vec<Rc<Expr<'a>>>),
    Cast(&'a schema::ty::Entity<'a>, Rc<Expr<'a>>),
    // IsOf,
    MethodCall(Method, Vec<Rc<Expr<'a>>>),
    // EntitySet(&'a schema::EntitySet, Rc<Options>),
    Filter(Rc<Expr<'a>>, Rc<Expr<'a>>),
    // Member(Vec<PathSegment<'a>>),
    Root,
    // JSON,
    // Member,
    EntitySet(&'a schema::EntitySet<'a>),
    Var(NodeId, Ty<'a>),
    Placeholder,
    Unimplemented,
    // stuff from path segment
    Singleton,
    Action(&'a schema::Action<'a>, Vec<Rc<Expr<'a>>>),
    Function(&'a schema::Function<'a>, Vec<Rc<Expr<'a>>>),
    Crossjoin,
    All,
    Count(Rc<Expr<'a>>),
    Each(Rc<Expr<'a>>),
    Key(Rc<Expr<'a>>, Vec<Rc<Expr<'a>>>),
    Property(&'a schema::property::Property<'a>),
    Ref(Rc<Expr<'a>>),
    Value,
    OrdinalIndex(Rc<Expr<'a>>, i64),
    Query(Rc<QueryExpr<'a>>),
}

impl<'a, 'b> ExprKind<'a> {
    pub fn to_ty(&'b self) -> Ty<'a> {
        match &self {
            ExprKind::Call(a, b) => unimplemented!(),
            ExprKind::Lit(l) => {
                match l {
                    Lit::Null => ty::Ty::Null,
                    Lit::Binary(_) => ty::Ty::Primitive(schema::ty::Primitive::Binary),
                    Lit::Boolean(_) => ty::Ty::Primitive(schema::ty::Primitive::Boolean),
                    Lit::Date(_, _, _) => ty::Ty::Primitive(schema::ty::Primitive::Date),
                    Lit::DateTimeOffset(_, _, _, _, _) => ty::Ty::Primitive(schema::ty::Primitive::DateTimeOffset),
                    Lit::Decimal => ty::Ty::Primitive(schema::ty::Primitive::Decimal),
                    Lit::Duration => ty::Ty::Primitive(schema::ty::Primitive::Duration),
                    Lit::Guid(_) => ty::Ty::Primitive(schema::ty::Primitive::Guid),
                    Lit::Str(_) => ty::Ty::Primitive(schema::ty::Primitive::String),
                    // FIXME create types for the rest of the literals
                    _ => ty::Ty::None,
                }
            },
            ExprKind::Binary(op, lhs, rhs) => {
                //FIXME the rules for binary operators are more complex than this simplification
                //http://docs.oasis-open.org/odata/odata/v4.01/cs01/part2-url-conventions/odata-v4.01-cs01-part2-url-conventions.html#sec_LogicalOperators
                lhs.node.to_ty()
            },
            ExprKind::Unary(op, arg) => unimplemented!(),
            ExprKind::List(xs) => unimplemented!(),
            ExprKind::Cast(ty, expr) => {
                //FIXME
                ty::Ty::Entity(ty)
                // match expr.ty {
                //     ty::Ty::Entity(ty)
                // }
            },
            ExprKind::MethodCall(method, args) => unimplemented!(),
            ExprKind::Filter(collection, predicate) => collection.ty,
            ExprKind::Root => unimplemented!(),
            ExprKind::EntitySet(entity_set) => ty::Collection::Entity(&entity_set.ty).into(),
            ExprKind::Var(_, ty) => *ty,
            ExprKind::Placeholder => unimplemented!(),
            ExprKind::Unimplemented => ty::Ty::None,
            ExprKind::Singleton => unimplemented!(),
            ExprKind::Action(_, _) => unimplemented!(),
            ExprKind::Function(_, _) => unimplemented!(),
            ExprKind::Crossjoin => unimplemented!(),
            ExprKind::All => unimplemented!(),
            ExprKind::Count(_) => unimplemented!(),
            ExprKind::Each(collection) => unimplemented!(),
            ExprKind::Key(collection, keys) => {
                match collection.ty {
                    ty::Ty::Collection(t) => t.into(),
                    _ => panic!(),
                }
            },
            ExprKind::Property(_) => unimplemented!(),
            ExprKind::Ref(_) => unimplemented!(),
            ExprKind::Value => unimplemented!(),
            ExprKind::OrdinalIndex(collection, _) => unimplemented!(),
            ExprKind::Query(q) => q.resource.node.to_ty(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct QueryExpr<'a> {
    pub resource: Rc<Expr<'a>>,
    pub select: Vec<Rc<QueryExpr<'a>>>,
    pub expand: Vec<Rc<QueryExpr<'a>>>,
    pub filter: Option<Rc<Expr<'a>>>,
    pub search: Option<Rc<Expr<'a>>>,
    pub orderby: Vec<Rc<Expr<'a>>>,
    pub skip: Option<u32>,
    pub top: Option<u32>,
    pub count: bool,
    pub levels: Option<u32>,
    pub compute: Vec<Rc<Expr<'a>>>,
    pub params: Vec<Rc<Expr<'a>>>,
}

impl<'a> QueryExpr<'a> {
    pub fn new(expr: Rc<Expr<'a>>) -> Self {
        Self {
            resource: expr.clone(),
            select: vec![],
            expand: vec![],
            filter: None,
            search: None,
            orderby: vec![],
            skip: None,
            top: None,
            count: false,
            levels: None,
            compute: vec![],
            params: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResourceQuery<'a> {
    pub expr: QueryExpr<'a>,
    pub delta_token: Option<&'a str>,
    pub format: Option<&'a str>,
    pub id: Option<&'a str>,
    pub schema_version: Option<&'a str>,
    pub skip_token: Option<&'a str>,
    pub index: Option<u32>,
}

impl<'a> ResourceQuery<'a> {
    pub fn new(expr: Rc<Expr<'a>>) -> Self {
        Self {
            expr: QueryExpr::new(expr),
            id: None,
            delta_token: None,
            format: None,
            schema_version: None,
            skip_token: None,
            index: None,
        }
    }
}

//#[derive(Debug,Clone)]
//pub enum Ty<'a> {
//	None,
//	Primitive(schema::ty::Primitive),
//	Enumeration(&'a schema::ty::Enumeration),
//	Complex(&'a schema::ty::Complex),
//	Entity(&'a schema::ty::Entity),
//	Function,
//	Collection(Rc<Ty<'a>>),
//}

pub use ty::Ty;

pub mod ty {
    use super::schema;

    #[derive(Debug, Copy, Clone)]
    pub enum Ty<'a> {
        None,
        Null,
        Primitive(Primitive),
        Enumeration(Enumeration<'a>),
        Complex(Complex<'a>),
        Entity(Entity<'a>),
        Function,
        Collection(Collection<'a>),
    }

    pub type Primitive = schema::ty::Primitive;
    pub type Enumeration<'a> = &'a schema::ty::Enum<'a>;
    pub type Complex<'a> = &'a schema::ty::Complex<'a>;
    pub type Entity<'a> = &'a schema::ty::Entity<'a>;

    #[derive(Debug, Copy, Clone)]
    pub enum Collection<'a> {
        Primitive(Primitive),
        Enumeration(Enumeration<'a>),
        Complex(Complex<'a>),
        Entity(Entity<'a>),
    }

    impl<'a> From<Collection<'a>> for Ty<'a> {
        fn from(ty: Collection<'a>) -> Ty<'a> {
            Ty::Collection(ty)
        }
    }
    impl<'a> From<Entity<'a>> for Ty<'a> {
        fn from(ty: Entity<'a>) -> Ty<'a> {
            Ty::Entity(ty)
        }
    }
    impl<'a> From<Complex<'a>> for Ty<'a> {
        fn from(ty: Complex<'a>) -> Ty<'a> {
            Ty::Complex(ty)
        }
    }
    impl<'a> From<Enumeration<'a>> for Ty<'a> {
        fn from(ty: Enumeration<'a>) -> Ty<'a> {
            Ty::Enumeration(ty)
        }
    }
    impl<'a> From<Primitive> for Ty<'a> {
        fn from(ty: Primitive) -> Ty<'a> {
            Ty::Primitive(ty)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Method {
    Substring,
    Concat,
    Contains,
    EndsWith,
    IndexOf,
    StartsWith,
    GeoDistance,
    GeoIntersects,
    HasSubset,
    HasSubsequence,
    Length,
    ToLower,
    ToUpper,
    Trim,
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    FractionalSeconds,
    TotalSeconds,
    Date,
    Time,
    TotalOffsetMinutes,
    Round,
    Floor,
    Ceiling,
    GeoLength,
    MinDatetime,
    MaxDatetime,
    Now,
}

pub struct MethodParseError(());

impl Method {
    pub fn arity(&self) -> (usize, usize) {
        match self {
            Method::Substring => (2, 3),
            Method::Concat => (2, 2),
            Method::Contains => (2, 2),
            Method::EndsWith => (2, 2),
            Method::IndexOf => (2, 2),
            Method::StartsWith => (2, 2),
            Method::GeoDistance => (2, 2),
            Method::GeoIntersects => (2, 2),
            Method::HasSubset => (2, 2),
            Method::HasSubsequence => (2, 2),
            Method::Length => (1, 1),
            Method::ToLower => (1, 1),
            Method::ToUpper => (1, 1),
            Method::Trim => (1, 1),
            Method::Year => (1, 1),
            Method::Month => (1, 1),
            Method::Day => (1, 1),
            Method::Hour => (1, 1),
            Method::Minute => (1, 1),
            Method::Second => (1, 1),
            Method::FractionalSeconds => (1, 1),
            Method::TotalSeconds => (1, 1),
            Method::Date => (1, 1),
            Method::Time => (1, 1),
            Method::TotalOffsetMinutes => (1, 1),
            Method::Round => (1, 1),
            Method::Floor => (1, 1),
            Method::Ceiling => (1, 1),
            Method::GeoLength => (1, 1),
            Method::MinDatetime => (0, 0),
            Method::MaxDatetime => (0, 0),
            Method::Now => (0, 0),
        }
    }
}

impl FromStr for Method {
    type Err = MethodParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "substring" => Ok(Method::Substring),
            "concat" => Ok(Method::Concat),
            "contains" => Ok(Method::Contains),
            "endswith" => Ok(Method::EndsWith),
            "indexof" => Ok(Method::IndexOf),
            "startswith" => Ok(Method::StartsWith),
            "geo.distance" => Ok(Method::GeoDistance),
            "geo.intersects" => Ok(Method::GeoIntersects),
            "hassubset" => Ok(Method::HasSubset),
            "hassubsequence" => Ok(Method::HasSubsequence),
            "length" => Ok(Method::Length),
            "tolower" => Ok(Method::ToLower),
            "toupper" => Ok(Method::ToUpper),
            "trim" => Ok(Method::Trim),
            "year" => Ok(Method::Year),
            "month" => Ok(Method::Month),
            "day" => Ok(Method::Day),
            "hour" => Ok(Method::Hour),
            "minute" => Ok(Method::Minute),
            "second" => Ok(Method::Second),
            "fractionalseconds" => Ok(Method::FractionalSeconds),
            "totalseconds" => Ok(Method::TotalSeconds),
            "date" => Ok(Method::Date),
            "time" => Ok(Method::Time),
            "totaloffsetminutes" => Ok(Method::TotalOffsetMinutes),
            "round" => Ok(Method::Round),
            "floor" => Ok(Method::Floor),
            "ceiling" => Ok(Method::Ceiling),
            "geo.length" => Ok(Method::GeoLength),
            "mindatetime" => Ok(Method::MinDatetime),
            "maxdatetime" => Ok(Method::MaxDatetime),
            "now" => Ok(Method::Now),
            _ => Err(MethodParseError(())),
        }
    }
}
