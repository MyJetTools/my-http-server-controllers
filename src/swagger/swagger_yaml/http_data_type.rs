use crate::controllers::documentation::data_types::{ArrayElement, HttpDataType, HttpSimpleType};

use super::yaml_writer::YamlWriter;

pub fn build(yaml_writer: &mut YamlWriter, root_name: &str, data_type: &HttpDataType) {
    match &data_type {
        HttpDataType::SimpleType(param_type) => {
            yaml_writer.write_empty(root_name);
            write_simple_type(yaml_writer, param_type);
        }
        HttpDataType::ObjectId { struct_id } => {
            yaml_writer.write_empty(root_name);
            write_object_type(yaml_writer, struct_id);
        }
        HttpDataType::Object(object_type) => {
            yaml_writer.write_empty(root_name);
            write_object_type(yaml_writer, &object_type.struct_id);
        }
        HttpDataType::Enum(enum_type) => {
            yaml_writer.write_empty(root_name);
            write_object_type(yaml_writer, &enum_type.struct_id);
        }
        HttpDataType::None => {}
        HttpDataType::ArrayOf(array_element) => {
            yaml_writer.write_empty(root_name);
            yaml_writer.increase_level();
            yaml_writer.write("type", "array");

            yaml_writer.write_empty("items");

            let items = match array_element {
                ArrayElement::SimpleType(param_type) => write_simple_type(yaml_writer, param_type),
                ArrayElement::ObjectId { struct_id } => write_object_type(yaml_writer, struct_id),
                ArrayElement::Object(object_type) => {
                    write_object_type(yaml_writer, &object_type.struct_id)
                }
            };

            yaml_writer.decrease_level();
        }
    }
}

fn write_simple_type(yaml_writer: &mut YamlWriter, param_type: &HttpSimpleType) {
    yaml_writer.increase_level();
    yaml_writer.write("type", param_type.as_swagger_type());
    yaml_writer.decrease_level();
}

fn write_object_type(yaml_writer: &mut YamlWriter, struct_id: &str) {
    yaml_writer.increase_level();
    yaml_writer.write("$ref", format!("#/definitions/{}", struct_id).as_str());
    yaml_writer.decrease_level();
}