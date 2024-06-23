import { Component, createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api";

export const RegisterForm: Component = () => {
  const [email, setEmail] = createSignal<string>();
  const [password, setPassword] = createSignal<string>();

  const onSubmit = async (event: SubmitEvent) => {
    event.preventDefault();

    const [salt, verifier] = await invoke<[string, string]>(
      "create_salt_and_verifier",
      {
        email: email(),
        password: password(),
      },
    );

    await fetch(`http://localhost:3000/register`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        email: email(),
        salt,
        verifier,
      }),
    });
  };

  return (
    <form
      class="flex flex-col gap-2 p-10 outline-white outline"
      onSubmit={onSubmit}
    >
      <h3 class="text-white">Register</h3>
      <input
        class="rounded-md p-2"
        name="email"
        type="email"
        placeholder="Email"
        onInput={(e) => setEmail(e.currentTarget.value)}
      />
      <input
        class="rounded-md p-2"
        name="password"
        type="password"
        placeholder="Password"
        onInput={(e) => setPassword(e.currentTarget.value)}
      />
      <button
        class="border outline-white rounded-md text-white bg-sky-900 p-2"
        type="submit"
      >
        Submit
      </button>
    </form>
  );
};

export const LoginForm: Component = () => {
  const [email, setEmail] = createSignal<string>();
  const [_password, setPassword] = createSignal<string>();

  const onSubmit = async (event: SubmitEvent) => {
    event.preventDefault();

    const publicA = await invoke<string>("compute_client_value_a");

    const response = await fetch(`http://localhost:3000/login`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        email: email(),
        public_value_a: publicA,
      }),
    });

    const { public_b: publicB }: { public_b: string } = await response.json();

    // const premasterSecret = ( publicB  -  ()) ;
  };

  return (
    <form
      class="flex flex-col gap-2 p-10 outline-white outline"
      onSubmit={onSubmit}
    >
      <h3 class="text-white">Login</h3>
      <input
        class="rounded-md p-2"
        name="email"
        type="email"
        placeholder="Email"
        onInput={(e) => setEmail(e.currentTarget.value)}
      />
      <input
        class="rounded-md p-2"
        name="password"
        type="password"
        placeholder="Password"
        onInput={(e) => setPassword(e.currentTarget.value)}
      />
      <button
        class="border outline-white rounded-md text-white bg-sky-900 p-2"
        type="submit"
      >
        Submit
      </button>
    </form>
  );
};
