use li_shacl::ast::ASTSchema;
use li_shacl::validator::report::ValidationReport;
use rudof_rdf::rdf_core::Rdf;

pub(crate) struct TestInstance<RDF: Rdf> {
    pub data: RDF,
    pub shapes: ASTSchema,
    pub report: ValidationReport,
}

impl<RDF: Rdf> TestInstance<RDF> {
    pub fn new(data: RDF, shapes: ASTSchema, report: ValidationReport) -> Self {
        Self { data, shapes, report }
    }
}
