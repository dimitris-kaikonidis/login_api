import { Component, createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api";

export const RegisterForm: Component = () => {
  const [email, setEmail] = createSignal<string>();
  const [password, setPassword] = createSignal<string>();

  const onSubmit = async (event: SubmitEvent) => {
    event.preventDefault();

    const [salt, verifier] = await invoke<[number[], number[]]>(
      "create_salt_and_verifier",
      {
        username: email(),
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
  const [password, setPassword] = createSignal<string>();

  const onSubmit = async (event: SubmitEvent) => {
    event.preventDefault();

    const ws = new WebSocket("ws://localhost:3000/login");
    const public_a = await invoke<number[]>("public_a");

    if (ws.readyState === 1) {
      ws.send(JSON.stringify({ email: email() }));
      ws.send(JSON.stringify(public_a));
    }

    ws.addEventListener("message", async (event) => {
      const data = JSON.parse(event.data);

      switch (data.type) {
        case "first_step_server":
          const clientProof = await invoke<number[]>("compute_verifier", {
            username: email(),
            password: password(),
            salt: data.salt,
            publicB: data.public_b,
          });

          ws.send(JSON.stringify(clientProof));

          break;

        case "second_step_server":
          console.log(data);

          const res = await invoke("verify_server_proof", {
            serverProof: data.server_proof,
          });

          console.log(res);

          break;
      }
    });

    ws.addEventListener("close", () => {
      console.log("disconnected");
    });
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
