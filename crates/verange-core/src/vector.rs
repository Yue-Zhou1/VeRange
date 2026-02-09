use crate::curve::Scalar;
use crate::CoreError;
use ark_ff::Zero;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScalarVector {
    values: Vec<Scalar>,
}

impl ScalarVector {
    pub fn new(values: Vec<Scalar>) -> Result<Self, CoreError> {
        Ok(Self { values })
    }

    pub fn values(&self) -> &[Scalar] {
        &self.values
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn add(&self, other: &Self) -> Result<Self, CoreError> {
        self.ensure_same_len(other)?;
        Ok(Self {
            values: self
                .values
                .iter()
                .zip(other.values.iter())
                .map(|(a, b)| *a + *b)
                .collect(),
        })
    }

    pub fn sub(&self, other: &Self) -> Result<Self, CoreError> {
        self.ensure_same_len(other)?;
        Ok(Self {
            values: self
                .values
                .iter()
                .zip(other.values.iter())
                .map(|(a, b)| *a - *b)
                .collect(),
        })
    }

    pub fn mul_elementwise(&self, other: &Self) -> Result<Self, CoreError> {
        self.ensure_same_len(other)?;
        Ok(Self {
            values: self
                .values
                .iter()
                .zip(other.values.iter())
                .map(|(a, b)| *a * *b)
                .collect(),
        })
    }

    pub fn inner_product(&self, other: &Self) -> Result<Scalar, CoreError> {
        self.ensure_same_len(other)?;
        Ok(self
            .values
            .iter()
            .zip(other.values.iter())
            .fold(Scalar::zero(), |acc, (a, b)| acc + (*a * *b)))
    }

    pub fn mul_constant(&self, constant: Scalar) -> Self {
        Self {
            values: self.values.iter().map(|v| *v * constant).collect(),
        }
    }

    pub fn power(base: Scalar, size: usize) -> Self {
        if size == 0 {
            return Self { values: vec![] };
        }

        let mut values = Vec::with_capacity(size);
        let mut current = Scalar::from(1u64);
        values.push(current);

        for _ in 1..size {
            current *= base;
            values.push(current);
        }

        Self { values }
    }

    fn ensure_same_len(&self, other: &Self) -> Result<(), CoreError> {
        if self.len() != other.len() {
            return Err(CoreError::VectorLengthMismatch {
                left: self.len(),
                right: other.len(),
            });
        }
        Ok(())
    }
}
