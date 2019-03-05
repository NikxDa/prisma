use std::collections::BTreeSet;
use crate::model::{ModelRef, RelationField, ScalarField};
use std::sync::Arc;
use once_cell::unsync::OnceCell;

trait IntoSelectedFields {
    fn into_selected_fields(self, model: ModelRef, from_field: Option<Arc<RelationField>>) -> SelectedFields;
}

#[derive(Default)]
pub struct SelectedFields {
    fields: BTreeSet<Arc<SelectedField>>,
    scalar: BTreeSet<Weak<SelectedScalarField>>,
    relation: BTreeSet<Weak<SelectedRelationField>>,
    columns: OnceCell<Vec<Column>>,
}

impl SelectedFields {
    const RELATED_MODEL_ALIAS: &'static str = "__RelatedModel__";
    const PARENT_MODEL_ALIAS: &'static str = "__ParentModel__";

    pub fn new(fields: BTreeSet<SelectedField>, from_field: Option<Arc<RelationField>>) -> SelectedFields {
        let fields = fields.into_iter().map(Arc::new).collect();

        let scalar = fields.iter().fold(BTreeSet::new(), |mut acc, field| {
            if let SelectedField::Scalar(ref sf) = field {
                acc.insert(Arc::downgrade(sf))
            };

            acc
        });

        let relation = fields.iter().fold(BTreeSet::new(), |mut acc, field| {
            if let SelectedField::Relation(ref rf) = field {
                acc.insert(Arc::downgrade(rf))
            };

            acc
        });

        let columns = OnceCell::new();

        SelectedFields { fields, scalar, relation, from_field, columns, }
    }

    pub fn columns(&self) -> &[&Column] {
        self.columns.get_or_init(|| {
            let mut result = self.scalar().iter().map(|field| field.as_column()).collect();

            if let Some(from_field) = self.from_field {
                let relation = from_field.relation();

                result.push(
                    relation.column_for_relation_side(
                        from_field.relation_side.opposite()
                    ).alias(RELATED_MODEL_ALIAS)
                );

                result.push(
                    relation.column_for_relation_side(
                        from_field.relation_side
                    ).alias(PARENT_MODEL_ALIAS)
                );
            };

            result
        }).as_slice()
    }

    fn scalar(&self) -> Vec<Arc<SelectedField>> {
        self.scalar.iter().map(|field| field.upgrade().unwrap()).collect()
    }

    fn relation(&self) -> Vec<Arc<SelectedField>> {
        self.scalar.iter().map(|field| field.upgrade().unwrap()).collect()
    }
}

pub enum SelectedField {
    Scalar<Arc<ScalarField>>,
    Relation<SelectedRelationField>
}

pub struct SelectedScalarField {
    pub field: Arc<ScalarField>
}

pub struct SelectedRelationField {
    pub field: Arc<RelationField>,
    pub selected_fields: SelectedFields
}
