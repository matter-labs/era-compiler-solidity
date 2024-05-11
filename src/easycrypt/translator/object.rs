use anyhow::Error;

use crate::easycrypt::syntax::module::Module;
use crate::yul::parser::statement::object::Object;
use crate::Translator;

impl Translator {
    /// Transpile an arbitrary YUL object.
    pub fn transpile_object(&mut self, obj: &Object, is_root: bool) -> Result<Module, Error> {
        let module_name = &obj.identifier;

        self.location_tracker.enter_object(module_name);
        let mut result = Module::new(if is_root {
            Some(module_name.to_owned())
        } else {
            None
        });

        result.merge(&self.transpile_code(&obj.code)?);

        if let Some(inner_object) = &obj.inner_object {
            let translated_inner_object = self.transpile_object(inner_object.as_ref(), false)?;
            result.merge(&translated_inner_object)
        }

        self.location_tracker.leave();

        Ok(result)
    }
}
