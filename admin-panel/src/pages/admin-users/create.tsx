import { Create, useForm } from "@refinedev/antd";
import { Checkbox, Form, Input } from "antd";

export const AdminUserCreate = () => {
  const { formProps, saveButtonProps } = useForm({});

  return (
    <Create saveButtonProps={saveButtonProps}>
      <Form {...formProps} layout="vertical">
        <Form.Item
          label="Email"
          name="email"
          rules={[
            {
              required: true,
            },
          ]}
        >
          <Input type="email" />
        </Form.Item>
        <Form.Item
          label="Active"
          name="active"
          valuePropName="checked"
        >
          <Checkbox />
        </Form.Item>
      </Form>
    </Create>
  );
};
