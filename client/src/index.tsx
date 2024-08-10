/* @refresh reload */
import { Component, ParentComponent } from "solid-js";
import { render } from "solid-js/web";
import { LoginPage, RegisterPage } from "./auth-form";
import { Route, Router } from "@solidjs/router";

const App: ParentComponent = (props) => (
  <main class="h-screen flex flex-col justify-center items-center bg-sky-800">
    {props.children}
  </main>
);

const PasswordsPage: Component = () => (
  <ul>
    <li>test</li>
  </ul>
);

render(
  () => (
    <Router root={App}>
      <Route path="/" component={LoginPage} />
      <Route path="/register" component={RegisterPage} />
      <Route path="/passwords" component={PasswordsPage} />
    </Router>
  ),
  document.getElementById("root")!,
);
