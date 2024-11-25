import React from "react";
import { createRoot } from "react-dom/client";
import DynamicAuth0 from "./Auth0";

import App from "./App";

const container = document.getElementById("root") as HTMLElement;
const root = createRoot(container);

root.render(
  <React.StrictMode>
    <DynamicAuth0>
      <App />
    </DynamicAuth0>
  </React.StrictMode >
);
