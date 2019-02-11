use test::{self, Bencher};
use cgmath::SquareMatrix;

#[path="../macros.rs"]
mod macros;


/*
 *
 * nalgebra
 *
 */
bench_unop!(mat4_inverse_na, try_inverse, reproductible_matrix4_na);
bench_unop!(mat3_inverse_na, try_inverse, reproductible_matrix3_na);
bench_unop!(mat2_inverse_na, try_inverse, reproductible_matrix2_na);

/*
 *
 * cgmath
 *
 */
bench_unop!(mat4_inverse_cgmath, invert, reproductible_matrix4_cgmath);
bench_unop!(mat3_inverse_cgmath, invert, reproductible_matrix3_cgmath);
bench_unop!(mat2_inverse_cgmath, invert, reproductible_matrix2_cgmath);

/*
 *
 * vek
 *
 */
bench_unop!(mat4_inverse_vek, inverted, reproductible_matrix4_vek);
//bench_unop!(mat3_inverse_vek, inverted, reproductible_matrix3_vek);
//bench_unop!(mat2_inverse_vek, inverted, reproductible_matrix2_vek);
