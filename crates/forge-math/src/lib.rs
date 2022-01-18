use num::Num;

pub fn round_up<T: Num + Copy>(value: T, multiple: T) -> T {
    return ((value + multiple - T::one()) / multiple) * multiple;
}

pub fn round_down<T: Num + Copy>(value: T, multiple: T) -> T {
    return value - (value % multiple);
}
