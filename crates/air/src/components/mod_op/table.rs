use stwo_prover::{
    constraint_framework::TraceTable,
    core::{
        fields::{m31::BaseField, qm31::SecureField},
        trace::{
            table::{
                Column, ColumnConfig, TableCore, TableHeader, TableTrait, TableTraitWithStaticHeader,
            },
            TraceConfig,
        },
    },
};

use crate::components::{ModClaim, NodeInfo};

/// Table for the modulo operation.
#[derive(Clone, Debug)]
pub struct ModTable {
    pub header: TableHeader,
    pub name: String,
    pub log_size: u32,
    pub dividend: Column<SecureField>,
    pub divisor: Column<SecureField>,
    pub remainder: Column<SecureField>,
    pub quotient: Column<SecureField>,
}

impl ModTable {
    /// Creates a new modulo table from a claim.
    pub fn new(claim: &ModClaim, config: &TraceConfig) -> Self {
        let log_size = claim.log_size;
        let size = 1 << log_size;
        
        let header = TableHeader::new(&format!("mod_table_{}", claim.node_info.id));
        
        // Initialize columns
        let dividend = Column::new(
            ColumnConfig::new(&format!("dividend_{}", claim.node_info.id), size),
            SecureField::default(),
        );
        
        let divisor = Column::new(
            ColumnConfig::new(&format!("divisor_{}", claim.node_info.id), size),
            SecureField::default(),
        );
        
        let remainder = Column::new(
            ColumnConfig::new(&format!("remainder_{}", claim.node_info.id), size),
            SecureField::default(),
        );
        
        let quotient = Column::new(
            ColumnConfig::new(&format!("quotient_{}", claim.node_info.id), size),
            SecureField::default(),
        );
        
        Self {
            header,
            name: format!("mod_table_{}", claim.node_info.id),
            log_size,
            dividend,
            divisor,
            remainder,
            quotient,
        }
    }
    
    /// Fills the table with values for the modulo operation.
    pub fn fill(&mut self, values: &[Vec<SecureField>]) {
        assert_eq!(values.len(), 3); // Should have dividend, divisor, and remainder
        
        // Fill in dividend and divisor columns
        for (i, value) in values[0].iter().enumerate() {
            self.dividend.values[i] = *value;
        }
        
        for (i, value) in values[1].iter().enumerate() {
            self.divisor.values[i] = *value;
        }
        
        // Calculate remainder and quotient
        for i in 0..self.dividend.values.len() {
            let dividend = self.dividend.values[i].to_base();
            let divisor = self.divisor.values[i].to_base();
            
            // Handle division by zero - you might want a better approach here
            if divisor.is_zero() {
                self.remainder.values[i] = values[2][i]; // Use provided remainder
                self.quotient.values[i] = SecureField::zero();
                continue;
            }
            
            // Compute quotient and remainder
            let quotient_val = dividend.clone() / divisor.clone();
            let remainder_val = dividend - quotient_val.clone() * divisor;
            
            self.quotient.values[i] = SecureField::from(quotient_val);
            self.remainder.values[i] = SecureField::from(remainder_val);
        }
    }
}

impl TableCore for ModTable {
    fn name(&self) -> &str {
        &self.name
    }

    fn header(&self) -> &TableHeader {
        &self.header
    }

    fn column_count(&self) -> usize {
        4 // dividend, divisor, remainder, quotient
    }
}

impl TableTrait for ModTable {
    fn column_iter(&self) -> Box<dyn Iterator<Item = &dyn stwo_prover::core::trace::table::ColumnCore> + '_> {
        Box::new(
            [
                &self.dividend as &dyn stwo_prover::core::trace::table::ColumnCore,
                &self.divisor as &dyn stwo_prover::core::trace::table::ColumnCore,
                &self.remainder as &dyn stwo_prover::core::trace::table::ColumnCore,
                &self.quotient as &dyn stwo_prover::core::trace::table::ColumnCore,
            ]
            .into_iter(),
        )
    }
}

impl TableTraitWithStaticHeader for ModTable {
    fn column_configs() -> Vec<ColumnConfig> {
        vec![
            ColumnConfig::new("dividend", 0),
            ColumnConfig::new("divisor", 0),
            ColumnConfig::new("remainder", 0), 
            ColumnConfig::new("quotient", 0),
        ]
    }
}

impl TraceTable for ModTable {} 