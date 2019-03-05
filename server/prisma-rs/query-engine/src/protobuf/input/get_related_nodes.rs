use crate::{
    data_resolvers::{IntoSelectQuery, SelectQuery},
    protobuf::{prelude::*, InputValidation},
};
use prisma_common::PrismaResult;
use prisma_models::prelude::*;

impl IntoSelectQuery for GetRelatedNodesInput {
    fn into_select_query(self) -> PrismaResult<SelectQuery> {
        let project_template: ProjectTemplate = serde_json::from_reader(self.project_json.as_slice())?;

        let project: ProjectRef = project_template.into();
        let model = project.schema().find_model(&self.model_name)?;
        unimplemented!()
    }
}

impl InputValidation for GetRelatedNodesInput {
    fn validate(&self) -> PrismaResult<()> {
        Self::validate_args(&self.query_arguments)
    }
}
