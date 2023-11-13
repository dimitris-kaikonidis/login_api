/* @refresh reload */
import { Component } from "solid-js";
import { render } from "solid-js/web";
import { AuthForm } from "./auth-form";

const login = async () => {
  const res = await fetch("http://localhost:3000/login", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      email: "dim@g.c",
      password: "kai",
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
    <main class="h-screen flex justify-center items-center bg-sky-800">
      <AuthForm action="register" />
    </main>
  );
};

render(() => <App />, document.getElementById("root")!);
