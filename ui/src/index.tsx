/* @refresh reload */
import { Component } from "solid-js";
import { render } from "solid-js/web";
import { RegisterForm } from "./register-form";

const login = async () => {
  const res = await fetch("http://localhost:3000/login", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      email: "EMAIL",
      password: "password",
    }),
  });

  return res;
};

const App: Component = () => {
  (async () => {
    const res = await login();
    console.log(res);
  })();

  return (
    <>
      <RegisterForm />
    </>
  );
};

render(() => <App />, document.getElementById("root")!);
