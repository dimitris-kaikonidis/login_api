/* @refresh reload */
import { Component, createResource } from "solid-js";
import { render } from "solid-js/web";
import { RegisterForm } from "./register-form";

const listUsers = async () => {
  const res = await fetch("http://localhost:3000/list_users");
  const users = await res.json();

  console.log(users);

  return users;
};

const App: Component = () => {
  const [_data] = createResource(listUsers);

  return (
    <>
      <RegisterForm />
    </>
  );
};

render(() => <App />, document.getElementById("root")!);
