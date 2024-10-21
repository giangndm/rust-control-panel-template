import { Edit, useForm } from "@refinedev/antd";
import { Checkbox, Form, Input } from "antd";

export const AdminUserEdit = () => {
  const { formProps, saveButtonProps, formLoading, query } = useForm({});

  return (
    <Edit saveButtonProps={saveButtonProps} isLoading={formLoading}>
      <Form {...formProps} layout="vertical">
        <Form.Item
          label="Email"
          name="email"
        >
          <Input disabled />
        </Form.Item>
        <Form.Item
          label="Active"
          name="active"
          valuePropName="checked"
          rules={[
            {
              required: true,
            },
          ]}
        >
          <Checkbox />
        </Form.Item>
      </Form>
    </Edit>
  );
};
