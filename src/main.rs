use lambdaworks_math::cyclic_group::IsGroup;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;
use lambdaworks_math::elliptic_curve::short_weierstrass::point::ShortWeierstrassProjectivePoint;
use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;
use lambdaworks_math::traits::{AsBytes, ByteConversion};
use lambdaworks_math::unsigned_integer::element::U256;
use lambdaworks_math::unsigned_integer::traits::IsUnsignedInteger;


/**
    This function computes the public key from a given private key operating on the BLS12_381 curve.
    Lemma: The public key is computed as the product of the private key and the generator point.

    pub_k = priv_k * G mod p
*/
pub fn public_key<T: IsUnsignedInteger>(private_key: T) -> ShortWeierstrassProjectivePoint<BLS12381Curve> {
    let generator = BLS12381Curve::generator();
    let public_key = generator.operate_with_self(private_key);

    public_key
}



fn main() {
    let private_key = U256::from_hex_unchecked("6C616D6264617370");
    let public_key = public_key(private_key);
    let public_key_bytes = public_key.as_bytes();
    let public_u256 = U256::from_bytes_be(&public_key_bytes).expect("Failed to convert public key to U256");

    println!("public key: {:?}", public_u256.to_hex());
}

// public key: "EFC2D10AD531CEBF2B8C7B4325BC93ED91E6477D260304C1F9ECC7BA0E6F5711"