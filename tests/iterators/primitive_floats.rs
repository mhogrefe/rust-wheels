use common::{get_expected_test_outputs, TestOutput};
use malachite_base::misc::CheckedFrom;
use malachite_base::num::{PrimitiveFloat, PrimitiveUnsigned};
use malachite_nz::integer::Integer;
use rust_wheels::iterators::primitive_floats::*;

fn exhaustive_positive_finite_primitive_floats_helper<T: PrimitiveFloat>(eo: &TestOutput)
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    eo.match_vec(
        &format!("exhaustive_positive_finite_primitive_floats_{}", T::NAME),
        &mut exhaustive_positive_finite_primitive_floats::<T>(),
    );
}

#[test]
fn test_exhaustive_positive_finite_primitive_floats() {
    let eo = get_expected_test_outputs();
    exhaustive_positive_finite_primitive_floats_helper::<f32>(&eo);
    exhaustive_positive_finite_primitive_floats_helper::<f64>(&eo);
}

fn exhaustive_negative_finite_primitive_floats_helper<T: PrimitiveFloat>(eo: &TestOutput)
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    eo.match_vec(
        &format!("exhaustive_negative_finite_primitive_floats_{}", T::NAME),
        &mut exhaustive_negative_finite_primitive_floats::<T>(),
    );
}

#[test]
fn test_exhaustive_negative_finite_primitive_floats() {
    let eo = get_expected_test_outputs();
    exhaustive_negative_finite_primitive_floats_helper::<f32>(&eo);
    exhaustive_negative_finite_primitive_floats_helper::<f64>(&eo);
}

fn exhaustive_nonzero_finite_primitive_floats_helper<T: PrimitiveFloat>(eo: &TestOutput)
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    eo.match_vec(
        &format!("exhaustive_nonzero_finite_primitive_floats_{}", T::NAME),
        &mut exhaustive_nonzero_finite_primitive_floats::<T>(),
    );
}

#[test]
fn test_exhaustive_nonzero_finite_primitive_floats() {
    let eo = get_expected_test_outputs();
    exhaustive_nonzero_finite_primitive_floats_helper::<f32>(&eo);
    exhaustive_nonzero_finite_primitive_floats_helper::<f64>(&eo);
}

fn exhaustive_positive_primitive_floats_helper<T: PrimitiveFloat>(eo: &TestOutput)
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    eo.match_vec(
        &format!("exhaustive_positive_primitive_floats_{}", T::NAME),
        &mut exhaustive_positive_primitive_floats::<T>(),
    );
}

#[test]
fn test_exhaustive_positive_primitive_floats() {
    let eo = get_expected_test_outputs();
    exhaustive_positive_primitive_floats_helper::<f32>(&eo);
    exhaustive_positive_primitive_floats_helper::<f64>(&eo);
}

fn exhaustive_negative_primitive_floats_helper<T: PrimitiveFloat>(eo: &TestOutput)
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    eo.match_vec(
        &format!("exhaustive_negative_primitive_floats_{}", T::NAME),
        &mut exhaustive_negative_primitive_floats::<T>(),
    );
}

#[test]
fn test_exhaustive_negative_primitive_floats() {
    let eo = get_expected_test_outputs();
    exhaustive_negative_primitive_floats_helper::<f32>(&eo);
    exhaustive_negative_primitive_floats_helper::<f64>(&eo);
}

fn exhaustive_nonzero_primitive_floats_helper<T: PrimitiveFloat>(eo: &TestOutput)
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    eo.match_vec(
        &format!("exhaustive_nonzero_primitive_floats_{}", T::NAME),
        &mut exhaustive_nonzero_primitive_floats::<T>(),
    );
}

#[test]
fn test_exhaustive_nonzero_primitive_floats() {
    let eo = get_expected_test_outputs();
    exhaustive_nonzero_primitive_floats_helper::<f32>(&eo);
    exhaustive_nonzero_primitive_floats_helper::<f64>(&eo);
}

fn exhaustive_primitive_floats_helper<T: PrimitiveFloat>(eo: &TestOutput)
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    eo.match_vec(
        &format!("exhaustive_primitive_floats_{}", T::NAME),
        &mut exhaustive_primitive_floats::<T>(),
    );
}

#[test]
fn test_exhaustive_primitive_floats() {
    let eo = get_expected_test_outputs();
    exhaustive_primitive_floats_helper::<f32>(&eo);
    exhaustive_primitive_floats_helper::<f64>(&eo);
}
