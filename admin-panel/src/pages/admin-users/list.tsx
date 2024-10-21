import {
  BooleanField,
  DateField,
  DeleteButton,
  EditButton,
  List,
  ShowButton,
  useTable,
} from "@refinedev/antd";
import { type BaseRecord } from "@refinedev/core";
import { Space, Table } from "antd";

export const AdminUserList = () => {
  const { tableProps } = useTable({
    syncWithLocation: true,
  });

  return (
    <List>
      <Table {...tableProps} rowKey="id">
        <Table.Column dataIndex="id" title={"ID"} />
        <Table.Column dataIndex="email" title={"Email"} />
        <Table.Column dataIndex="active" title={"Active"} render={(value) => <BooleanField value={value} />} />
        <Table.Column dataIndex="created_at" title={"Created At"} render={(value) => <DateField value={value} format="YYYY-MM-DD HH:mm:ss" />} />
        <Table.Column dataIndex="updated_at" title={"Updated At"} render={(value) => <DateField value={value} format="YYYY-MM-DD HH:mm:ss" />} />
        <Table.Column
          title={"Actions"}
          dataIndex="actions"
          render={(_, record: BaseRecord) => (
            <Space>
              <EditButton hideText size="small" recordItemId={record.id} />
              <ShowButton hideText size="small" recordItemId={record.id} />
              <DeleteButton hideText size="small" recordItemId={record.id} />
            </Space>
          )}
        />
      </Table>
    </List>
  );
};
