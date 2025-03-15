use num_traits::{One, Zero};
use numerair::eval::EvalFixedPoint;
use stwo_prover::{
    constraint_framework::{EvalAtRow, FrameworkComponent, FrameworkEval, RelationEntry},
    core::fields::{m31::BaseField, qm31::SecureField},
};

use crate::components::{ModClaim, NodeElements};

/// Component for modulo operations, using `SimdBackend` with fallback to `CpuBackend` for small traces.
pub type ModComponent = FrameworkComponent<ModEval>;

/// Defines the AIR for the modulo component.
pub struct ModEval {
    log_size: u32,
    lookup_elements: NodeElements,
    dividend_multiplicity: SecureField,
    divisor_multiplicity: SecureField,
    remainder_multiplicity: SecureField,
}

impl ModEval {
    /// Creates a new `ModEval` instance from a claim and lookup elements.
    pub fn new(claim: &ModClaim, lookup_elements: NodeElements) -> Self {
        let dividend_multiplicity = if claim.node_info.inputs[0].is_initializer {
            SecureField::zero()
        } else {
            -SecureField::one()
        };

        let divisor_multiplicity = if claim.node_info.inputs[1].is_initializer {
            SecureField::zero()
        } else {
            -SecureField::one()
        };
        
        let remainder_multiplicity = if claim.node_info.output.is_final_output {
            SecureField::zero()
        } else {
            SecureField::one() * BaseField::from_u32_unchecked(claim.node_info.num_consumers as u32)
        };

        Self {
            log_size: claim.log_size,
            lookup_elements,
            dividend_multiplicity,
            divisor_multiplicity,
            remainder_multiplicity,
        }
    }
}

impl FrameworkEval for ModEval {
    /// Returns the logarithmic size of the main trace.
    fn log_size(&self) -> u32 {
        self.log_size
    }

    /// The degree of the constraints is bounded by the size of the trace.
    ///
    /// Returns the ilog2 (upper) bound of the constraint degree for the component.
    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size + 1
    }

    /// Evaluates the AIR constraints for the modulo operation.
    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let dividend = eval.next_trace_mask();
        let divisor = eval.next_trace_mask();
        let remainder = eval.next_trace_mask();
        let quotient = eval.next_trace_mask(); // Need quotient for the mod calculation
        
        // Constraint: dividend = divisor * quotient + remainder
        // And: 0 <= remainder < divisor
        eval.eval_fixed_mod(dividend.clone(), divisor.clone(), remainder.clone(), quotient.clone());

        // Add the lookup relations for the inputs and outputs
        eval.add_to_relation(RelationEntry::new(
            &self.lookup_elements,
            self.dividend_multiplicity.into(),
            &[dividend],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.lookup_elements,
            self.divisor_multiplicity.into(),
            &[divisor],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.lookup_elements,
            self.remainder_multiplicity.into(),
            &[remainder],
        ));

        eval.finalize_logup();

        eval
    }
} 