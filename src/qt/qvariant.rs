pub enum Type {
    Invalid = 0,
    Bool = 1,
    Int = 2,
    UInt = 3,
    LongLong = 4,
    ULongLong = 5,
    Double = 6,
    Char = 7,
    VariantMap = 8,
    VariantList = 9,
    String = 10,
    StringList = 11,
    ByteArray = 12,
    BitArray = 13,
    Date = 14,
    Time = 15,
    DateTime = 16,
    Url = 17,
    Locale = 18,
    Rect = 19,
    RectF = 20,
    Size = 21,
    SizeF = 22,
    Line = 23,
    LineF = 24,
    Point = 25,
    PointF = 26,
    RegExp = 27,
    VariantHash = 28,
    EasingCurve = 29,
    LastCoreType = 30,

    FirstGuiType = 63,
    Font = 64,
    Pixmap = 65,
    Brush = 66,
    Color = 67,
    Palette = 68,
    Icon = 69,
    Image = 70,
    Polygon = 71,
    Region = 72,
    Bitmap = 73,
    Cursor = 74,
    SizePolicy = 75,
    KeySequence = 76,
    Pen = 77,
    TextLength = 78,
    TextFormat = 79,
    Matrix = 80,
    Transform = 81,
    Matrix4x4 = 82,
    Vector2D = 83,
    Vector3D = 84,
    Vector4D = 85,
    Quaternion = 86,

    UserType = 127,
    UShort = 133,
    LastType = 0xffffffff
}

pub struct qVariant<T> {
    qType: Type,
    data: T
}

trait qVariantSerial {
    fn new<T>(data: T, qType: Type) -> qVariant;
}

impl qVariantSerial for bool {
    fn new<T>(data: bool, qType: Type) -> qVariant {
        qVariant {data: data, qType: Type}
    }
}

impl qVariantSerial for String {
    fn new<T>(data: String, qType: Type) -> qVariant {
        qVariant {data: data, qType: Type}
    }
}

impl qVariantSerial for int {
    fn new<T>(data: int, qType: Type) -> qVariant {
        qVariant {data: data, qType: Type}
    }
}
