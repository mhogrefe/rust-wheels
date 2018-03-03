use common::{get_expected_test_outputs, TestOutput};
use malachite_base::num::{Float, PrimitiveUnsigned};
use malachite_nz::integer::Integer;
use rust_wheels::iterators::floats::*;

fn exhaustive_positive_ordinary_floats_helper<T: 'static + Float>(eo: &TestOutput)
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: From<Integer>,
{
    eo.match_vec(
        &format!("exhaustive_positive_ordinary_floats_{}", T::NAME),
        &mut exhaustive_positive_ordinary_floats::<T>(),
    );
}

#[test]
fn test_exhaustive_positive_ordinary_floats() {
    let eo = get_expected_test_outputs();
    exhaustive_positive_ordinary_floats_helper::<f32>(&eo);
    exhaustive_positive_ordinary_floats_helper::<f64>(&eo);
}

fn exhaustive_negative_ordinary_floats_helper<T: 'static + Float>(eo: &TestOutput)
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: From<Integer>,
{
    eo.match_vec(
        &format!("exhaustive_negative_ordinary_floats_{}", T::NAME),
        &mut exhaustive_negative_ordinary_floats::<T>(),
    );
}

#[test]
fn test_exhaustive_negative_ordinary_floats() {
    let eo = get_expected_test_outputs();
    exhaustive_negative_ordinary_floats_helper::<f32>(&eo);
    exhaustive_negative_ordinary_floats_helper::<f64>(&eo);
}

fn exhaustive_nonzero_ordinary_floats_helper<T: 'static + Float>(eo: &TestOutput)
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: From<Integer>,
{
    eo.match_vec(
        &format!("exhaustive_nonzero_ordinary_floats_{}", T::NAME),
        &mut exhaustive_nonzero_ordinary_floats::<T>(),
    );
}

#[test]
fn test_exhaustive_nonzero_ordinary_floats() {
    let eo = get_expected_test_outputs();
    exhaustive_nonzero_ordinary_floats_helper::<f32>(&eo);
    exhaustive_nonzero_ordinary_floats_helper::<f64>(&eo);
}

fn exhaustive_positive_floats_helper<T: 'static + Float>(eo: &TestOutput)
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: From<Integer>,
{
    eo.match_vec(
        &format!("exhaustive_positive_floats_{}", T::NAME),
        &mut exhaustive_positive_floats::<T>(),
    );
}

#[test]
fn test_exhaustive_positive_floats() {
    let eo = get_expected_test_outputs();
    exhaustive_positive_floats_helper::<f32>(&eo);
    exhaustive_positive_floats_helper::<f64>(&eo);
}

fn exhaustive_negative_floats_helper<T: 'static + Float>(eo: &TestOutput)
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: From<Integer>,
{
    eo.match_vec(
        &format!("exhaustive_negative_floats_{}", T::NAME),
        &mut exhaustive_negative_floats::<T>(),
    );
}

#[test]
fn test_exhaustive_negative_floats() {
    let eo = get_expected_test_outputs();
    exhaustive_negative_floats_helper::<f32>(&eo);
    exhaustive_negative_floats_helper::<f64>(&eo);
}

fn exhaustive_nonzero_floats_helper<T: 'static + Float>(eo: &TestOutput)
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: From<Integer>,
{
    eo.match_vec(
        &format!("exhaustive_nonzero_floats_{}", T::NAME),
        &mut exhaustive_nonzero_floats::<T>(),
    );
}

#[test]
fn test_exhaustive_nonzero_floats() {
    let eo = get_expected_test_outputs();
    exhaustive_nonzero_floats_helper::<f32>(&eo);
    exhaustive_nonzero_floats_helper::<f64>(&eo);
}

fn exhaustive_floats_helper<T: 'static + Float>(eo: &TestOutput)
where
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    Integer: From<T::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: From<Integer>,
{
    eo.match_vec(
        &format!("exhaustive_floats_{}", T::NAME),
        &mut exhaustive_floats::<T>(),
    );
}

#[test]
fn test_exhaustive_floats() {
    let eo = get_expected_test_outputs();
    exhaustive_floats_helper::<f32>(&eo);
    exhaustive_floats_helper::<f64>(&eo);
}
