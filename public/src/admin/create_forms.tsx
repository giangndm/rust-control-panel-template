import { Create, required, SimpleForm, TextInput } from "react-admin"

export const UserCreate = () => {
    return (<Create>
        <SimpleForm>
            <TextInput source="name" validate={[required()]} />
            <TextInput source="user_name" validate={[required()]} />
        </SimpleForm>
    </Create>)
}