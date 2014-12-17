use qt::qvarianttypes::Type;

pub struct QVariant<T> {
    data: T,
    var_type: Type
}
