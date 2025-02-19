use validator::ValidationErrors;

pub fn collect_error(param: ValidationErrors) -> String {
    let mut message = String::new();

    for (field, error_message) in param.field_errors() {
        let mut error_messages = vec![];

        for er in error_message {
            error_messages.push(er.to_string());
        }

        let error = error_messages.join("|");

        message =
            format!("{message}, Field {field}: Message: {error}").to_string();
    }

    message
}
