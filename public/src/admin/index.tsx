import { Admin, Resource, ListGuesser, EditGuesser } from "react-admin";
import jsonServerProvider from "ra-data-json-server";
import { UserCreate } from "./create_forms";

const dataProvider = jsonServerProvider("/api/admin-panel");

const App = () => (
    <Admin dataProvider={dataProvider}>
        <Resource name="users" list={ListGuesser} create={UserCreate} edit={EditGuesser} />
    </Admin>
);

export default App;
