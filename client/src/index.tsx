/* @refresh reload */
import { Component } from "solid-js";
import { render } from "solid-js/web";
import { LoginForm, RegisterForm } from "./auth-form";

const App: Component = () => {
  return (
    <main class="h-screen flex flex-col justify-center items-center bg-sky-800">
      <RegisterForm />
      <LoginForm />
    </main>
  );
};

render(() => <App />, document.getElementById("root")!);
