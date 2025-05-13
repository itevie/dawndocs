import { StrictMode } from "react";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import ReactDOM from "react-dom/client";

$imports;

const root = ReactDOM.createRoot(document.getElementById("root"));
const routes = createBrowserRouter([$routes]);

import "./dawn-ui/index.js";
import Row from "./dawn-ui/components/Row.js";
import Column from "./dawn-ui/components/Column.js";
import Sidebar from "./dawn-ui/components/Sidebar.js";
import Navbar from "./dawn-ui/components/Navbar.js";
import FullPage from "./dawn-ui/components/FullPage.js";
import Link from "./dawn-ui/components/Link.js";

export const config = $config;

function App() {
  return (
    <FullPage style={{ overflow: "hidden" }}>
      <Navbar title={config.title} noPage>
        <Row></Row>
      </Navbar>
      <Row className="full-page" util={["no-gap"]}>
        <Sidebar>
          <Column util={["overflow-y-scroll"]}>$sidebar_links</Column>
        </Sidebar>
        <Column
          util={["overflow-y-scroll"]}
          style={{ gap: "0px", margin: "10px" }}
        >
          <RouterProvider router={routes} />
        </Column>
      </Row>
    </FullPage>
  );
}

root.render(
  <StrictMode>
    <App />
  </StrictMode>,
);
